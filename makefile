AS=nasm
LD=ld

KERNEL=kernel/src
BOOTLOADER=boot
BIN=bin

KERNEL_ELF=\
	target/i686-kernel/release/ruslamos

KERNEL_ASMS=$(shell find $(KERNEL) -name '*.asm')
KERNEL_AOBJS=$(filter-out $(KERNEL)/kstart.o, $(KERNEL_ASMS:.asm=.o))

.PHONY: dir all clean run

all: os.bin clean

run: 
	qemu-system-i386 -m 128M -drive format=raw,file=os.bin,if=ide,index=0,media=disk

dir:
	mkdir -p $(BIN)

clean:
	rm -rf */*.o
	rm -rf */*/*.o
	rm -rf */*/*/*.o
	rm -rf */*/*/*/*.o
	rm -rf */*/*/*/*/*.o
	rm -rf */*/*/*/*/*/*.o
	rm -rf */*/*/*/*/*/*/*.o

.PHONY: $(BIN)/bootsector.bin
$(BIN)/bootsector.bin: $(BOOTLOADER)/bootsector.asm
	$(AS) $< -f bin -o $@

.PHONY: $(BIN)/stage2.bin
$(BIN)/stage2.bin: $(BOOTLOADER)/stage2.asm
	$(AS) $< -f bin -o $@

.PHONY: $(BIN)/font.bin
$(BIN)/font.bin: $(BOOTLOADER)/font.asm
	$(AS) $< -f bin -o $@

$(KERNEL_ELF): $(KERNEL)/kstart.o $(KERNEL_AOBJS)
	cargo xbuild --target=kernel/i686-kernel.json --release

os.bin: $(BIN)/bootsector.bin $(BIN)/stage2.bin $(BIN)/font.bin $(KERNEL_ELF)
	cat $^ > $(BIN)/temp.bin
	dd if=/dev/zero of=$@ bs=512 count=2880
	dd if=$(BIN)/temp.bin of=$@ conv=notrunc

%.o: %.asm
	$(AS) $< -f elf -o $@
