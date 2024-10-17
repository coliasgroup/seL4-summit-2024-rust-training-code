//
// Copyright 2024, Colias Group, LLC
//
// SPDX-License-Identifier: BSD-2-Clause
//

#![no_std]
#![no_main]

use core::ptr;

use microkit_shared_memory_common::{RegionB, REGION_A_SIZE};

use sel4_microkit::{debug_println, protection_domain, var, Channel, Handler, Infallible};

const SERVER: Channel = Channel::new(13);

#[protection_domain]
fn init() -> impl Handler {
    debug_println!("client: initializing");

    let region_a = *var!(region_a_vaddr: usize = 0) as *mut [u8; REGION_A_SIZE];
    let region_b = *var!(region_b_vaddr: usize = 0) as *mut RegionB;

    debug_println!("client: region_a = {region_a:#x?}");
    debug_println!("client: region_b = {region_b:#x?}");

    unsafe {
        (region_a as *mut u8).offset(13).write(37);
    }

    unsafe {
        (ptr::addr_of!((*region_b).foo) as *mut u8)
            .offset(1)
            .write(23);
    }

    HandlerImpl { region_a, region_b }
}

struct HandlerImpl {
    region_a: *mut [u8; REGION_A_SIZE],
    region_b: *mut RegionB,
}

impl Handler for HandlerImpl {
    type Error = Infallible;
}
