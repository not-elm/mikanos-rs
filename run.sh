#!/bin/bash

sh build.sh
cd ./qemu || exit
sh ./make_img.sh
sh ./run_qemu.sh