#!/usr/bin

cd ./common || exit
cargo build
cd ../kernel || exit
cargo build
cd ../bootloader || exit
cargo build