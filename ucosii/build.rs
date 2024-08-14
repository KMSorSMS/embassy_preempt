use std::env;

fn main() {
    // get the value of the environment variable "OS_MAX_MEM_PART"
    let os_max_mem_part: i32 = env::var("OS_MAX_MEM_PART")
        .unwrap_or("0".to_string())
        .parse() // 尝试将字符串解析为i32类型
        .unwrap_or(0); // 如果解析失败，使用0作为默认值
                       // if os_max_mem_part is bigger than 0, then add the feature "OS_MAX_MEM_PART_EN" to the compilation
                       // println!("cargo:warning=Debug message: the value of OS_MAX_MEM_PART is {}", os_max_mem_part);
    if os_max_mem_part > 0 {
        // println!("cargo:warning=Debug message: the value of OS_MAX_MEM_PART is {}", os_max_mem_part);
        println!("cargo:rustc-cfg=feature=\"OS_MAX_MEM_PART_EN\"");
    }
    // about feature OS_EVENT_EN
    let os_q_en: i32 = env::var("OS_MAX_MEM_PART")
        .unwrap_or("0".to_string())
        .parse() // 尝试将字符串解析为i32类型
        .unwrap_or(0); // 如果解析失败，使用0作为默认值
    let os_max_qs: i32 = env::var("OS_MAX_MEM_PART")
        .unwrap_or("0".to_string())
        .parse() // 尝试将字符串解析为i32类型
        .unwrap_or(0); // 如果解析失败，使用0作为默认值
    let os_mbox_en: i32 = env::var("OS_MAX_MEM_PART")
        .unwrap_or("0".to_string())
        .parse() // 尝试将字符串解析为i32类型
        .unwrap_or(0); // 如果解析失败，使用0作为默认值
    let os_sem_en: i32 = env::var("OS_MAX_MEM_PART")
        .unwrap_or("0".to_string())
        .parse() // 尝试将字符串解析为i32类型
        .unwrap_or(0); // 如果解析失败，使用0作为默认值
    let os_mutex_en: i32 = env::var("OS_MAX_MEM_PART")
        .unwrap_or("0".to_string())
        .parse() // 尝试将字符串解析为i32类型
        .unwrap_or(0); // 如果解析失败，使用0作为默认值
    if (os_q_en == 1 && os_max_qs == 1) || os_mbox_en == 1 || os_sem_en == 1 || os_mutex_en == 1 {
        println!("cargo:rustc-cfg=feature=\"OS_EVENT_EN\"");
    }
    // about feature OS_EVENT_NAME_EN
    let os_event_name_en: i32 = env::var("OS_MAX_MEM_PART")
        .unwrap_or("0".to_string())
        .parse() // 尝试将字符串解析为i32类型
        .unwrap_or(0); // 如果解析失败，使用0作为默认值
    if os_event_name_en == 1 {
        println!("cargo:rustc-cfg=feature=\"OS_EVENT_NAME_EN\"");
    }
}
