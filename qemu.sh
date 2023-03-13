# QEMUの起動

IS_DEBUG=$1

if [ "$IS_DEBUG" = "debug" ]
  then
  qemu-system-x86_64 \
    -m 512 \
    -bios 'OVMF.fd' \
    -hda 'disk.img' \
    -device nec-usb-xhci,id=xhci \
    -device usb-mouse -device usb-kbd \
    -monitor stdio \
    -s \
    -S \

else
  qemu-system-x86_64 \
    -m 512 \
    -bios 'OVMF.fd' \
    -hda 'disk.img' \
    -device nec-usb-xhci,id=xhci \
    -device usb-mouse \
    -monitor stdio
fi

