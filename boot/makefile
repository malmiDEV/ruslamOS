BIN=../bin

BOOT_ELF=target/i686-boot/release/boot

.PHONY: all

all: $(BIN)/bootlod_image.bin 

.PHONY: $(BIN)/bootsector.bin
$(BIN)/bootsector.bin: asm/bootsector.asm
	@nasm $< -f bin -o $@

.PHONY: $(BIN)/stage2.bin
$(BIN)/stage2.bin: asm/stage2.asm
	@nasm $< -f bin -o $@

$(BOOT_ELF): 
	@nasm asm/loader.asm -f elf -o asm/loader.o
	@cargo xbuild --target=i686-boot.json --release

.PHONY: $(BIN)/bootlod_image.bin
$(BIN)/bootlod_image.bin: $(BIN)/bootsector.bin $(BIN)/stage2.bin $(BOOT_ELF)
	@cat $^ > $@
