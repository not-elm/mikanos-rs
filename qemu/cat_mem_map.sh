#!/bin/bash

# 書き込んだメモリーマップの情報をディスクイメージから読み取ります
sudo mkdir -p mnt
sudo mount -o loop disk.img mnt
sudo cat ./mnt/mem_map
# アンマウントするのを忘れないこと！
sudo umount mnt