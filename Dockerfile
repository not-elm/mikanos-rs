FROM ubuntu:18.04

RUN apt-get update && apt-get install -y \
    # http client
    curl  \
    # エミュレータ
    qemu-kvm \
    # C言語系のビルドツールなど
    build-essential \
    #util-linux \
    dosfstools \
    ovmf \
    sudo
.
└── mnt
    ├── EFI
    │   └── BOOT
    │       └── BOOTx64.EFI
    └── kernel.elf

ENV RUST_VERSION nightly
# rustup,rustc,cargoをインストールできる
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain ${RUST_VERSION}
ENV PATH $PATH:$HOME/.cargo/bin


# xlaunch用
ENV DISPLAY host.docker.internal:0.0

RUN mkdir works
WORKDIR works

#CMD ["rustup target add thumbv7em-none-eabihf"]



