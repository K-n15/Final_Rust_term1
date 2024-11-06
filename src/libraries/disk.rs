use sysinfo::System;

pub fn disk_read()->(u64,u64){
    let info = System::new_all();
    let mut total_read = 0;
    let mut read = 0;
    for (_pid, process) in info.processes() {
        let disk_usage = process.disk_usage();
        total_read += disk_usage.total_read_bytes;
        read += disk_usage.read_bytes;
    }
    total_read /= info.processes().len() as u64;
    read /= info.processes().len() as u64;
    return (read,total_read);
}
pub fn disk_write()->(u64,u64){
    let info = System::new_all();
    let mut total_write = 0;
    let mut write = 0;
    for (_pid, process) in info.processes() {
        let disk_usage = process.disk_usage();
        total_write += disk_usage.total_written_bytes;
        write += disk_usage.written_bytes;
    }
    total_write /= info.processes().len() as u64;
    write /= info.processes().len() as u64;
    return (write,total_write);
}