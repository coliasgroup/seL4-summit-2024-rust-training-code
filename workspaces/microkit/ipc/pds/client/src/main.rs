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

        sel4_microkit::with_msg_regs_mut(|msg_regs| {
            msg_regs[0] = 0xf00d;
        });

        let msg_info = SERVER.pp_call(MessageInfo::new(0, 1));

        assert_eq!(msg_info.count(), 1);

        sel4_microkit::with_msg_regs(|msg_regs| {
            assert_eq!(msg_regs[0], 0xf33d);
        });

        debug_println!("client: TEST_PASS");

        Ok(())
    }
}
