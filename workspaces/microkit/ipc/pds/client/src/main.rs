//
// Copyright 2024, Colias Group, LLC
//
// SPDX-License-Identifier: BSD-2-Clause
//

#![no_std]
#![no_main]

use sel4_microkit::{debug_println, protection_domain, Channel, Handler, Infallible, MessageInfo};

const SERVER: Channel = Channel::new(13);

#[protection_domain]
fn init() -> impl Handler {
    debug_println!("client: initializing");
    HandlerImpl
}

struct HandlerImpl;

impl Handler for HandlerImpl {
    type Error = Infallible;

    fn notified(&mut self, channel: Channel) -> Result<(), Self::Error> {
        debug_println!("client: notified by {:?}", channel);

        debug_println!("client: TEST_PASS");

        Ok(())
    }
}
