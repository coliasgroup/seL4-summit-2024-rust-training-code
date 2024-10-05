#
# Copyright 2024, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

include ../../../mk/microkit.mk

crate = $(build_dir)/$(1).elf

define build_crate

$(crate): FORCE
	cargo build \
		$(common_cargo_args) \
		-p $(1)

endef

crate_names := \
	microkit-shared-memory-client \
	microkit-shared-memory-server

crates := $(foreach crate_name,$(crate_names),$(call crate,$(crate_name)))

$(eval $(foreach crate_name,$(crate_names),$(call build_crate,$(crate_name))))

$(system_description): shared-memory.system
	cp $< $@

$(image): $(crates)
