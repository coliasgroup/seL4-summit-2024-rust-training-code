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

        assert_eq!(msg_info.count(), 1);

        sel4_microkit::with_msg_regs(|msg_regs| {
            assert_eq!(msg_regs[0], 0xf00d);
        });

        sel4_microkit::with_msg_regs_mut(|msg_regs| {
            msg_regs[0] = 0xf33d;
        });

        debug_println!("server: TEST_PASS");

        Ok(MessageInfo::new(0, 1))
    }
}
