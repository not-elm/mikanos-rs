# QEMUの起動

IS_DEBUG=$1

if [ "$IS_DEBUG" = "debug" ]
  then
  qemu-system-x86_64 \
    -m 512 \
    -bios 'OVMF.fd' \
    -hda 'disk.img' \
    -monitor stdio \
    -s \
    -S \

else
  qemu-system-x86_64 \
    -m 512 \
    -bios 'OVMF.fd' \
    -hda 'disk.img' \
    -monitor stdio
fi

