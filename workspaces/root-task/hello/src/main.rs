//
// Copyright 2023, Colias Group, LLC
//
// SPDX-License-Identifier: BSD-2-Clause
//

#![no_std]
#![no_main]

use sel4_root_task::{panicking, root_task};

#[root_task]
fn main(_bootinfo: &sel4::BootInfoPtr) -> ! {
    sel4::debug_println!("Hello, World!");

    panicking::set_hook(&|info| {
        sel4::debug_println!("!!!!");
        sel4::debug_println!("{info}");
        sel4::debug_println!("????");
    });

    let result = panicking::catch_unwind(|| {
        panic!("uh oh");
    });
    assert!(result.is_err());

    sel4::debug_println!("TEST_PASS");

    sel4::init_thread::suspend_self()
}
