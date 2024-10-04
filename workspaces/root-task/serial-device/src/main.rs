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
