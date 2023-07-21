sudo mount -o loop fat_disk mnt
sudo cp app.elf mnt/app.elf
sudo umount mnt
zip fat_disk.zip fat_disk