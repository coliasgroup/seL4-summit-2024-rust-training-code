#
# Copyright 2024, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

build_dir := build

.PHONY: none
none:

$(build_dir):
	mkdir -p $(build_dir)

workspaces := \
	root-task \
	microkit

examples := \
    root-task/hello-world \
    root-task/kernel-objects \
    root-task/address-space \
    root-task/serial-device \
    root-task/spawn-task \
    root-task/spawn-thread \
    microkit/hello-world \
    microkit/ipc \
    microkit/shared-memory \
    microkit/banscii

.PHONY: clean-each-example test-each-example
clean-each-example test-each-example:
	set -eu; \
	$(foreach example,$(examples), \
		$(MAKE) -C workspaces/$(example) $(subst -each-example,,$@);)

.PHONY: clean
clean:
	rm -rf $(build_dir)

.PHONY: clean-all
clean-all: clean clean-each-example

.PHONY: test
test: test-each-example

.PHONY: fmt
fmt:
	set -eu; \
	$(foreach workspace,$(workspaces), \
		(cd workspaces/$(workspace) && cargo fmt);)

.PHONY: fmt-check
fmt-check:
	set -eu; \
	$(foreach workspace,$(workspaces), \
		(cd workspaces/$(workspace) && cargo fmt --check);)

.PHONY: update
update:
	set -eu; \
	$(foreach workspace,$(workspaces), \
		(cd workspaces/$(workspace) && cargo update);)

rustdoc_dir := $(build_dir)/rustdoc

.PHONY: rustdoc
rustdoc: 
	set -eu; \
	$(foreach workspace,$(workspaces), \
		(cd workspaces/$(workspace) && \
			cargo doc --target-dir $(abspath $(rustdoc_dir)/$(workspace)));)

.PHONY: clean-rustdoc
clean-rustdoc:
	rm -rf $(rustdoc_dir)

.PHONY: prune-rustdoc
prune-rustdoc:
	set -eu; \
	cd $(rustdoc_dir); \
	rm -rf */debug */*/debug

.PHONY: check-step
check-step: fmt-check test

# exported_rustdoc_dir := $(build_dir)/exported-rustdoc

# .PHONY: exported-rustdoc
# exported-rustdoc: rustdoc | $(build_dir)
# 	rm -rf $(exported_rustdoc_dir)
# 	time rsync -av $(rustdoc_dir)/ $(exported_rustdoc_dir)/ \
# 		--info=progress2 --info=name0 \
# 		--exclude '/*/debug' \
# 		--exclude '/*/*/debug' \
# 		--exclude '/*/.*.json' \
# 		--exclude '/*/CACHEDIR.TAG'
