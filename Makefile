#####
## COMMON
#####
PROJECT_NAME=taos

#####
## BUILD
#####
CROSS_COMPILER = riscv64-unknown-linux-gnu
RISCV_GCC     := $(CROSS_COMPILER)-gcc
RISCV_GXX     := $(CROSS_COMPILER)-g++
RISCV_OBJDUMP := $(CROSS_COMPILER)-objdump
RISCV_OBJCOPY := $(CROSS_COMPILER)-objcopy
RISCV_GDB     := $(CROSS_COMPILER)-gdb
RISCV_AR      := $(CROSS_COMPILER)-ar
RISCV_SIZE    := $(CROSS_COMPILER)-size

CFLAGS=-Wall -Wextra -pedantic -O0 -g -std=c++17
CFLAGS+=-static -ffreestanding -nostdlib -fno-rtti -fno-exceptions
CFLAGS+=-march=rv32imac -mabi=ilp32
CFLAGS+=-Wl,-Map=$(PROJECT_NAME).map
INCLUDES=
LINKER_SCRIPT=-Tsrc/lds/virt.lds
TYPE=release
RUST_TARGET=./target/riscv32imac-unknown-none-elf/$(TYPE)
LIBS=-L$(RUST_TARGET)
SOURCES_ASM=$(wildcard src/asm/*.S)
LIB=-l$(PROJECT_NAME) -lgcc
OUT=$(PROJECT_NAME).elf

#####
## QEMU
#####
QEMU=qemu-system-riscv32
MACH=virt
CPU=rv32
CPUS=1
MEM=16K
DRIVE=hdd.dsk

all:
	cargo build --release
	$(RISCV_GXX) $(CFLAGS) $(LINKER_SCRIPT) $(INCLUDES) -o $(OUT) $(SOURCES_ASM) $(LIBS) $(LIB)
	$(RISCV_OBJDUMP) -S --disassemble $(OUT) > $(PROJECT_NAME).list
	$(RISCV_SIZE) $(OUT)

run: all
	$(QEMU) -machine $(MACH) -cpu $(CPU) -smp $(CPUS) -m $(MEM) \
	-nographic -serial mon:stdio -bios none -kernel $(OUT) \
	-drive if=none,format=raw,file=$(DRIVE),id=foo -device virtio-blk-device,scsi=off,drive=foo

.PHONY: clean
clean:
	cargo clean
	rm -f $(OUT) $(PROJECT_NAME).map $(PROJECT_NAME).list
