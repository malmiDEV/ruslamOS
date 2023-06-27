nasm asm/bootsector.asm -f bin -o ../bin/bootsector.bin 
nasm asm/stage2.asm -f bin -o ../bin/stage2.bin

nasm asm/loader.asm -f elf -o asm/loader.o
cargo xbuild --target=i686-boot.json --release

cat ../bin/bootsector.bin ../bin/stage2.bin target/i686-boot/release/boot > ../bin/bootloader.bin

size=$(stat -f %z ../bin/bootloader.bin)
newsize=$(expr $size - $(expr $size % 512) + 512)

echo "BOOTLOADER SECTOR 	-> " $size
echo "BOOTLOADER SECTOR COUNT -> " $newsize
echo "SECTOR PADD	        -> " $(expr $newsize - $size)

dd if=/dev/zero of=../bin/bootloader.bin bs=1 seek=$size count=$(expr $newsize - $size + 512)