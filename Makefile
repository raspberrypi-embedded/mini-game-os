TARGRT 		:= aarch64-unknown-none
MODE   		:= debug
ELF	   		:= target/$(TARGRT)/$(MODE)/raspi4-game
SDCARD 		:= /media/kuangjux/boot
KERNEL_IMG 	:= kernel8.img
CONFIG_FILE	:= config.txt

QEMU 	     := qemu-system-aarch64 
QEMU_OPTIONS := -M raspi3 -kernel kernel8.img -serial null -serial stdio

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
	@sudo cp $(CONFIG_FILE) $(SDCARD)
	

kernel:
	@echo Platform: $(BOARD)
	@cp src/linker/$(BOARD).ld src/linker.ld
	@cargo build --features "board_$(BOARD)"
	@cp $(ELF) kernel8.elf
	@$(OBJCOPY) -O binary kernel8.elf kernel8.img

qemu: kernel
	$(QEMU) $(QEMU_OPTIONS)

clean:
	@cargo clean
	@rm kernel8.*