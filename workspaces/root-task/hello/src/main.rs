//
// Copyright 2023, Colias Group, LLC
//
// SPDX-License-Identifier: BSD-2-Clause
//

#![no_std]
#![no_main]

use sel4_root_task::{panicking, root_task};

#[root_task(stack_size = 256 * 1024)]
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

    cause_stack_overflow();

    sel4::debug_println!("TEST_PASS");

    sel4::init_thread::suspend_self()
}

fn cause_stack_overflow() {
    fn f(i: i32) {
        sel4::debug_print!(".");
        let _arr = [0u8; 1024];
        if i > 0 {
            f(i - 1);
        }
    }

    sel4::debug_println!("trying to caues stack overflow");

    f(150);

    sel4::debug_println!("");
    sel4::debug_println!("did not cause stack overflow");
}
