#!/bin/bash

cargo build
cd ./qemu || exit
sh ./make_img.sh
sh ./run_qemu.sh