TARGET = time_performance
.PHONY: bear clean

UBUNTU_VERSION := $(shell lsb_release -rs)

# 根据 TEST 变量的值设置 EXCLUDE_FILE 变量
ifeq ($(TARGET), space_performance)
    EXCLUDE_FILE = time_performance
else ifeq ($(TARGET), time_performance)
    EXCLUDE_FILE = space_performance
endif

DEBUG = 1
OPT = -O0
# Build path
BUILD_DIR = build
C_SOURCES = $(shell find app/src/ -name '*.c' ! -name '$(EXCLUDE_FILE).c')  # 添加 app/src 目录下的所有 .c 文件
C_SOURCES += $(wildcard app/HARDWARE/*.c)  # 添加 app/HARDWARE/*.c 中的所有 .c 文件
# C_SOURCES += $(shell find Trace -name '*.c') # 添加 Trace 目录下的所有 .c 文件
C_SOURCES += Source/ucos_ii.c \
ports/os_cpu_c.c \

ASM_SOURCES = startup_stm32f401xe.s
ASM_SOURCES +=  \
ports/os_cpu_a.S \
# SEGGER/SEGGER_RTT_ASM_ARMv7M.S \

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
-DSTM32F401xE


# AS includes
AS_INCLUDES = \
# -ITrace/ThirdPartyLib/Config \


# C includes
C_INCLUDES =  \
-ISource \
-ICfg \
-Iports \
-Iapp/inc \
-Iapp/HARDWARE \

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
LIBS = -lc -lm -lnosys
LIBDIR = 
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

#######################################
# clean up
#######################################
clean:
	-rm -fR $(BUILD_DIR)
  
#######################################
# dependencies
#######################################
-include $(wildcard $(BUILD_DIR)/*.d)


# bear -o build/compile_commands.json make -j4
# arm-none-eabi-gdb -x init.gdb
# 需要在当前终端中关闭代理才能使用bear。这样在执行make bear的时候，会暂时关闭代理
bear: clean
	@echo UBUNTU_VERSION:${UBUNTU_VERSION}
ifeq ($(UBUNTU_VERSION),20.04)
	rm ./compile_commands.json || true 
	bear make -j4
else ifeq ($(UBUNTU_VERSION),22.04)
	rm ./compile_commands.json || true 
	export https_proxy= && export http_proxy= && bear -- make -j4
else  ifeq ($(UBUNTU_VERSION),24.04)
	rm ./compile_commands.json || true 
	export https_proxy= && export http_proxy= && bear -- make -j4
endif
# JLinkExe -device STM32F401RE -autoconnect 1 -if SWD -speed 4000 -CommanderScript JLinkScript.jlink
debug:
#	make bear
# 需要下载tmux
	tmux has-session -t test0 2>/dev/null && tmux kill-session -t test0 || true
	tmux new-session -d -s test0 'openocd -f interface/stlink.cfg -f target/stm32f4x.cfg -c init -c "halt" -c "flash write_image erase build/learn_startup.bin 0x8000000" -c reset'
# 这里看你电脑的性能,因为gdb-server服务器要启动一会儿
	sleep 5
	gdb-multiarch -x .gdbinit
format:
	find . -iname *.h -o -iname *.c | xargs clang-format -i
STdownload:
	make bear
	openocd -f interface/stlink.cfg -f target/stm32f4x.cfg -c init -c "halt" -c "flash write_image erase build/learn_startup.bin 0x8000000" -c "reset" -c "shutdown"
Jdownload:
	make bear
# 生成脚本
	@echo "h\nloadfile $(FILE_BIN) 0x08000000\nr\ng\nexit" > JLinkDownload.jlink
	openocd -f interface/jlink.cfg -f target/stm32f4x.cfg -c init -c "halt" -c "flash write_image erase build/learn_startup.bin 0x8000000" -c "reset" -c "shutdown"
pico:
	picocom --omap crcrlf --imap lfcrlf -c -b 115200 /dev/ttyACM0