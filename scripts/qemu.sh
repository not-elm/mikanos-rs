# QEMUの起動

QEMU_STATE=$1

if [ "$QEMU_STATE" = "debug" ]; then
  qemu-system-x86_64 \
    -bios OVMF.fd \
    -drive if=ide,index=0,media=disk,format=raw,file='disk.img' \
    -device nec-usb-xhci,id=xhci \
    -device usb-mouse \
    -device usb-kbd \
    -serial stdio \
    -s \
    -S \
    -m 8G \
    -device isa-debug-exit,iobase=0xf4,iosize=0x04
elif [ "$QEMU_STATE" = "test" ];
then
  qemu-system-x86_64 \
    -bios OVMF.fd \
    -drive if=ide,index=0,media=disk,format=raw,file='disk.img' \
    -device nec-usb-xhci,id=xhci \
    -device usb-mouse \
    -device usb-kbd \
    -serial stdio \
    -display none \
    -m 8G \
    -device isa-debug-exit,iobase=0xf4,iosize=0x04
else
  qemu-system-x86_64 \
    -m 2G \
    -bios OVMF.fd \
    -drive if=ide,index=0,media=disk,format=raw,file='disk.img' \
    -device nec-usb-xhci,id=xhci \
    -device usb-mouse \
    -device usb-kbd \
    -serial stdio
fi

# QEMUモニタを使う
#    -monitor stdio \
