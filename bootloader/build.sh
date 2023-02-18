#!/bin/bash


cargo build

sh ../qemu/make_img.sh
sh ../qemu/run.sh