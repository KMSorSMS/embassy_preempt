# 定义源文件目录和目标文件目录
TARGET = ffi_test
.PHONY: bear clean
UBUNTU_VERSION := $(shell lsb_release -rs)
DEVICE := STM32F401RE
DEBUG = 1
OPT = -O3
# Build path
BUILD_DIR = build
C_SOURCES = $(wildcard src/*.c)

ASM_SOURCES = \
# startup_stm32f401xe.s

# 从openocd里获取interface路径，interface在openocd的安装的父目录的父目录的tcl目录下
# OPENOCD_PATH = $(shell which openocd)
# OPENOCD_TCL_PATH = $(shell dirname $(shell dirname $(OPENOCD_PATH)))/tcl

# ports/os_cpu_a.S 

#######################################
# binaries
#######################################
PREFIX = arm-none-eabi-
# The gcc compiler bin path can be either defined in make command via GCC_PATH variable (> make GCC_PATH=xxx)
# either it can be added to the PATH environment variable.
CC = arm-none-eabi-gcc
AS = arm-none-eabi-gcc -x assembler-with-cpp
CP = arm-none-eabi-objcopy
SZ = arm-none-eabi-size
HEX = $(CP) -O ihex
BIN = $(CP) -O binary -S

# mcu
MCU = -mcpu=cortex-m4 -mthumb -mfpu=fpv4-sp-d16 -mfloat-abi=soft

# macros for gcc
# AS defines
AS_DEFS = 

# C defines
C_DEFS =  \


# AS includes
AS_INCLUDES = \


# C includes
C_INCLUDES =  \
-Iinc \

# compile gcc flags
ASFLAGS = $(MCU) $(AS_DEFS) $(AS_INCLUDES) $(OPT) -Wall -fdata-sections -ffunction-sections

CFLAGS += $(MCU) $(C_DEFS) $(C_INCLUDES) $(OPT) -Wall -fdata-sections -ffunction-sections

ifeq ($(DEBUG), 1)
CFLAGS += -g -gdwarf-2
endif


# Generate dependency information
CFLAGS += -MMD -MP -MF"$(@:%.o=%.d)"


#######################################
# LDFLAGS
#######################################
# link script
LDSCRIPT = STM32F401RETx_FLASH.ld

# libraries
LIBS = -lc -lm -lnosys -lembassy_preempt
LIBDIR = -L../../embassy_preempt/target/thumbv7em-none-eabi/release/
LDFLAGS = $(MCU) -specs=nano.specs -T$(LDSCRIPT) $(LIBDIR) $(LIBS) -Wl,-Map=$(BUILD_DIR)/$(TARGET).map,--cref -Wl,--gc-sections

# default action: build all
all: $(BUILD_DIR)/$(TARGET).elf $(BUILD_DIR)/$(TARGET).hex $(BUILD_DIR)/$(TARGET).bin


#######################################
# build the application
#######################################
# list of objects
OBJECTS = $(addprefix $(BUILD_DIR)/,$(notdir $(C_SOURCES:.c=.o)))
vpath %.c $(sort $(dir $(C_SOURCES)))
# list of ASM program objects
temp = $(ASM_SOURCES:.S=.o)
OBJECTS += $(addprefix $(BUILD_DIR)/,$(notdir $(temp:.s=.o)))

vpath %.s $(sort $(dir $(ASM_SOURCES)))
vpath %.S $(sort $(dir $(ASM_SOURCES)))


$(BUILD_DIR)/%.o: %.c Makefile STM32F401RETx_FLASH.ld | $(BUILD_DIR) 
	$(CC) -c $(CFLAGS) -Wa,-a,-ad,-alms=$(BUILD_DIR)/$(notdir $(<:.c=.lst)) $< -o $@

$(BUILD_DIR)/%.o: %.s Makefile STM32F401RETx_FLASH.ld | $(BUILD_DIR)
	$(AS) -c $(ASFLAGS) $< -o $@

$(BUILD_DIR)/%.o: %.S Makefile STM32F401RETx_FLASH.ld | $(BUILD_DIR)
	$(AS) -c $(ASFLAGS) $< -o $@

$(BUILD_DIR)/$(TARGET).elf: $(OBJECTS) Makefile STM32F401RETx_FLASH.ld
	$(CC) $(OBJECTS) $(LDFLAGS) -o $@
	$(SZ) $@

$(BUILD_DIR)/%.hex: $(BUILD_DIR)/%.elf | $(BUILD_DIR)
	$(HEX) $< $@
	
$(BUILD_DIR)/%.bin: $(BUILD_DIR)/%.elf | $(BUILD_DIR)
	$(BIN) $< $@	
	
$(BUILD_DIR):
	mkdir $@		

# bear: clean
# 	rm ./compile_commands.json || true 
# 	bear make -j4
bear: clean
	@echo UBUNTU_VERSION:${UBUNTU_VERSION}
ifeq ($(UBUNTU_VERSION),20.04)
	rm ./compile_commands.json || true 
	bear make -j4
else ifeq ($(UBUNTU_VERSION),22.04)

	rm ./compile_commands.json || true 
	export https_proxy= && export http_proxy= && bear -- make -j4
else ifeq ($(UBUNTU_VERSION),24.04)
	rm ./compile_commands.json || true 
	export https_proxy= && export http_proxy= && bear -- make -j4
endif
Jdownload: bear
# 生成脚本
	@echo "h\nloadfile $(BUILD_DIR)/$(TARGET).bin 0x08000000\nr\ng\nexit" > JLinkDownload.jlink
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
	"RUST_GDB=/usr/bin/gdb-multiarch rust-gdb -ex 'file $(BUILD_DIR)/$(TARGET).elf' -ex 'set arch arm' -ex 'target extended-remote localhost:2331' \
	-ex 'dashboard -layout assembly !breakpoints !expressions !history memory registers source !stack !threads variables' \
	-ex 'monitor reset'" && \
	tmux -2 attach-session -d
STdownload:
	make
	openocd -f interface/stlink.cfg -f target/stm32f4x.cfg -c init -c "halt" -c "flash write_image erase $(BUILD_DIR)/$(TARGET).bin 0x8000000" -c "reset" -c "shutdown"

liam_debug: clean all
	tmux new-session -d \
	"openocd -f interface/stlink.cfg -f target/stm32f4x.cfg -c init -c "halt" -c 'flash write_image erase $(BUILD_DIR)/$(TARGET).bin 0x8000000' " && \
	tmux split-window -h "gdb-multiarch -ex 'file $(BUILD_DIR)/$(TARGET).elf' -ex 'set arch arm' -ex 'target extended-remote localhost:3333' \
	-ex 'source ./bp.gdb' -ex 'source ./.gdbinit' -ex 'monitor reset'" && \
	tmux -2 attach-session -d

clean:
	rm -rf $(BUILD_DIR)