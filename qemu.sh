# QEMUの起動

QEMU_STATE=$1

if [ "$QEMU_STATE" = "debug" ]
  then
  qemu-system-x86_64 \
    -m 512 \
    -bios 'OVMF.fd' \
    -hda 'disk.img' \
    -device nec-usb-xhci,id=xhci \
    -device usb-mouse -device usb-kbd \
    -serial stdio \
    -s \
    -S \
    -device isa-debug-exit,iobase=0xf4,iosize=0x04

elif [ "$QEMU_STATE" = "test" ]; then
  qemu-system-x86_64 \
    -m 512 \
    -bios 'OVMF.fd' \
    -hda 'disk.img' \
    -device nec-usb-xhci,id=xhci \
    -device usb-mouse \
    -serial stdio \
    -display none \
    -device isa-debug-exit,iobase=0xf4,iosize=0x04

else
  qemu-system-x86_64 \
    -m 512 \
    -bios 'OVMF.fd' \
    -hda 'disk.img' \
    -device nec-usb-xhci,id=xhci \
    -device usb-mouse \
    -serial stdio \
    -device isa-debug-exit,iobase=0xf4,iosize=0x04
fi

# QEMUモニタを使う
#    -monitor stdio \