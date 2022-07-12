TARGRT := aarch64-unknown-none
MODE   := debug
ELF	   := target/$(TARGRT)/$(MODE)/raspi4-game
SDCARD := /dev/sdb1
KERNEL_IMG := kernel8.img

BOARD ?= raspi4

# Building mode argument
ifeq ($(MODE), release)
	MODE_ARG := --release
endif

# Binutils
OBJDUMP := rust-objdump --arch-name=aarch64
OBJCOPY := rust-objcopy --binary-architecture=aarch64

.PNONY: kernel clean sdcard 

build: kernel sdcard

sdcard: 
	@echo "Are you sure write to $(SDCARD) ? [y/N] " && read ans && [ $${ans:-N} = y ]
	@sudo cp $(KERNEL_IMG) $(SDCARD)

kernel:
	@echo Platform: $(BOARD)
	@cp src/linker/$(BOARD).ld src/linker.ld
	@cargo build --features "board_$(BOARD)"
	@cp $(ELF) kernel8.elf
	@$(OBJCOPY) -O binary kernel8.elf kernel8.img

clean:
	@cargo clean
	@rm kernel8.*