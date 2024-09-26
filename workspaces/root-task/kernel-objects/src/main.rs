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

    sel4::debug_println!("TEST_PASS");

    sel4::init_thread::suspend_self()
}
