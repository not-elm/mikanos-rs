#!/bin/bash

# 環境構築用のスクリプトが列挙されたファイルです

sudo apt-get update
sudo apt-get install -y \
    # http client
    curl  \
    # エミュレータ
    qemu-kvm \
    # C言語系のビルドツールなど
    build-essential \
    #util-linux \
    dosfstools \
    ovmf


RUST_VERSION=nightly
# rustup,rustc,cargoをインストールできる
sudo curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain ${RUST_VERSION}
source "${HOME}"/.cargo/env


