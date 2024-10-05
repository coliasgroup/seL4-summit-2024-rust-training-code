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

const SERVER: Channel = Channel::new(13);

#[protection_domain]
fn init() -> HandlerImpl {
    debug_println!("client: initializing");

    let region_a = unsafe {
        ExternallySharedRef::new(
            memory_region_symbol!(region_a_vaddr: *mut [u8], n = REGION_A_SIZE),
        )
    };

    let region_b =
        unsafe { ExternallySharedRef::new(memory_region_symbol!(region_b_vaddr: *mut RegionB)) };

    HandlerImpl { region_a, region_b }
}

struct HandlerImpl {
    region_a: ExternallySharedRef<'static, [u8], ReadOnly>,
    region_b: ExternallySharedRef<'static, RegionB, ReadWrite>,
}

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
