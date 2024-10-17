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

const CLIENT: Channel = Channel::new(37);

#[protection_domain]
fn init() -> impl Handler {
    debug_println!("server: initializing");

    let region_a = *var!(region_a_vaddr: usize = 0) as *mut [u8; REGION_A_SIZE];
    let region_b = *var!(region_b_vaddr: usize = 0) as *mut RegionB;

    debug_println!("server: region_a = {region_a:#x?}");
    debug_println!("server: region_b = {region_b:#x?}");

    HandlerImpl { region_a, region_b }
}

struct HandlerImpl {
    region_a: *mut [u8; REGION_A_SIZE],
    region_b: *mut RegionB,
}

impl Handler for HandlerImpl {
    type Error = Infallible;

    fn notified(&mut self, _channel: Channel) -> Result<(), Self::Error> {
        unsafe {
            assert_eq!((self.region_a as *mut u8).offset(13).read(), 37);
        }

        unsafe {
            assert_eq!(
                (ptr::addr_of!((*self.region_b).foo) as *mut u8)
                    .offset(1)
                    .read(),
                23
            );
        }

        Ok(())
    }
}
