#mkfs.fat
#-n 'MIKAN OS'    ファイルシステムのボリューム名。デフォルト値:無し
#-s 2             クラスタごとセクタ数。2の累乗であること
#-f 2             ファイルシステムのファイルアロケーションテーブル数。デフォルト値:2
#-R 32            予約セクタ数
#-F 32            ファイルアロケーションテーブルのタイプを指定(12,16,32ビットのどれか)
#disk.img         このデバイスにファイルシステムを構築する
# 1. ディスクイメージを作成します。
img=disk.img
sudo rm -f ${img}
qemu-img create -f raw ${img} 200M

mkfs.fat        \
  -n 'OS' \
  -s 2          \
  -f 2          \
  -R 32         \
  -F 32         \
  ${img}

[ "$1" = "test" ] && kernel=$(find target/kernel/debug/deps/ -name '*.elf') || kernel="target/kernel/debug/kernel.elf"

echo "path to kernel.elf=$kernel"
# 2. EFIファイルシステムをディスクイメージ内にコピーします。
sudo rm -r -f
sudo mkdir -p mnt
sudo mount -o loop ${img} mnt
sudo mkdir -p mnt/EFI/BOOT
sudo cp "$HOME"/workspace/mikanos-rs/target/x86_64-unknown-uefi/debug/bootloader.efi ./mnt/EFI/BOOT/BOOTX64.EFI
sudo cp "$kernel" ./mnt/kernel.elf
sudo umount mnt
rm -r -f mnt