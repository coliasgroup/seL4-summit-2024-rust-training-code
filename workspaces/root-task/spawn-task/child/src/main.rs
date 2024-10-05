//
// Copyright 2024, Colias Group, LLC
//
// SPDX-License-Identifier: BSD-2-Clause
//

#![no_std]
#![no_main]

mod runtime;

const OWN_TCB: sel4::cap::Tcb = sel4::cap::Tcb::from_bits(1);
const INTRA_TASK_EP: sel4::cap::Endpoint = sel4::cap::Endpoint::from_bits(2);

fn main() -> ! {
    sel4::debug_println!("In child task");

    sel4::with_ipc_buffer_mut(|ipc_buf| {
        ipc_buf.msg_regs_mut()[0] = 1337;
    });

    INTRA_TASK_EP.send(sel4::MessageInfoBuilder::default().length(1).build());

    OWN_TCB.tcb_suspend().unwrap();

    unreachable!()
}
