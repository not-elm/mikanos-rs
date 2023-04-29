mkdir mnt
sudo mount -o loop disk.img mnt
cat ./mnt/memory_map
cat ./mnt/memory_map > memory_map
sudo umount mnt
rm mnt -r -f

