//
// Copyright 2024, Colias Group, LLC
//
// SPDX-License-Identifier: BSD-2-Clause
//

#![no_std]
#![no_main]

use core::ptr;

use microkit_shared_memory_common::{RegionB, REGION_A_SIZE};

use sel4_externally_shared::{
    access::ReadOnly, map_field, ExternallySharedRef, ExternallySharedRefExt,
};
use sel4_microkit::{
    debug_println, memory_region_symbol, protection_domain, Channel, Handler, Infallible,
};

const CLIENT: Channel = Channel::new(37);

#[protection_domain]
fn init() -> impl Handler {
    debug_println!("server: initializing");

    let region_a = unsafe {
        ExternallySharedRef::new(
            memory_region_symbol!(region_a_vaddr: *mut [u8], n = REGION_A_SIZE),
        )
    };

    let region_b =
        unsafe { ExternallySharedRef::new(memory_region_symbol!(region_b_vaddr: *mut RegionB)) };

    debug_println!("server: region_a = {region_a:#x?}");
    debug_println!("server: region_b = {region_b:#x?}");

    HandlerImpl { region_a, region_b }
}

struct HandlerImpl {
    region_a: ExternallySharedRef<'static, [u8], ReadOnly>,
    region_b: ExternallySharedRef<'static, RegionB, ReadOnly>,
}

impl Handler for HandlerImpl {
    type Error = Infallible;

    fn notified(&mut self, _channel: Channel) -> Result<(), Self::Error> {
        assert_eq!(self.region_a.as_ptr().index(13).read(), 37);

        let region_b_ptr = self.region_b.as_mut_ptr();
        assert_eq!(map_field!(region_b_ptr.foo).as_slice().index(1).read(), 23);

        Ok(())
    }
}
