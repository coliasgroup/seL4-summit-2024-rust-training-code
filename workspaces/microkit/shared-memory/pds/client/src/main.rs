//
// Copyright 2024, Colias Group, LLC
//
// SPDX-License-Identifier: BSD-2-Clause
//

#![no_std]
#![no_main]

use sel4_microkit::{debug_println, protection_domain, var, Channel, Handler, Infallible};

const SERVER: Channel = Channel::new(13);

#[protection_domain]
fn init() -> impl Handler {
    debug_println!("client: initializing");

    let region_a = *var!(region_a_vaddr: usize = 0);
    let region_b = *var!(region_b_vaddr: usize = 0);

    debug_println!("client: region_a = {region_a:#x?}");
    debug_println!("client: region_b = {region_b:#x?}");

    HandlerImpl { region_a, region_b }
}

struct HandlerImpl {
    region_a: usize,
    region_b: usize,
}

impl Handler for HandlerImpl {
    type Error = Infallible;
}
