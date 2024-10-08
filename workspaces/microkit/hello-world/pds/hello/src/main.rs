//
// Copyright 2024, Colias Group, LLC
//
// SPDX-License-Identifier: BSD-2-Clause
//

#![no_std]
#![no_main]

use sel4_microkit::{debug_println, protection_domain, Handler, Infallible};

#[protection_domain]
fn init() -> impl Handler {
    debug_println!("Hello, World!");
    HandlerImpl
}

struct HandlerImpl;

impl Handler for HandlerImpl {
    type Error = Infallible;
}
