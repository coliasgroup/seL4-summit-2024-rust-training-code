//
// Copyright 2024, Colias Group, LLC
//
// SPDX-License-Identifier: BSD-2-Clause
//

#![no_std]
#![no_main]

use microkit_shared_memory_common::{RegionB, REGION_A_SIZE};

use sel4_externally_shared::{
    access::{ReadOnly, ReadWrite},
    ExternallySharedRef, ExternallySharedRefExt,
};
use sel4_microkit::{
    debug_println, memory_region_symbol, protection_domain, Channel, Handler, Infallible,
    MessageInfo,
};

const CLIENT: Channel = Channel::new(37);

#[protection_domain]
fn init() -> HandlerImpl {
    debug_println!("server: initializing");

    let region_a = unsafe {
        ExternallySharedRef::new(
            memory_region_symbol!(region_a_vaddr: *mut [u8], n = REGION_A_SIZE),
        )
    };

    let region_b =
        unsafe { ExternallySharedRef::new(memory_region_symbol!(region_b_vaddr: *mut RegionB)) };

    debug_println!("server: notifying client");

    CLIENT.notify();

    HandlerImpl { region_a, region_b }
}

struct HandlerImpl {
    region_a: ExternallySharedRef<'static, [u8], ReadOnly>,
    region_b: ExternallySharedRef<'static, RegionB, ReadWrite>,
}

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
