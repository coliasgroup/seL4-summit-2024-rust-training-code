//
// Copyright 2024, Colias Group, LLC
//
// SPDX-License-Identifier: BSD-2-Clause
//

use core::ops::Range;

pub(crate) struct ObjectAllocator<'a> {
    bootinfo: &'a sel4::BootInfo,
    empty_slots: Range<usize>,
    ut: sel4::cap::Untyped,
}

impl<'a> ObjectAllocator<'a> {
    pub(crate) fn new(bootinfo: &'a sel4::BootInfo) -> Self {
        Self {
            bootinfo,
            empty_slots: bootinfo.empty().range(),
            ut: find_largest_kernel_untyped(bootinfo),
        }
    }

    pub(crate) fn allocate_slot(&mut self) -> sel4::init_thread::Slot {
        sel4::init_thread::Slot::from_index(self.empty_slots.next().unwrap())
    }

    pub(crate) fn allocate(&mut self, blueprint: sel4::ObjectBlueprint) -> sel4::cap::Unspecified {
        let slot = self.allocate_slot();
        self.ut
            .untyped_retype(
                &blueprint,
                &sel4::init_thread::slot::CNODE
                    .cap()
                    .absolute_cptr_for_self(),
                slot.index(),
                1,
            )
            .unwrap();
        slot.cap()
    }

    pub(crate) fn allocate_fixed_sized<T: sel4::CapTypeForObjectOfFixedSize>(
        &mut self,
    ) -> sel4::Cap<T> {
        self.allocate(T::object_blueprint()).cast()
    }

    pub(crate) fn allocate_variable_sized<T: sel4::CapTypeForObjectOfVariableSize>(
        &mut self,
        size_bits: usize,
    ) -> sel4::Cap<T> {
        self.allocate(T::object_blueprint(size_bits)).cast()
    }

    pub(crate) fn recklessly_allocate_at(
        &mut self,
        blueprint: sel4::ObjectBlueprint,
        paddr: usize,
    ) -> sel4::cap::Unspecified {
        let (ut_ix, ut_desc) = self
            .bootinfo
            .untyped_list()
            .iter()
            .enumerate()
            .find(|(_i, desc)| {
                (desc.paddr()..(desc.paddr() + (1 << desc.size_bits()))).contains(&paddr)
            })
            .unwrap();

        let ut_cap = self.bootinfo.untyped().index(ut_ix).cap();

        trim_untyped(
            ut_cap,
            ut_desc.paddr(),
            paddr,
            self.allocate_slot(),
            self.allocate_slot(),
        );

        let obj_slot = self.allocate_slot();

        ut_cap
            .untyped_retype(
                &blueprint,
                &sel4::init_thread::slot::CNODE
                    .cap()
                    .absolute_cptr_for_self(),
                obj_slot.index(),
                1,
            )
            .unwrap();

        obj_slot.cap()
    }
}

fn find_largest_kernel_untyped(bootinfo: &sel4::BootInfo) -> sel4::cap::Untyped {
    let (ut_ix, _desc) = bootinfo
        .untyped_list()
        .iter()
        .enumerate()
        .filter(|(_i, desc)| !desc.is_device())
        .max_by_key(|(_i, desc)| desc.size_bits())
        .unwrap();

    bootinfo.untyped().index(ut_ix).cap()
}

fn trim_untyped(
    ut: sel4::cap::Untyped,
    ut_paddr: usize,
    target_paddr: usize,
    free_slot_a: sel4::init_thread::Slot,
    free_slot_b: sel4::init_thread::Slot,
) {
    let rel_a = sel4::init_thread::slot::CNODE
        .cap()
        .absolute_cptr(free_slot_a.cptr());
    let rel_b = sel4::init_thread::slot::CNODE
        .cap()
        .absolute_cptr(free_slot_b.cptr());
    let mut cur_paddr = ut_paddr;
    while cur_paddr != target_paddr {
        let size_bits = (target_paddr - cur_paddr).ilog2().try_into().unwrap();
        ut.untyped_retype(
            &sel4::ObjectBlueprint::Untyped { size_bits },
            &sel4::init_thread::slot::CNODE
                .cap()
                .absolute_cptr_for_self(),
            free_slot_b.index(),
            1,
        )
        .unwrap();
        rel_a.delete().unwrap();
        rel_a.move_(&rel_b).unwrap();
        cur_paddr += 1 << size_bits;
    }
}
