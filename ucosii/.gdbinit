# monitor reset
# b *0x08000000
# b main
# b main.rs:62
# b main.rs:65
# b uc_thread/os_core.rs:47
# b uc_thread/os_task.rs:26
# b uc_thread/os_core.rs:146
# b uc_thread/os_core.rs:57
# b uc_thread/os_cpu.rs:106
# b uc_thread/os_cpu.rs:44
# b uc_thread/os_cpu.rs:66
# ignore $bpnum 13
dashboard -layout assembly !breakpoints !expressions !history memory registers source !stack !threads variables
# b test_basic_schedule
# b ucosii/src/os_core.rs:336
b ucosii/src/bin/ucosii_main.rs:18
# b ucosii/src/os_core.rs:345
# b ucosii/src/os_core.rs:339
# b ucosii/src/bin/ucosii_main.rs:38
# b ucosii/src/bin/ucosii_main.rs:39
# b ucosii/src/bin/ucosii_main.rs:46
# b ucosii/src/bin/ucosii_main.rs:55
b ucosii/src/bin/ucosii_main.rs:66
# can be reached
b ucosii/src/bin/ucosii_main.rs:57
# b ucosii/src/os_task.rs:78
b ucosii/src/os_task.rs:80
# dashboard -layout assembly !breakpoints !expressions !history memory registers source !stack !threads variables
# b test_basic_schedule
# b ucosii/src/os_core.rs:337
# b ucosii/src/os_core.rs:345
# # b ucosii/src/os_core.rs:347
# b task1
# b task2
# b task3
# b task4



start

define dss
  dashboard source -output /dev/pts/$arg0
  dashboard source -style height 0
end

define dsa
  dashboard assembly -output /dev/pts/$arg0
  dashboard assembly -style height 0
end

define sbp
  save breakpoints bp.gdb
end