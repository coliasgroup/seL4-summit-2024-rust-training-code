//
// Copyright 2024, Colias Group, LLC
//
// SPDX-License-Identifier: BSD-2-Clause
//

#![no_std]
#![no_main]

use core::ptr;

use sel4::CapTypeForObjectOfFixedSize;
use sel4_root_task::root_task;

const GRANULE_SIZE: usize = sel4::FrameObjectType::GRANULE.bytes(); // 4096

#[repr(C, align(4096))]
struct PagePlaceHolder(#[allow(dead_code)] [u8; GRANULE_SIZE]);

static mut PAGE_A: PagePlaceHolder = PagePlaceHolder([0xee; GRANULE_SIZE]);

#[root_task]
fn main(bootinfo: &sel4::BootInfoPtr) -> ! {
    let largest_kernel_ut = find_largest_kernel_untyped(bootinfo);

    let cnode = sel4::init_thread::slot::CNODE.cap();

    let mut empty_slots = bootinfo
        .empty()
        .range()
        .map(sel4::init_thread::Slot::from_index);

    let new_frame_slot = empty_slots.next().unwrap();

    largest_kernel_ut
        .untyped_retype(
            &sel4::cap_type::Granule::object_blueprint(),
            &cnode.relative_self(),
            new_frame_slot.index(),
            1,
        )
        .unwrap();

    let new_frame = new_frame_slot.downcast::<sel4::cap_type::Granule>().cap();

    let page_a_addr = ptr::addr_of!(PAGE_A).cast::<u8>();
    sel4::debug_println!("PAGE_A: {page_a_addr:#x?}");
    assert_eq!(page_a_addr as usize % GRANULE_SIZE, 0);
    assert_eq!(unsafe { page_a_addr.read() }, 0xee);

    let page_a_slot = get_user_image_frame_slot(bootinfo, page_a_addr as usize);
    page_a_slot.cap().frame_unmap().unwrap();

    unsafe { page_a_addr.read() };

    sel4::debug_println!("TEST_PASS");

    sel4::init_thread::suspend_self()
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
