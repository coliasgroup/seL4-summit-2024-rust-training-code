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
