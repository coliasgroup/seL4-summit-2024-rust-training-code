//
// Copyright 2023, Colias Group, LLC
//
// SPDX-License-Identifier: BSD-2-Clause
//

#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec;

use sel4_logging::{log::info, LevelFilter, Logger, LoggerBuilder};
use sel4_root_task::{panicking, root_task};

static LOGGER: Logger = LoggerBuilder::const_default()
    .level_filter(LevelFilter::Info)
    .write(|s| sel4::debug_print!("{}", s))
    .build();

#[root_task(stack_size = 256 * 1024, heap_size = 64 * 1024)]
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

    use_heap();

    LOGGER.set().unwrap();

    info!("logging info");

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

fn use_heap() {
    let mut v = vec![43, 612, 620, 12, 3, 6];
    v.sort();
    assert!(v.is_sorted());
}
