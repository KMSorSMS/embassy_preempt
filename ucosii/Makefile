# TARGET := $(shell sed -n 's/^name = "\(.*\)"/\1/p' Cargo.toml)
# PLATFORM := $(shell sed -n 's/^target = "\(.*\)"/\1/p' .cargo/config.toml)
# DEVICE = STM32F401RE
# MODE ?= release
# # use command rust-nm -S target/thumbv7em-none-eabi/release/stm32f401_embassy | grep RTT ,to get the address of RTT
# # the command's output is 20000000 00000030 D _SEGGER_RTT
# RTT_ADDR := $(shell rust-nm -S target/$(PLATFORM)/$(MODE)/$(TARGET) 2>/dev/null | grep RTT | awk '{print $$1}')
# RTT_SIZE := $(shell rust-nm -S target/$(PLATFORM)/$(MODE)/$(TARGET) 2>/dev/null | grep RTT | awk '{print $$2}')
# PORT := 8765

# FILE_ELF := target/$(PLATFORM)/$(MODE)/$(TARGET)
# FILE_BIN := target/$(PLATFORM)/$(MODE)/$(TARGET).bin

# OBJDUMP := rust-objdump --arch-name=arm
# OBJCOPY := rust-objcopy --binary-architecture=arm

# ENTRY_PA := 0x08000000

# # depend on MODE to determine the mode arg
# ifeq ($(MODE), release)
# 	MODE_ARG := --release
# endif

# build:
# 	cargo build $(MODE_ARG) 

# bin: $(TARGET)
# 	@echo "Build done"

# $(TARGET): build $(FILE_ELF)
# 	$(OBJCOPY) -O binary $(FILE_ELF) $(FILE_BIN)
# debug: build bin
# 	tmux new-session -d \
# 	"openocd -f interface/stlink.cfg -f target/stm32f4x.cfg -c init -c "halt" -c 'flash write_image erase $(FILE_BIN) 0x8000000' " && \
# 	tmux split-window -h "RUST_GDB=/usr/bin/gdb-multiarch rust-gdb -ex 'file $(FILE_ELF)' -ex 'set arch arm' -ex 'target extended-remote localhost:3333' \
# 	-ex 'source ./.gdbinit' -ex 'monitor reset' -ex 'monitor rtt server start $(PORT) 0' -ex 'monitor rtt setup 0x$(RTT_ADDR) 0x$(RTT_SIZE) \"SEGGER RTT\" '  -ex 'monitor rtt start'  " && \
# 	tmux -2 attach-session -d

# download: build bin
# 	openocd -f interface/stlink.cfg -f target/stm32f4x.cfg -c init -c "halt" -c "flash write_image erase $(FILE_BIN) 0x8000000" -c "reset" -c "shutdown"
# Jdownload: build bin
# 	JLinkExe -device $(DEVICE) -autoconnect 1 -if SWD -speed 4000 -CommanderScript JLinkDownload.jlink
# Jdebug: build bin Jdownload JGDBServer
# 	tmux new-session -d \
# 	"nc localhost 19021 | defmt-print -e $(FILE_ELF) " && \
# 	tmux split-window -h "RUST_GDB=/usr/bin/gdb-multiarch rust-gdb -ex 'file $(FILE_ELF)' -ex 'set arch arm' -ex 'target extended-remote localhost:2331' \
# 	-ex 'source ./.gdbinit' -ex 'monitor reset' " && \
# 	tmux -2 attach-session -d
# # -ex 'monitor reset' -ex 'monitor reset' -ex 'monitor rtt server start $(PORT) 0' -ex 'monitor rtt setup 0x$(RTT_ADDR) 0x$(RTT_SIZE) \"SEGGER RTT\" '  -ex 'monitor rtt start'
# JGDBServer:
# 	@if ! pgrep JLinkGDBServer > /dev/null; then \
#         echo "启动 JLinkGDBServer..."; \
#         JLinkGDBServer -device $(DEVICE) -if swd -speed 4000 & \
#     else \
#         echo "JLinkGDBServer 已经在运行。"; \
#     fi

# Jclien:
# 	nc localhost 19021 | defmt-print -e $(FILE_ELF) 
# clean:
# 	cargo clean
# run:
# 	clear
# 	cargo run $(MODE_ARG)
# defmt:
# 	zsh -c "nc localhost $(PORT) | defmt-print -e $(FILE_ELF) "

OBJDUMP := rust-objdump --arch-name=arm
OBJCOPY := rust-objcopy --binary-architecture=arm
TEST_BINARY := target/thumbv7em-none-eabi/debug/integration.bin
TEST_BINARY_LIB := target/thumbv7em-none-eabi/debug/ucosii_lib.bin 
ENTRY_PA := 0x08000000
PORT := 8765
TEST_ELF_NEW := target/thumbv7em-none-eabi/debug/integration
TEST_ELF_NEW_LIB := target/thumbv7em-none-eabi/debug/ucosii_lib
ifneq ($(filter bin-test debug-test bin-lib debug-lib,$(MAKECMDGOALS)),)
	TEST_ELF := $(shell cargo test --package ucosii --test integration --no-run 2>&1 | grep -oP 'Executable .* \(\K[^)]+') 
endif
ifneq ($(filter bin-lib debug-lib,$(MAKECMDGOALS)),)
	TEST_ELF_LIB := $(shell cargo test --lib --no-run  2>&1 | grep -oP 'Executable .* \(\K[^)]+') 
endif
RTT_ADDR := $(shell rust-nm -S $(TEST_ELF) 2>/dev/null | grep RTT | awk '{print $$1}')
RTT_SIZE := $(shell rust-nm -S $(TEST_ELF) 2>/dev/null | grep RTT | awk '{print $$2}')
RTT_ADDR_LIB := $(shell rust-nm -S $(TEST_ELF_LIB) 2>/dev/null | grep RTT | awk '{print $$1}')
RTT_SIZE_LIB := $(shell rust-nm -S $(TEST_ELF_LIB) 2>/dev/null | grep RTT | awk '{print $$2}')

bin-test:
# 将编译的elf文件转为二进制文件
	$(OBJCOPY) -O binary $(TEST_ELF) $(TEST_BINARY)
	$(OBJCOPY) $(TEST_ELF) $(TEST_ELF_NEW)
debug-test: bin-test
	tmux new-session -d \
	"openocd -f interface/stlink.cfg -f target/stm32f4x.cfg -c init -c "halt" -c 'flash write_image erase $(TEST_BINARY) $(ENTRY_PA)' " && \
	tmux split-window -h "RUST_GDB=/usr/bin/gdb-multiarch rust-gdb -ex 'file $(TEST_ELF)' -ex 'set arch arm' -ex 'target extended-remote localhost:3333' \
	-ex 'source ./bp.gdb' -ex 'source ./.gdbinit'  -ex 'monitor reset' -ex 'monitor rtt server start $(PORT) 0' -ex 'monitor rtt setup 0x$(RTT_ADDR) 0x$(RTT_SIZE) \"SEGGER RTT\" '  -ex 'monitor rtt start'  " && \
	tmux -2 attach-session -d
defmt-test:
	zsh -c "nc localhost $(PORT) | defmt-print -e $(TEST_ELF_NEW) "
test-run:
	cargo test --package ucosii --test integration --features test
bin-lib:
	$(OBJCOPY) -O binary $(TEST_ELF_LIB) $(TEST_BINARY_LIB)
	$(OBJCOPY) $(TEST_ELF_LIB) $(TEST_ELF_NEW_LIB)
debug-lib: bin-lib
	tmux new-session -d \
	"openocd -f interface/stlink.cfg -f target/stm32f4x.cfg -c init -c "halt" -c 'flash write_image erase $(TEST_BINARY_LIB) $(ENTRY_PA)' " && \
	tmux split-window -h "RUST_GDB=/usr/bin/gdb-multiarch rust-gdb -ex 'file $(TEST_ELF_LIB)' -ex 'set arch arm' -ex 'target extended-remote localhost:3333' \
	-ex 'source ./bp.gdb' -ex 'source ./.gdbinit'  -ex 'monitor reset' -ex 'monitor rtt server start $(PORT) 0' -ex 'monitor rtt setup 0x$(RTT_ADDR_LIB) 0x$(RTT_SIZE_LIB) \"SEGGER RTT\" '  -ex 'monitor rtt start'  " && \
	tmux -2 attach-session -d
