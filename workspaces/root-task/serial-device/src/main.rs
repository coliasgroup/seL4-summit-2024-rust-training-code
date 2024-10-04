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
    sel4::debug_println!("TEST_PASS");

    sel4::init_thread::suspend_self()
}
