OBJDUMP := rust-objdump --arch-name=arm
OBJCOPY := rust-objcopy --binary-architecture=arm
TEST_BINARY := target/thumbv7em-none-eabi/debug/integration.bin
TEST_BINARY_LIB := target/thumbv7em-none-eabi/debug/ucosii_lib.bin 
ENTRY_PA := 0x08000000
PORT := 8765
TEST_ELF_NEW := target/thumbv7em-none-eabi/debug/integration
# space_performance
# time_performance 
# sync_space_performance
# sync_time_performance
TARGET := sync_time_performance
liam_TARGET := time_performance
PLATFORM := $(shell sed -n 's/^target = "\(.*\)"/\1/p' .cargo/config.toml)
DEVICE = STM32F401RE
MODE ?= release
FILE_ELF := target/$(PLATFORM)/$(MODE)/$(TARGET)
FILE_BIN := target/$(PLATFORM)/$(MODE)/$(TARGET).bin

liam_FILE_ELF := target/$(PLATFORM)/$(MODE)/$(liam_TARGET)
liam_FILE_BIN := target/$(PLATFORM)/$(MODE)/$(liam_TARGET).bin

# 测试文件大小，使用方式为SZ + ELF文件
SZ = arm-none-eabi-size

ifeq ($(MODE), release)
	MODE_ARG := --release
endif

# ifneq ($(filter bin-test debug-test,$(MAKECMDGOALS)),)
TEST_ELF_NEW_LIB := target/thumbv7em-none-eabi/debug/ucosii_lib
ifneq ($(filter bin-test debug-test bin-lib debug-lib,$(MAKECMDGOALS)),)
	TEST_ELF := $(shell cargo test --package ucosii --test integration --no-run 2>&1 | grep -oP 'Executable .* \(\K[^)]+') 
endif
ifneq ($(filter bin-lib debug-lib,$(MAKECMDGOALS)),)
	TEST_ELF_LIB := $(shell cargo test --lib --no-run  2>&1 | grep -oP 'Executable .* \(\K[^)]+') 
endif
RTT_ADDR := $(shell rust-nm -S $(TEST_ELF) 2>/dev/null | grep RTT | awk '{print $$1}')
RTT_SIZE := $(shell rust-nm -S $(TEST_ELF) 2>/dev/null | grep RTT | awk '{print $$2}')

liam_RTT_ADDR := $(shell rust-nm -S $(liam_FILE_ELF) 2>/dev/null | grep RTT | awk '{print $$1}')
liam_RTT_SIZE := $(shell rust-nm -S $(liam_FILE_ELF) 2>/dev/null | grep RTT | awk '{print $$2}')

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

build:$(OBJ_FILES)
	cargo build $(MODE_ARG)
	$(SZ) $(FILE_ELF)
	objdump -h $(FILE_ELF)
# --bin $(TARGET)
# --features "defmt"

liam_build_release:
	cargo build --release
liam_build_debug:
	cargo build --features "defmt"
bin: $(TARGET)
	@echo "Build done"
liam_bin_debug: liam_TARGET_DEBUG
	@echo "liam debug bin build done"
liam_bin_release: liam_TARGET_RELEASE
	@echo "liam release bin build done"

$(TARGET): build $(FILE_ELF)
	$(OBJCOPY) -O binary $(FILE_ELF) $(FILE_BIN)
liam_TARGET_DEBUG: liam_build_debug $(liam_FILE_ELF)
	$(OBJCOPY) -O binary $(liam_FILE_ELF) $(liam_FILE_BIN)
liam_TARGET_RELEASE: liam_build_release $(liam_FILE_ELF)
	$(OBJCOPY) -O binary $(liam_FILE_ELF) $(liam_FILE_BIN)
liam_debug: liam_build_debug liam_bin_debug
	tmux new-session -d \
	"openocd -f interface/stlink.cfg -f target/stm32f4x.cfg -c 'adapter speed 4000' -c init -c "halt" -c 'flash write_image erase $(liam_FILE_BIN) 0x8000000' " && \
	tmux split-window -h "RUST_GDB=/usr/bin/gdb-multiarch rust-gdb -ex 'file $(liam_FILE_ELF)' -ex 'set arch arm' -ex 'target extended-remote localhost:3333' \
	-ex 'set remotetimeout 5' -ex 'source ./bp.gdb' -ex 'source ./.gdbinit' -ex 'monitor reset' -ex 'monitor rtt server start $(PORT) 0' -ex 'monitor rtt setup 0x$(liam_RTT_ADDR) 0x$(liam_RTT_SIZE) \"SEGGER RTT\" '  -ex 'monitor rtt start'  " && \
	tmux -2 attach-session -d
liam_debug_release: liam_build_release liam_bin_release
	tmux new-session -d \
	"openocd -f interface/stlink.cfg -f target/stm32f4x.cfg -c 'adapter speed 4000' -c init -c "halt" -c 'flash write_image erase $(liam_FILE_BIN) 0x8000000' " && \
	tmux split-window -h "RUST_GDB=/usr/bin/gdb-multiarch rust-gdb -ex 'file $(liam_FILE_ELF)' -ex 'set arch arm' -ex 'target extended-remote localhost:3333' \
	-ex 'set remotetimeout 5' -ex 'source ./bp.gdb' -ex 'source ./.gdbinit' " && \
	tmux -2 attach-session -d
liam_download_debug: liam_build_debug liam_bin_debug
	openocd -f interface/stlink.cfg -f target/stm32f4x.cfg -c init -c "halt" -c "flash write_image erase $(liam_FILE_BIN) 0x8000000" -c "reset" -c "shutdown"
liam_download_release: liam_build_release liam_bin_release
	openocd -f interface/stlink.cfg -f target/stm32f4x.cfg -c init -c "halt" -c "flash write_image erase $(liam_FILE_BIN) 0x8000000" -c "reset" -c "shutdown"

download: build bin
	openocd -f interface/stlink.cfg -f target/stm32f4x.cfg -c init -c "halt" -c "flash write_image erase $(FILE_BIN) 0x8000000" -c "reset" -c "shutdown"
Jdownload: build bin
# 生成脚本
	@echo "h\nerase 0x08000000 0x0807FFFF\nloadfile $(FILE_BIN) 0x08000000\nr\ng\nexit" > JLinkDownload.jlink
	JLinkExe -device $(DEVICE) -autoconnect 1 -if SWD -speed 4000 -CommanderScript JLinkDownload.jlink
Jdebug: Jdownload JGDBServer
	tmux new-session -d \
	"nc localhost 19021 | defmt-print -e $(FILE_ELF) " && \
	tmux split-window -h "RUST_GDB=/usr/bin/gdb-multiarch rust-gdb -ex 'file $(FILE_ELF)' -ex 'set arch arm' -ex 'target extended-remote localhost:2331' \
	-ex 'dashboard -layout assembly !breakpoints !expressions !history memory registers source !stack !threads variables' \
	-ex 'source ./.gdbinit' -ex 'monitor reset' -ex 'source ./bp.gdb'" && \
	tmux -2 attach-session -d
JGDBServer:kill
	@echo "启动 JLinkGDBServer...";
	@JLinkGDBServer -device $(DEVICE) -if swd -speed 4000 > /dev/null & 
	@sleep 1;
kill:
	@PID=$$(lsof -t -i:2331); \
    if [ -n "$$PID" ]; then \
        echo "Killing process on port 2331 with PID: $$PID"; \
        kill -9 $$PID; \
    else \
        echo "No process found on port 2331"; \
    fi
Jrun:Jdownload JGDBServer
	nc localhost 19021 | defmt-print -e $(FILE_ELF)
# lib:
# 	cargo build --lib $(MODE_ARG)
clean:kill
	cargo clean
run:
	clear
	cargo run $(MODE_ARG)
defmt:
	zsh -c "nc localhost $(PORT) | defmt-print -e $(FILE_ELF) "
bin-lib:
	$(OBJCOPY) -O binary $(TEST_ELF_LIB) $(TEST_BINARY_LIB)
	$(OBJCOPY) $(TEST_ELF_LIB) $(TEST_ELF_NEW_LIB)
debug-lib: bin-lib
	tmux new-session -d \
	"openocd -f interface/stlink.cfg -f target/stm32f4x.cfg -c init -c "halt" -c 'flash write_image erase $(TEST_BINARY_LIB) $(ENTRY_PA)' " && \
	tmux split-window -h "RUST_GDB=/usr/bin/gdb-multiarch rust-gdb -ex 'file $(TEST_ELF_LIB)' -ex 'set arch arm' -ex 'target extended-remote localhost:3333' \
	-ex 'source ./bp.gdb' -ex 'source ./.gdbinit'  -ex 'monitor reset' -ex 'monitor rtt server start $(PORT) 0' -ex 'monitor rtt setup 0x$(RTT_ADDR_LIB) 0x$(RTT_SIZE_LIB) \"SEGGER RTT\" '  -ex 'monitor rtt start'  " && \
	tmux -2 attach-session -d
defmt-lib:
	zsh -c "nc localhost $(PORT) | defmt-print -e $(TEST_ELF_NEW_LIB) "
liam_defmt:
	zsh -c "nc localhost $(PORT) | defmt-print -e $(liam_FILE_ELF) "
liam_run:
	cargo rb $(liam_TARGET) 
liam_run_release:
	cargo rrb $(liam_TARGET) 