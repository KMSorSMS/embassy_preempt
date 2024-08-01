
OBJDUMP := rust-objdump --arch-name=arm
OBJCOPY := rust-objcopy --binary-architecture=arm
TEST_BINARY = target/thumbv7em-none-eabi/debug/integration.bin
ENTRY_PA := 0x08000000
PORT := 8765
TEST_ELF_NEW := target/thumbv7em-none-eabi/debug/integration
ifneq ($(filter bin-test debug-test,$(MAKECMDGOALS)),)
	TEST_ELF := $(shell cargo test --package ucosii --test integration --no-run 2>&1 | grep -oP 'Executable .* \(\K[^)]+') 
endif
RTT_ADDR := $(shell rust-nm -S $(TEST_ELF) 2>/dev/null | grep RTT | awk '{print $$1}')
RTT_SIZE := $(shell rust-nm -S $(TEST_ELF) 2>/dev/null | grep RTT | awk '{print $$2}')
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