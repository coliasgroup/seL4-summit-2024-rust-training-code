//
// Copyright 2024, Colias Group, LLC
//
// SPDX-License-Identifier: BSD-2-Clause
//

#![no_std]
#![no_main]
#![feature(never_type)]

extern crate alloc;

use alloc::boxed::Box;
use core::cell::UnsafeCell;
use core::mem;
use core::ops::Range;
use core::panic::UnwindSafe;
use core::ptr;

use sel4_elf_header::{ElfHeader, PT_TLS};
use sel4_initialize_tls::{TlsImage, TlsReservationLayout, UncheckedTlsImage};
use sel4_root_task::{
    abort, panicking::catch_unwind, root_task, set_global_allocator_mutex_notification, Never,
};
use sel4_stack::Stack;

const GRANULE_SIZE: usize = sel4::FrameObjectType::GRANULE.bytes(); // 4096

static SECONDARY_THREAD_STACK: Stack<8192> = Stack::new();

static SECONDARY_THREAD_IPC_BUFFER_FRAME: IpcBufferFrame = IpcBufferFrame::new();

#[root_task(heap_size = 64 * 1024)]
fn main(bootinfo: &sel4::BootInfoPtr) -> sel4::Result<Never> {
    let mut object_allocator = ObjectAllocator::new(bootinfo);

    // Provide a notification capability that the global heap allocator can use for its mutex. If we
    // didn't provide this, then the global heap allocator would panic in the case of contention.
    set_global_allocator_mutex_notification(
        object_allocator.allocate_fixed_sized::<sel4::cap_type::Notification>(),
    );

    let inter_thread_ep = object_allocator.allocate_fixed_sized::<sel4::cap_type::Endpoint>();

    let secondary_thread_tcb = object_allocator.allocate_fixed_sized::<sel4::cap_type::Tcb>();

    secondary_thread_tcb.tcb_configure(
        sel4::init_thread::slot::NULL.cptr(),
        sel4::init_thread::slot::CNODE.cap(),
        sel4::CNodeCapData::new(0, 0),
        sel4::init_thread::slot::VSPACE.cap(),
        SECONDARY_THREAD_IPC_BUFFER_FRAME.ptr() as sel4::Word,
        SECONDARY_THREAD_IPC_BUFFER_FRAME.cap(bootinfo),
    )?;

    // This is the function that will run in the secondary thread.
    let secondary_thread_fn = SecondaryThreadFn::new(move || {
        unsafe { sel4::set_ipc_buffer(SECONDARY_THREAD_IPC_BUFFER_FRAME.ptr().as_mut().unwrap()) }
        secondary_thread_main(inter_thread_ep);
        secondary_thread_tcb.tcb_suspend().unwrap();
        unreachable!()
    });

    // Initialize the secondary thread context and start it.
    secondary_thread_tcb
        .tcb_write_all_registers(true, &mut create_user_context(secondary_thread_fn))?;

    interact_with_secondary_thread(inter_thread_ep);

    sel4::debug_println!("TEST_PASS");

    sel4::init_thread::suspend_self()
}

fn secondary_thread_main(inter_thread_ep: sel4::cap::Endpoint) {
    sel4::debug_println!("In secondary thread");

    inter_thread_ep.send(sel4::MessageInfoBuilder::default().label(123).build());
}

fn interact_with_secondary_thread(inter_thread_ep: sel4::cap::Endpoint) {
    sel4::debug_println!("In primary thread");

    let (msg_info, _badge) = inter_thread_ep.recv(());

    assert_eq!(msg_info.label(), 123);
}

// Simple object allocator that just uses the largest kernel untyped to allocate objects into the
// root task's initial CSpace's empty slots.
struct ObjectAllocator {
    empty_slots: Range<usize>,
    ut: sel4::cap::Untyped,
}

impl ObjectAllocator {
    fn new(bootinfo: &sel4::BootInfo) -> Self {
        Self {
            empty_slots: bootinfo.empty().range(),
            ut: find_largest_kernel_untyped(bootinfo),
        }
    }

    fn allocate_fixed_sized<T: sel4::CapTypeForObjectOfFixedSize>(&mut self) -> sel4::Cap<T> {
        let slot_index = self.empty_slots.next().unwrap();
        self.ut
            .untyped_retype(
                &T::object_blueprint(),
                &sel4::init_thread::slot::CNODE
                    .cap()
                    .absolute_cptr_for_self(),
                slot_index,
                1,
            )
            .unwrap();
        sel4::init_thread::Slot::from_index(slot_index).cap()
    }
}

fn find_largest_kernel_untyped(bootinfo: &sel4::BootInfo) -> sel4::cap::Untyped {
    let (ut_ix, _desc) = bootinfo
        .untyped_list()
        .iter()
        .enumerate()
        .filter(|(_i, desc)| !desc.is_device())
        .max_by_key(|(_i, desc)| desc.size_bits())
        .unwrap();

    bootinfo.untyped().index(ut_ix).cap()
}

// Set up a user context for a secondary thread. This context's PC will point to `f`.
fn create_user_context(f: SecondaryThreadFn) -> sel4::UserContext {
    let mut ctx = sel4::UserContext::default();

    *ctx.sp_mut() = (SECONDARY_THREAD_STACK.bottom().ptr() as usize)
        .try_into()
        .unwrap();
    *ctx.pc_mut() = (secondary_thread_entrypoint as usize).try_into().unwrap();
    *ctx.c_param_mut(0) = f.into_arg();

    let tls_reservation = TlsReservation::new(&get_tls_image());
    ctx.inner_mut().tpidr_el0 = tls_reservation.thread_pointer() as sel4::Word;
    mem::forget(tls_reservation);

    ctx
}

// Concrete entrypoint for secondary threads. `arg` will be interpreted by
// `SecondaryThreadFn::from_arg`.
unsafe extern "C" fn secondary_thread_entrypoint(arg: sel4::Word) -> ! {
    let f = SecondaryThreadFn::from_arg(arg);
    let _ = catch_unwind(|| f.run());
    abort!("secondary thread panicked")
}

// Type of a closure that will be sent to `secondary_thread_entrypoint` and ultimately run in a
// secondary thread.
struct SecondaryThreadFn(Box<dyn FnOnce() -> ! + UnwindSafe + Send + 'static>);

impl SecondaryThreadFn {
    fn new(f: impl FnOnce() -> ! + UnwindSafe + Send + 'static) -> Self {
        Self(Box::new(f))
    }

    fn run(self) -> ! {
        (self.0)()
    }

    fn into_arg(self) -> sel4::Word {
        Box::into_raw(Box::new(self)) as sel4::Word
    }

    unsafe fn from_arg(arg: sel4::Word) -> Self {
        *Box::from_raw(arg as *mut Self)
    }
}

// A region of memory to be used as thread-local storage for a secondary thread. We use the raw
// `alloc::alloc` API to have run-time control over layout.
struct TlsReservation {
    start: *mut u8,
    layout: TlsReservationLayout,
}

impl TlsReservation {
    fn new(tls_image: &TlsImage) -> Self {
        let layout = tls_image.reservation_layout();
        let start = unsafe { ::alloc::alloc::alloc(layout.footprint()) };
        unsafe {
            tls_image.initialize_tls_reservation(start);
        };
        Self { start, layout }
    }

    fn thread_pointer(&self) -> usize {
        (self.start as usize) + self.layout.thread_pointer_offset()
    }
}

impl Drop for TlsReservation {
    fn drop(&mut self) {
        unsafe {
            ::alloc::alloc::dealloc(self.start, self.layout.footprint());
        }
    }
}

// Find the TLS image in the ELF PHDRs. The linker provides `__ehdr_start`, which points at the ELF
// file header.
fn get_tls_image() -> TlsImage {
    extern "C" {
        static __ehdr_start: ElfHeader;
    }
    let phdrs = unsafe {
        assert!(__ehdr_start.check_magic());
        __ehdr_start.locate_phdrs()
    };
    let phdr = phdrs.iter().find(|phdr| phdr.p_type == PT_TLS).unwrap();
    let unchecked = UncheckedTlsImage {
        vaddr: phdr.p_vaddr,
        filesz: phdr.p_filesz,
        memsz: phdr.p_memsz,
        align: phdr.p_align,
    };
    unchecked.check().unwrap()
}

// An aligned granule to be used as the IPC buffer for a secondary thread.
#[repr(C, align(4096))]
struct IpcBufferFrame(UnsafeCell<[u8; GRANULE_SIZE]>);

unsafe impl Sync for IpcBufferFrame {}

impl IpcBufferFrame {
    const fn new() -> Self {
        Self(UnsafeCell::new([0; GRANULE_SIZE]))
    }

    const fn ptr(&self) -> *mut sel4::IpcBuffer {
        self.0.get().cast()
    }

    fn cap(&self, bootinfo: &sel4::BootInfo) -> sel4::cap::Granule {
        get_user_image_frame_slot(bootinfo, self.ptr() as usize).cap()
    }
}

// Find the slot in the root task initial CSpace containing the page capability containing the given
// address.
fn get_user_image_frame_slot(
    bootinfo: &sel4::BootInfo,
    addr: usize,
) -> sel4::init_thread::Slot<sel4::cap_type::Granule> {
    extern "C" {
        static __executable_start: usize;
    }
    let user_image_addr = ptr::addr_of!(__executable_start) as usize;
    bootinfo
        .user_image_frames()
        .index(addr / GRANULE_SIZE - user_image_addr / GRANULE_SIZE)
}
