AS=nasm
LD=ld

KERNEL=kernel/src
BOOTLOADER=boot
BIN=bin

KERNEL_ELF=\
	target/i686-kernel/release/ruslamos

KERNEL_ASMS=$(shell find $(KERNEL) -name '*.asm')
KERNEL_AOBJS=$(filter-out $(KERNEL)/loader.o, $(KERNEL_ASMS:.asm=.o))

.PHONY: dir all clean run

all: os.bin clean

run: 
	qemu-system-i386 -m 128M -drive format=raw,file=os.bin,if=ide,index=0,media=disk

dir:
	@mkdir -p $(BIN)

clean:
	@rm -rf */*.o
	@rm -rf */*/*.o
	@rm -rf */*/*/*.o
	@rm -rf */*/*/*/*.o
	@rm -rf */*/*/*/*/*.o
	@rm -rf */*/*/*/*/*/*.o
	@rm -rf */*/*/*/*/*/*/*.o

.PHONY: $(BIN)/bootloader.bin
$(BIN)/bootloader.bin: 
	@echo " -> START COMPILE BOOTLOADER"
	@cd boot && ./build.sh && cd ../
	@echo " -> BOOTLOADER COMPILED"

$(KERNEL_ELF): $(KERNEL)/loader.o $(KERNEL_AOBJS) 
	@cargo xbuild --target=kernel/i686-kernel.json --release
	@echo " -> KERNEL IMAGE COMPILED"

# $(BIN)/bootloader.bin
os.bin: $(BIN)/bootloader.bin $(KERNEL_ELF) #$(BIN)/testkernel.bin
	@echo " -> START GENERATE IMAGE"
	@cat $^ > $(BIN)/temp.bin
	@dd if=/dev/zero of=$@ bs=512 count=2880
	@dd if=$(BIN)/temp.bin of=$@ conv=notrunc
	@echo " -> OS.BIN GENERATED"

%.o: %.asm
	@$(AS) $< -f elf -o $@
