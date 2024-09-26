#
# Copyright 2024, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

include ../../../mk/root-task.mk

child_crate_name := spawn-task-child
child_crate := $(build_dir)/$(child_crate_name).elf

$(child_crate): FORCE
	cargo build \
		$(common_cargo_args) \
		-p $(child_crate_name)

crate_name := spawn-task
crate := $(build_dir)/$(crate_name).elf

$(crate): $(child_crate) FORCE
	CHILD_ELF=$(abspath $(child_crate)) \
		cargo build \
			$(common_cargo_args) \
			-p $(crate_name)

$(app): $(crate)
	cp $< $@
