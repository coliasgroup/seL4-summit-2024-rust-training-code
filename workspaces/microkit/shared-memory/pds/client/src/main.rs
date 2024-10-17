//
// Copyright 2024, Colias Group, LLC
//
// SPDX-License-Identifier: BSD-2-Clause
//

#![no_std]
#![no_main]

use core::ptr;

use microkit_shared_memory_common::{RegionB, REGION_A_SIZE};

use sel4_externally_shared::{map_field, ExternallySharedRef, ExternallySharedRefExt};
use sel4_microkit::{
    debug_println, memory_region_symbol, protection_domain, Channel, Handler, Infallible,
};

const SERVER: Channel = Channel::new(13);

#[protection_domain]
fn init() -> impl Handler {
    debug_println!("client: initializing");

    let mut region_a = unsafe {
        ExternallySharedRef::new(
            memory_region_symbol!(region_a_vaddr: *mut [u8], n = REGION_A_SIZE),
        )
    };

    let mut region_b =
        unsafe { ExternallySharedRef::new(memory_region_symbol!(region_b_vaddr: *mut RegionB)) };

    debug_println!("client: region_a = {region_a:#x?}");
    debug_println!("client: region_b = {region_b:#x?}");

    region_a.as_mut_ptr().index(13).write(37);

    let region_b_ptr = region_b.as_mut_ptr();
    map_field!(region_b_ptr.foo).as_slice().index(1).write(23);

    HandlerImpl { region_a, region_b }
}

struct HandlerImpl {
    region_a: ExternallySharedRef<'static, [u8]>,
    region_b: ExternallySharedRef<'static, RegionB>,
}

impl Handler for HandlerImpl {
    type Error = Infallible;
}
