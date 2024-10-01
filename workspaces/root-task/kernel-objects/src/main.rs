//
// Copyright 2024, Colias Group, LLC
//
// SPDX-License-Identifier: BSD-2-Clause
//

#![no_std]
#![no_main]

use sel4_root_task::root_task;

#[root_task]
fn main(bootinfo: &sel4::BootInfoPtr) -> ! {
    sel4::debug_println!("untyped:");
    for ut_desc in bootinfo.untyped_list() {
        sel4::debug_println!(
            "    paddr: {:#016x?}, size bits: {:02?}, is device: {:?}",
            ut_desc.paddr(),
            ut_desc.size_bits(),
            ut_desc.is_device()
        );
    }

    let largest_kernel_ut = find_largest_kernel_untyped(bootinfo);
    sel4::debug_println!("largest kernel untyped: {largest_kernel_ut:?}");

    let cnode = sel4::init_thread::slot::CNODE.cap();

    let mut empty_slots = bootinfo
        .empty()
        .range()
        .map(sel4::init_thread::Slot::from_index);
    let notification_slot = empty_slots.next().unwrap();

    sel4::debug_println!("allocating notification");
    largest_kernel_ut
        .untyped_retype(
            &sel4::ObjectBlueprint::Notification,
            &cnode.relative_self(),
            notification_slot.index(),
            1,
        )
        .unwrap();

    let notification = notification_slot
        .downcast::<sel4::cap_type::Notification>()
        .cap();

    sel4::debug_println!("signaling notification");
    notification.signal();

    sel4::debug_println!("waiting on notification");
    notification.wait();

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
