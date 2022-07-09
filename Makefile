TARGRT := aarch64-unknown-none
MODE   := debug
ELF	   := target/$(TARGRT)/$(MODE)/raspi4-game

BOARD ?= raspi4

# Building mode argument
ifeq ($(MODE), release)
	MODE_ARG := --release
endif

# Binutils
OBJDUMP := rust-objdump --arch-name=aarch64
OBJCOPY := rust-objcopy --binary-architecture=aarch64

build:
	@echo Platform: $(BOARD)
	@cp src/linker/$(BOARD).ld src/linker.ld
	@cargo build --features "board_$(BOARD)"
	@cp $(ELF) kernel8.elf
	@$(OBJCOPY) -O binary kernel8.elf kernel8.img

clean:
	@cargo clean
	rm kernel7.*