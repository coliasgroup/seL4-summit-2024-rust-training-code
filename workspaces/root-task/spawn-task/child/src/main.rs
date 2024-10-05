//
// Copyright 2024, Colias Group, LLC
//
// SPDX-License-Identifier: BSD-2-Clause
//

#![no_std]
#![no_main]

mod device;
mod runtime;

use device::Device;
use runtime::addr_of_page_beyond_image;

const OWN_TCB: sel4::cap::Tcb = sel4::cap::Tcb::from_bits(1);
const INTRA_TASK_EP: sel4::cap::Endpoint = sel4::cap::Endpoint::from_bits(2);
const IRQ_HANDLER: sel4::cap::IrqHandler = sel4::cap::IrqHandler::from_bits(3);
const IRQ_NFN: sel4::cap::Notification = sel4::cap::Notification::from_bits(4);

fn main() -> ! {
    sel4::debug_println!("In child task");

    sel4::with_ipc_buffer_mut(|ipc_buf| {
        ipc_buf.msg_regs_mut()[0] = 1337;
    });

    let serial_device_mmio_page_addr = addr_of_page_beyond_image(1);

    let serial_device = unsafe { Device::new(serial_device_mmio_page_addr as *mut _) };

    serial_device.init();

    for c in b"echo> " {
        serial_device.put_char(*c);
    }

    loop {
        serial_device.clear_all_interrupts();
        IRQ_HANDLER.irq_handler_ack().unwrap();

        IRQ_NFN.wait();

        while let Some(c) = serial_device.get_char() {
            serial_device.put_char(b'[');
            serial_device.put_char(c);
            serial_device.put_char(b']');
        }
    }

    INTRA_TASK_EP.send(sel4::MessageInfoBuilder::default().length(1).build());

    OWN_TCB.tcb_suspend().unwrap();

    unreachable!()
}
