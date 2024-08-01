TARGET := $(shell sed -n 's/^name = "\(.*\)"/\1/p' Cargo.toml)
PLATFORM := $(shell sed -n 's/^target = "\(.*\)"/\1/p' .cargo/config.toml)
DEVICE = STM32F401RE
MODE ?= release
# use command rust-nm -S target/thumbv7em-none-eabi/release/stm32f401_embassy | grep RTT ,to get the address of RTT
# the command's output is 20000000 00000030 D _SEGGER_RTT
RTT_ADDR := $(shell rust-nm -S target/$(PLATFORM)/$(MODE)/$(TARGET) 2>/dev/null | grep RTT | awk '{print $$1}')
RTT_SIZE := $(shell rust-nm -S target/$(PLATFORM)/$(MODE)/$(TARGET) 2>/dev/null | grep RTT | awk '{print $$2}')
PORT := 8765

FILE_ELF := target/$(PLATFORM)/$(MODE)/$(TARGET)
FILE_BIN := target/$(PLATFORM)/$(MODE)/$(TARGET).bin

OBJDUMP := rust-objdump --arch-name=arm
OBJCOPY := rust-objcopy --binary-architecture=arm

ENTRY_PA := 0x08000000

# depend on MODE to determine the mode arg
ifeq ($(MODE), release)
	MODE_ARG := --release
endif

build:
	cargo build $(MODE_ARG) 

bin: $(TARGET)
	@echo "Build done"

$(TARGET): build $(FILE_ELF)
	$(OBJCOPY) -O binary $(FILE_ELF) $(FILE_BIN)
debug: build bin
	tmux new-session -d \
	"openocd -f interface/stlink.cfg -f target/stm32f4x.cfg -c init -c "halt" -c 'flash write_image erase $(FILE_BIN) 0x8000000' " && \
	tmux split-window -h "RUST_GDB=/usr/bin/gdb-multiarch rust-gdb -ex 'file $(FILE_ELF)' -ex 'set arch arm' -ex 'target extended-remote localhost:3333' \
	-ex 'source ./.gdbinit' -ex 'monitor reset' -ex 'monitor rtt server start $(PORT) 0' -ex 'monitor rtt setup 0x$(RTT_ADDR) 0x$(RTT_SIZE) \"SEGGER RTT\" '  -ex 'monitor rtt start'  " && \
	tmux -2 attach-session -d

download: build bin
	openocd -f interface/stlink.cfg -f target/stm32f4x.cfg -c init -c "halt" -c "flash write_image erase $(FILE_BIN) 0x8000000" -c "reset" -c "shutdown"
Jdownload: build bin
	JLinkExe -device $(DEVICE) -autoconnect 1 -if SWD -speed 4000 -CommanderScript JLinkDownload.jlink
Jdebug: build bin Jdownload JGDBServer
	tmux new-session -d \
	"nc localhost 19021 | defmt-print -e $(FILE_ELF) " && \
	tmux split-window -h "RUST_GDB=/usr/bin/gdb-multiarch rust-gdb -ex 'file $(FILE_ELF)' -ex 'set arch arm' -ex 'target extended-remote localhost:2331' \
	-ex 'source ./.gdbinit' -ex 'monitor reset' " && \
	tmux -2 attach-session -d
# -ex 'monitor reset' -ex 'monitor reset' -ex 'monitor rtt server start $(PORT) 0' -ex 'monitor rtt setup 0x$(RTT_ADDR) 0x$(RTT_SIZE) \"SEGGER RTT\" '  -ex 'monitor rtt start'
JGDBServer:
	@if ! pgrep JLinkGDBServer > /dev/null; then \
        echo "启动 JLinkGDBServer..."; \
        JLinkGDBServer -device $(DEVICE) -if swd -speed 4000 & \
    else \
        echo "JLinkGDBServer 已经在运行。"; \
    fi

Jclien:
	nc localhost 19021 | defmt-print -e $(FILE_ELF) 
clean:
	cargo clean
run:
	clear
	cargo run $(MODE_ARG)
defmt:
	zsh -c "nc localhost $(PORT) | defmt-print -e $(FILE_ELF) "