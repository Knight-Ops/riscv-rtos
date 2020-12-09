# Default to the Hifive
ifndef BSP
	BSP = bsp_hifive
endif

# BSP-specific arguments
ifeq ($(BSP),bsp_hifive)
	TARGET = riscv32imac-unknown-none-elf
	OUTPUT = bootloader
	LINKER_FILE = src/bsp/hifive/memory.x
	RUSTC_MISC_ARGS = -C target-feature=-f -C target-cpu=sifive-e31
endif
ifeq ($(BSP),bsp_maix_m1w)
	TARGET = riscv64gc-unknown-none-elf
	OUTPUT = bootloader
	LINKER_FILE = src/bsp/maix_m1w/memory.x
	RUSTC_MISC_ARGS = -C target-feature=-f -C target-cpu=generic-rv64
endif
# ifeq ($(BSP),bsp_rpi4)
# 	TARGET = aarch64-unknown-none
# 	OUTPUT = kernel8.img
# 	QEMU_BINARY = qemu-system-aarch64
# 	QEMU_MACHINE_TYPE = raspi3
# 	QEMU_MISC_ARGS = -serial stdio
# 	LINKER_FILE = src/bsp/rpi4/link.ld
# 	RUSTC_MISC_ARGS = -C target-feature=-fp-armv8 -C target-cpu=cortex-a72
# endif

SOURCES = $(wildcard **/*.rs) $(wildcard **/*.S) $(wildcard **/*.ld)

BUILD_CMD = cargo rustc \
		--target=$(TARGET) \
		--features $(BSP) \
		--release \
		-Z build-std=core \
		-- \
		$(RUSTC_MISC_ARGS)
		# -C link-args=-T$(LINKER_FILE) \

CARGO_OUTPUT = target/$(TARGET)/release/riscv-rtos

OBJCOPY_CMD = cargo objcopy \
		--target=$(TARGET) \
		--features $(BSP) \
		--release \
		-Z build-std=core \
		-- \
		-O binary \
		riscv-rtos.bin

.PHONY: all clippy clean objcopy nm expand server_hifive program_hifive program_maix

all: $(CARGO_OUTPUT)

$(CARGO_OUTPUT): $(SOURCES)
	$(BUILD_CMD)

doc:
	cargo doc --target=$(TARGET) --features $(BSP) --document-private-items
	xdg-open target/$(TARGET)/doc/riscv-rtos/index.html

clippy:
	cargo clippy --target=$(TARGET) --features $(BSP)

clean:
	rm -f bootloader
	cargo clean

expand:
	cargo expand --target=$(TARGET) \
		--features $(BSP) \
		--release \
		-Z build-std=core

objcopy:
	$(OBJCOPY_CMD)

nm:
	cargo nm --target $(TARGET) -- riscv-rtos | sort

server_hifive:
	"C:\Program Files (x86)\SEGGER\JLink\JLinkGDBServer.exe" -device FE310 -if JTAG -speed 4000 -port 3333 -nolocalhostonly

program_hifive:
	cp $(CARGO_OUTPUT) ./bootloader
	~/riscv64-unknown-elf-gcc-8.3.0-2020.04.0-x86_64-linux-ubuntu14/bin/riscv64-unknown-elf-gdb -x gdb_init

program_maix:
	python ./kflash.py -p COM10 riscv-rtos.bin