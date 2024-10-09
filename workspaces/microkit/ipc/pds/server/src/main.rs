//
// Copyright 2024, Colias Group, LLC
//
// SPDX-License-Identifier: BSD-2-Clause
//

#![no_std]
#![no_main]

use sel4_microkit::{debug_println, protection_domain, Channel, Handler, Infallible, MessageInfo};

const CLIENT: Channel = Channel::new(37);

#[protection_domain]
fn init() -> impl Handler {
    debug_println!("server: initializing");

    debug_println!("server: notifying client");

    CLIENT.notify();

    HandlerImpl
}

struct HandlerImpl;

impl Handler for HandlerImpl {
    type Error = Infallible;

    fn protected(
        &mut self,
        channel: Channel,
        msg_info: MessageInfo,
    ) -> Result<MessageInfo, Self::Error> {
        debug_println!("server: called by {:?}", channel);

        todo!()
    }
}
