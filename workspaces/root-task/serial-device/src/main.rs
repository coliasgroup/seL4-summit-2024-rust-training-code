//
// Copyright 2024, Colias Group, LLC
//
// SPDX-License-Identifier: BSD-2-Clause
//

#![no_std]
#![no_main]

use sel4_root_task::root_task;

mod device;

use device::Device;

const SERIAL_DEVICE_MMIO_PADDR: usize = 0x0900_0000;

const SERIAL_DEVICE_IRQ: usize = 33;

#[root_task]
fn main(bootinfo: &sel4::BootInfoPtr) -> ! {
    let mut empty_slots = bootinfo
        .empty()
        .range()
        .map(sel4::init_thread::Slot::<sel4::cap_type::Unspecified>::from_index);

    let largest_kernel_ut = find_largest_kernel_untyped(bootinfo);

    let (device_ut_ix, device_ut_desc) = bootinfo
        .untyped_list()
        .iter()
        .enumerate()
        .find(|(_i, desc)| {
            (desc.paddr()..(desc.paddr() + (1 << desc.size_bits())))
                .contains(&SERIAL_DEVICE_MMIO_PADDR)
        })
        .unwrap();

    assert!(device_ut_desc.is_device());

    let device_ut_cap = bootinfo.untyped().index(device_ut_ix).cap();

    trim_untyped(
        device_ut_cap,
        device_ut_desc.paddr(),
        SERIAL_DEVICE_MMIO_PADDR,
        empty_slots.next().unwrap(),
        empty_slots.next().unwrap(),
    );

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

fn trim_untyped(
    ut: sel4::cap::Untyped,
    ut_paddr: usize,
    target_paddr: usize,
    free_slot_a: sel4::init_thread::Slot,
    free_slot_b: sel4::init_thread::Slot,
) {
    let rel_a = sel4::init_thread::slot::CNODE
        .cap()
        .absolute_cptr(free_slot_a.cptr());
    let rel_b = sel4::init_thread::slot::CNODE
        .cap()
        .absolute_cptr(free_slot_b.cptr());
    let mut cur_paddr = ut_paddr;
    while cur_paddr != target_paddr {
        let size_bits = (target_paddr - cur_paddr).ilog2().try_into().unwrap();
        ut.untyped_retype(
            &sel4::ObjectBlueprint::Untyped { size_bits },
            &sel4::init_thread::slot::CNODE
                .cap()
                .absolute_cptr_for_self(),
            free_slot_b.index(),
            1,
        )
        .unwrap();
        rel_a.delete().unwrap();
        rel_a.move_(&rel_b).unwrap();
        cur_paddr += 1 << size_bits;
    }
}
