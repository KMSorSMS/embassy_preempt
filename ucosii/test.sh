#!/bin/bash
# 测试有两个Executor 4个Task的情况
# 先运行测试：cargo run --bin test_2e_4t,将结果重定向到tmp文件，再从里面选取出执行时间
# 比如task_1 execute time:{}ms这种信息
# cargo run --bin test_2e_4t > tmp.yaml --release
# cat tmp | grep "task_1 execute time"
# cargo run --bin test_2e_4t --release > tmp.yaml
# 在后台运行cargo命令
run_test() {
    local test_name=$1
    rm tmp.yaml
    cargo run --bin "$test_name" --release > tmp.yaml &
    # 记录上一次文件大小
    PREV_SIZE=0

    # 检查文件大小的时间间隔（秒）
    INTERVAL=2

    # 获取cargo命令的PID
    CARGO_PID=$!
    echo "Cargo PID: $CARGO_PID"
    # 最大等待时间（秒）
    MAX_WAIT=3
    WAITED=0
    # 单次测试最长执行时间
    MAX_TIME=90
    # 记录总时间
    TOTAL_TIME=0
    # # 查找并终止probe-rs run进程
    # sleep 2
    # PROBE_RS_PID=$(pgrep -f "probe-rs")
    # echo "Probe-rs PID: $PROBE_RS_PID"
    while true; do
    # 获取当前文件大小
    CURRENT_SIZE=$(stat -c %s tmp.yaml 2>/dev/null || echo "0")
        # 只有当前文件大小非0才监测
    if [ "$CURRENT_SIZE" -ne 0 ]; then
        # 检查文件大小是否有变化
        if [ "$CURRENT_SIZE" -eq "$PREV_SIZE" ] || [ "$TOTAL_TIME" -ge "$MAX_TIME" ]; then
            if [ "$WAITED" -ge "$MAX_WAIT" ] || [ "$TOTAL_TIME" -ge "$MAX_TIME" ]; then
            # 文件大小在指定时间内没有变化，终止cargo命令
            kill $CARGO_PID
            echo "Cargo command terminated due to inactivity."
            break
            else
            # 等待更长时间
            ((WAITED+=INTERVAL))
            fi
        else
            # 文件大小有变化，重置等待时间
            WAITED=0
        fi
        PREV_SIZE=$CURRENT_SIZE
        ((TOTAL_TIME+=INTERVAL))
    fi
    sleep $INTERVAL
    done

    # 继续执行脚本的其他部分
    # 得到程序执行时间的信息task[0-9]+ *counted *execute *times:
    if [[ $test_name == *thread ]]; then
        cat tmp.yaml | grep -E "((=)*task((_[0-9]+)*|[0-9]+) execute time(=)*)|((=)*thread[0-9]+ is scheduled(=)*)" 
    else
        cat tmp.yaml | grep -E "task(_[0-9]+)* *execute *time"
    fi
    # #直接打印协程调度信息
    # awk '
    # /INFO  task[0-9]+ *execute *time:/ {
    #     # 使用正则表达式匹配task名称和执行次数
    #     match($0, /(task[0-9]+) *execute *time/, arr)
    #     printf("===========%s is polled===========\n",arr[1])
    # }' tmp.yaml
    # #直接打印线程调度信息
    # awk '
    # /INFO  thread[0-9]+ *is *scheduled/ {
    #     match($0, /(thread[0-9]+) *is *scheduled/, arr)
    #     # 输出线程被调度的信息
    #     printf("==========%s is scheduled===========\n",arr[1])
    # }' tmp.yaml
    awk '
    /INFO  task(_[0-9]+)* *execute *time/ {
        # 使用正则表达式匹配task名称和执行次数
        match($0, /(task(_[0-9]+)*) *execute *time: *([0-9]+)/, arr)
        # 使用task名称作为键,存储执行次数
        tasks[arr[1]] = arr[3]
    }
    END {
        # 遍历数组,打印每个task的执行时间
        for (task in tasks) {
            printf("%-10s ",task) 
        }
        # 换行
        printf("\n") 
        for (task in tasks) {
            printf("%-10s ",tasks[task])
        }
        # # 换行
        # printf("\n") 
    }' tmp.yaml
    # 需要捕获各个task最新打印的task(_[0-9]+)* counted execute times:
    # 需要捕获各个task最新的打印，也就是先按照task名字进行分组，然后选取各个组里面最后一个
    awk '
    /INFO  task(_[0-9]+)* *counted *execute *times:/ {
        # 使用正则表达式匹配task名称和执行次数
        match($0, /(task(_[0-9]+)*) *counted *execute *times: *([0-9]+)/, arr)
        # 使用task名称作为键,存储执行次数
        tasks[arr[1]] = arr[3]
    }
    END {
        # 遍历数组,打印每个task的最后一次执行次数
        for (task in tasks) {
            print "INFO  " task " counted execute times: " tasks[task]
        }
         # 遍历数组,打印每个task的最后一次执行次数
        for (task in tasks) {
            printf("%-10s ", task)
        }
        # 换行
        printf("\n") 
        for (task in tasks) {
            printf("%-10s ",tasks[task])
        }
        # 换行
        printf("\n") 
    }' tmp.yaml
}
clear
echo "=============Start testing=============" > record.yml

# 定义一个数组，包含所有测试
tests=(
"test_2e_4t" 
"test_3e_6t" 
"test_2e_8t" 
"test_2e_20t"
"test_3e_18t"
"test_2e_6t_thread"
)

# 循环遍历数组，执行测试
for test in "${tests[@]}"; do
    echo "=============${test}=============" >> record.yml
    run_test "$test" >> record.yml
    echo -e "=============${test} done=============\n" >> record.yml
    sleep 1
done