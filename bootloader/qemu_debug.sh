# QEMUの起動

qemu-system-x86_64 \
  -m 512 \
  -bios 'OVMF.fd' \
  -hda 'disk.img' \
  -monitor stdio \
  -s \
  -S \

sh cat_mem_map.sh