# 定义源文件目录和目标文件目录
SRC_DIR := .
OBJ_DIR := build
# 查找所有的 .c 文件
C_FILES := $(wildcard $(SRC_DIR)/*.c)
# 将 .c 文件转换为 elf 文件
OBJ_FILES := $(patsubst $(SRC_DIR)/%.c, $(OBJ_DIR)/%, $(C_FILES))
# 编译器和编译选项
CC = arm-none-eabi-gcc
OPT = -O0
# mcu
MCU = -mcpu=cortex-m4 -mthumb -mfpu=fpv4-sp-d16 -mfloat-abi=soft
CFLAGS += $(MCU) $(OPT) -Wall -fdata-sections -ffunction-sections
LIB := ../ucosii/target/thumbv7em-none-eabi/release/libucosii.a
LIBS = -lc -lm -lnosys -llibucosii
LIBDIR = -L../ucosii
TARGET = ffi_test
# link script
LDSCRIPT = STM32F401RETx_FLASH.ld
LDFLAGS = $(MCU) -specs=nano.specs -T$(LDSCRIPT) $(LIBDIR) $(LIBS) -Wl,-Map=$(OBJ_DIR)/$(TARGET).map,--cref -Wl,--gc-sections

clean:
	rm -rf $(OBJ_DIR)

build:$(OBJ_FILES)

$(OBJ_DIR)/%: $(SRC_DIR)/%.c
	@mkdir -p $(OBJ_DIR)
	$(CC) $(CFLAGS) $< $(LIB) -o $@

# # 每一个c文件都需要链接uc库
# $(OBJ_DIR)/%: $(OBJ_DIR)/%.o
# 	$(CC) $< $(LDFLAGS) -o $@

bin: $(TARGET)
	@echo "Build done"

$(TARGET): build $(FILE_ELF)
	$(OBJCOPY) -O binary $(FILE_ELF) $(FILE_BIN)

Jdownload: build bin
# 生成脚本
	@echo "h\nloadfile $(FILE_BIN) 0x08000000\nr\ng\nexit" > JLinkDownload.jlink
	JLinkExe -device $(DEVICE) -autoconnect 1 -if SWD -speed 4000 -CommanderScript JLinkDownload.jlink

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

Jdebug: Jdownload JGDBServer
	tmux new-session -d \
	"nc localhost 19021 | defmt-print -e $(FILE_ELF) " && \
	tmux split-window -h "RUST_GDB=/usr/bin/gdb-multiarch rust-gdb -ex 'file $(FILE_ELF)' -ex 'set arch arm' -ex 'target extended-remote localhost:2331' \
	-ex 'dashboard -layout assembly !breakpoints !expressions !history memory registers source !stack !threads variables' \
	-ex 'source ./.gdbinit' -ex 'monitor reset' -ex 'source ./bp.gdb'" && \
	tmux -2 attach-session -d
clean:
	rm -rf $(OBJ_DIR)