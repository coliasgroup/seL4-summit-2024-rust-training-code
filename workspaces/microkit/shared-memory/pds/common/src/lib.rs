//
// Copyright 2024, Colias Group, LLC
//
// SPDX-License-Identifier: BSD-2-Clause
//

#![no_std]

use zerocopy::{AsBytes, FromBytes, FromZeroes};

pub const REGION_A_SIZE: usize = 1337;

#[repr(C)]
#[derive(AsBytes, FromBytes, FromZeroes)]
pub struct RegionB {
    pub field_1: u64,
    pub foo: [u16; 16],
}
