use sysinfo::System;

pub fn cpu_usage()->f32{
    let mut info = System::new_all();
    info.refresh_all();
    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    let total_cpu = info.global_cpu_usage();
    return total_cpu;
}