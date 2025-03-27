use sysinfo::System;
use affinity;
use tauri::command;
#[command]
pub fn detect_and_set_affinity() -> Result<(), String> {
    // 获取系统信息
    let mut system = System::new_all();
    system.refresh_cpu_all();

    // 假设大核是那些具有较高性能指标的核心
    let mut large_cores = Vec::new();

    // 假设通过 CPU 频率来判定大核
    for (i, cpu) in system.cpus().iter().enumerate() {
        if cpu.frequency() > 2500 { // 比如假设频率大于 2.5 GHz 为大核心
            large_cores.push(i);
        }
    }

    // 如果没有找到大核心，则选择默认的 0b11（核心 0 和核心 1）
    let core_mask: Vec<usize> = if !large_cores.is_empty() {
        // 如果检测到大核心，选择第一个大核心
        let large_core = large_cores[0];
        println!("检测到大核：{}", large_core);
        vec![large_core] // 创建一个 Vec<usize>，只包含第一个大核心的索引
    } else {
        // 如果没有找到大核心，默认选择核心 0 和核心 1
        println!("未能检测到大核心，选择默认的核心 0 和 1");
        vec![0, 1] // 默认选择核心 0 和核心 1
    };

    // 设置当前线程亲和性
    match affinity::set_thread_affinity(&core_mask) {
        Ok(_) => {
            println!("线程已设置到大核心: {:?}", core_mask);
            Ok(())
        }
        Err(e) => Err(format!("设置线程亲和性失败: {}", e)),
    }
}
