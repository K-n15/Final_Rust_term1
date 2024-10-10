use sysinfo::{Networks, System,};
use systemstat::{saturating_sub_bytes, BatteryLife, ByteSize, Platform};

pub struct Status {
    pub cpu_usage : f32,
    pub memory_used : ByteSize,
    pub memory_total : ByteSize,
    pub disk_read : u64,
    pub disk_write : u64,
    pub tdisk_read : u64,
    pub tdisk_write : u64,
    pub network_receive : u64,
    pub network_transmit : u64,
    pub battery : BatteryLife,
    pub uptime : u64,
    pub boottime : u64,
}

impl Status{
    pub fn getinfo()->Status{
        let mut info = System::new_all();
        info.refresh_all();
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        let total_cpu = info.global_cpu_usage();

        let mut total_read = 0;
        let mut total_write = 0;
        let mut read = 0;
        let mut write = 0;
        for (_pid, process) in info.processes() {
            let disk_usage = process.disk_usage();
            total_read += disk_usage.total_read_bytes;
            read += disk_usage.read_bytes;
            total_write += disk_usage.total_written_bytes;
            write += disk_usage.written_bytes;
        }
        total_read /= info.processes().len() as u64;
        total_write /= info.processes().len() as u64;
        read /= info.processes().len() as u64;
        write /= info.processes().len() as u64;

        let networks = Networks::new_with_refreshed_list();
        let mut total_receive = 0;
        let mut total_transmit = 0;
        for (_interface_name, data) in &networks {
            total_receive += data.total_received();
            total_transmit += data.total_transmitted();
        }
        total_transmit /= networks.len() as u64;
        total_receive /= networks.len() as u64;

        let new = systemstat::System::new();
        let battery_life = new.battery_life().unwrap();

        let uptime = System::uptime();
        let boottime = System::boot_time();

        let memo = new.memory().unwrap();

        let sum = Status {
            cpu_usage : total_cpu,
            memory_used : saturating_sub_bytes(memo.total, memo.free),
            memory_total : memo.total,
            disk_read :read,
            disk_write : write,
            tdisk_read : total_read,
            tdisk_write : total_write,
            network_receive : total_receive,
            network_transmit : total_transmit,
            battery : battery_life,
            uptime : uptime,
            boottime : boottime,
        };
        sum
    }
}
