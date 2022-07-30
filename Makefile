TARGRT 		:= aarch64-unknown-none
MODE   		:= debug
ELF	   		:= target/$(TARGRT)/$(MODE)/raspi4-game
SDCARD 		:= /media/kuangjux/boot
KERNEL_FILE := kernel8.elf
KERNEL_IMG 	:= kernel8.img
KERNEL_ASM	:= kernel8.S
CONFIG_FILE	:= config.txt

GDB			:= gdb-multiarch

QEMU 	     := qemu-system-aarch64 
QEMUOPTS     := -M raspi3b -kernel kernel8.img -serial null -serial stdio
QEMUOPTS     += -display sdl 

BOARD ?= qemu

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
	$(QEMU) $(QEMUOPTS)

clean:
	@cargo clean
	@rm kernel8.*

gdb:
	gdb-multiarch -n -x .gdbinit

qemu-gdb: kernel
	@echo "***"
	@echo "*** make qemu-gdb'." 1>&2
	@echo "***"
	$(QEMU) -nographic $(QEMUOPTS) -S 

debug: kernel
	@tmux new-session -d \
		"$(QEMU) $(QEMUOPTS) -s -S" && \
		tmux split-window -h "$(GDB) -ex 'file $(KERNEL_FILE)' -ex 'set arch auto' -ex 'target remote localhost:1234'" && \
		tmux -2 attach-session -d

asm: kernel
	@$(OBJDUMP) -d $(KERNEL_FILE) > $(KERNEL_ASM)