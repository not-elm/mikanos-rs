#!/bin/bash

# 環境構築用のスクリプトが列挙されたファイルです

sudo apt-get update
sudo apt-get install -y curl  \
    qemu-kvm \
    build-essential \
    util-linux \
    dosfstools \
    gdb \
    nasm \
    ovmf


RUST_VERSION=nightly
# rustup,rustc,cargoをインストールできる
sudo curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain ${RUST_VERSION}
source "${HOME}"/.cargo/env
rustup target add x86_64-unknown-none