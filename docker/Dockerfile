#
# Copyright 2024, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

FROM debian:bookworm

RUN apt-get update -q && \
    apt-get install -y --no-install-recommends \
        build-essential \
        curl \
        git \
        rsync \
        # for seL4
        python3-dev \
        python3-venv \
        cmake \
        ninja-build \
        device-tree-compiler \
        libxml2-utils \
        gcc-riscv64-unknown-elf \
        # for bindgen
        libclang-dev \
        # for test scripts
        python3-pexpect \
        python3-requests \
        # for book
        linkchecker \
        # for hacking
        bash-completion \
        man \
        less \
        sudo \
    && \
    # install these without --no-install-recommends
    apt-get install -y \
        qemu-system-arm \
        qemu-system-misc \
    && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /tmp

RUN set -eux; \
    host=$(uname -m); \
    url="https://developer.arm.com/-/media/Files/downloads/gnu/12.2.rel1/binrel/arm-gnu-toolchain-12.2.rel1-$host-aarch64-none-elf.tar.xz"; \
    dst=/opt/gcc-aarch64-none-elf; \
    mkdir -p $dst; \
    curl -sSfL "$url" | tar -xJ -C $dst --strip-components=1;

ENV PATH=/opt/gcc-aarch64-none-elf/bin:$PATH

ARG RUSTUP_CHANNEL

RUN curl -sSf https://sh.rustup.rs | \
        bash -s -- -y --no-modify-path \
            --default-toolchain $RUSTUP_CHANNEL \
            --component rustfmt \
            --component rust-src

ENV PATH=/root/.cargo/bin:$PATH

RUN cargo install mdbook@0.4.36

ENV SEL4_INSTALL_DIR=/opt/seL4

RUN set -eux; \
    git clone \
        https://github.com/seL4/seL4.git \
        --config advice.detachedHead=false; \
    cd seL4; \
    git checkout cd6d3b8c25d49be2b100b0608cf0613483a6fffa;

RUN set -eux; \
    cd seL4; \
    python3 -m venv pyenv; \
    export PATH=$(realpath ./pyenv/bin):$PATH; \
    pip install tools/python-deps; \
    cmake \
        -DCROSS_COMPILER_PREFIX=aarch64-none-elf- \
        -DCMAKE_INSTALL_PREFIX=$SEL4_INSTALL_DIR \
        -DKernelPlatform=qemu-arm-virt \
        # TODO further develop kernel loader to make KernelArmHypervisorSupport=ON unecessary
        -DKernelArmHypervisorSupport=ON \
        -DKernelVerificationBuild=OFF \
        -DARM_CPU=cortex-a53 \
        -G Ninja \
        -S . \
        -B build; \
    ninja -C build all; \
    ninja -C build install; \
    rm -rf $(pwd);

RUN set -eux; \
    url="https://github.com/seL4/rust-sel4"; \
    rev="1cd063a0f69b2d2045bfa224a36c9341619f0e9b"; \
    common_args="--git $url --rev $rev --root $SEL4_INSTALL_DIR"; \
    CC_aarch64_unknown_none=aarch64-none-elf-gcc \
    SEL4_PREFIX=$SEL4_INSTALL_DIR \
        cargo install \
            -Z build-std=core,compiler_builtins \
            -Z build-std-features=compiler-builtins-mem \
            --target aarch64-unknown-none \
            $common_args \
            sel4-kernel-loader; \
    cargo install \
        $common_args \
        sel4-kernel-loader-add-payload;

ENV MICROKIT_SDK=/opt/microkit/sdk

RUN set -eux; \
    git clone \
        https://github.com/seL4/microkit.git \
        --config advice.detachedHead=false; \
    cd microkit; \
    git checkout 395cf0e5be489bbd7586b012188fc1f712cd1a57; \
    git clone \
        https://github.com/seL4/seL4.git \
        --config advice.detachedHead=false; \
    cd seL4; \
    git checkout 4cae30a6ef166a378d4d23697b00106ce7e4e76f;

RUN set -eux; \
    cd microkit; \
    python3 -m venv pyenv; \
    export PATH=$(realpath ./pyenv/bin):$PATH; \
    pip install --upgrade pip setuptools wheel; \
    pip install -r requirements.txt; \
    python3 build_sdk.py \
        --sel4 ./seL4 \
        --tool-target-triple $(uname -m)-unknown-linux-gnu \
        --skip-docs \
        --skip-tar; \
    mkdir $(dirname $MICROKIT_SDK); \
    mv release/microkit-sdk-* $MICROKIT_SDK; \
    rm -rf $(pwd);

ARG UID
ARG GID

RUN set -eux; \
    if [ $UID -eq 0 ]; then \
        if [ $GID -ne 0 ]; then \
            echo "error: \$UID == 0 but \$GID != 0" >&2; \
            exit 1; \
        fi; \
    else \
        if getent passwd $UID; then \
            echo "error: \$UID $UID already exists" >&2; \
            exit 1; \
        fi; \
        if ! getent group $GID; then \
            groupadd --gid $GID x; \
        fi; \
        useradd --uid $UID --gid $GID --groups sudo --create-home x; \
    fi;

RUN echo '%sudo ALL=(ALL) NOPASSWD:ALL' >> /etc/sudoers

USER $UID

RUN set -eux; \
    if [ $UID -ne 0 ]; then \
        curl -sSf https://sh.rustup.rs | \
            bash -s -- -y --no-modify-path \
                --default-toolchain $RUSTUP_CHANNEL \
                --component rustfmt \
                --component rust-src; \
    fi;

ENV PATH=/home/x/.cargo/bin:$PATH

ENV SHELL=/bin/bash

# TODO HACK
RUN set -eu; \
    f=~/.gitconfig; \
    echo '[safe]' > $f; \
    echo 'directory = /work/code' >> $f;

WORKDIR /work
