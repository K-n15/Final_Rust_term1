use std::collections::HashMap;
use iced::{alignment::Horizontal::Left, widget::{button, column, container, row,text}, Alignment::Center, Element, Length::Fill, Subscription};
use systemstat::ByteSize;

mod libraries;
mod log_handler;

fn main() -> iced::Result {
    iced::application("Status Manager", Status::update, Status::view)
    .subscription(Status::subscription)
    .window_size(iced::Size { width: 400.0, height: 420.0 } )
    .run()
}

#[derive(Debug, Clone)]
enum Computer {
    CpuUsage,
    MemoryUsed,
    MemoryTotal,
    DiskRead,
    DiskWrite,
    TotalRead,
    TotalWrite,
    NetReceive,
    NetTransmit,
    Battery,
    UpTime,
    BootTime,
    RefreshAll,
    FirstOperation,
}

#[derive(Default)]
struct Status {
    cpu_usage : f32,
    memory_used : ByteSize,
    memory_total : ByteSize,
    disk_read : u64,
    disk_write : u64,
    tdisk_read : u64,
    tdisk_write : u64,
    network_receive : u64,
    network_transmit : u64,
    battery : f32,
    uptime : String,
    boottime : String,
    last_update : f32,
    dictionary : HashMap<String,Vec<String>>,
    first_operation : bool,
}

fn flush(value: &mut Status){
    value.cpu_usage = libraries::cpuusage::cpu_usage();
    apply(value, String::from("CPU"), value.cpu_usage.to_string());
    value.memory_total = libraries::memory::memory_usage().1;
    apply(value, String::from("MEMORY_TOTAL"), value.memory_total.to_string());
    value.memory_used = libraries::memory::memory_usage().0;
    apply(value, String::from("MEMORY_USED"), value.memory_used.to_string());
    value.disk_read = libraries::disk::disk_read().0;
    apply(value, String::from("DISK_READ"), value.disk_read.to_string());
    value.disk_write = libraries::disk::disk_write().0;
    apply(value, String::from("DISK_WRITE"), value.disk_write.to_string());
    value.tdisk_read = libraries::disk::disk_read().1;
    apply(value, String::from("TOTAL_DISK_READ"), value.tdisk_read.to_string());
    value.tdisk_write = libraries::disk::disk_write().1;
    apply(value, String::from("TOTAL_DISK_WRITE"), value.tdisk_write.to_string());
    value.network_receive = libraries::network::network_status().0;
    apply(value, String::from("NETWORK_RECEIVED"), value.network_receive.to_string());
    value.network_transmit = libraries::network::network_status().1;
    apply(value, String::from("NETWORK_TRANSMIT"), value.network_transmit.to_string());
    value.battery = libraries::battery::get_battery().remaining_capacity;
    apply(value, String::from("BATTERY"), value.battery.to_string());
    value.uptime = libraries::uptime::get_uptime();
    apply(value, String::from("UPTIME"), value.uptime.to_string());
    value.boottime = libraries::boottime::boot_time();
    apply(value, String::from("BOOTTIME"), value.boottime.to_string());
}

fn apply(value: &mut Status, message: String,element : String){
    value.dictionary.entry(message).or_insert(vec![]).push(element);
}

impl Status{
    fn update(&mut self, message: Computer) {
        self.last_update = iced::time::Instant::now().elapsed().as_secs_f32();
        match message {
            Computer::CpuUsage => self.cpu_usage = libraries::cpuusage::cpu_usage(),
            Computer::MemoryUsed => {
                Status::update(self, Computer::MemoryTotal);
                self.memory_used = libraries::memory::memory_usage().0
            },
            Computer::MemoryTotal => self.memory_total = libraries::memory::memory_usage().1,
            Computer::DiskRead => {
                Status::update(self, Computer::TotalRead);
                self.disk_read = libraries::disk::disk_read().0
            },
            Computer::DiskWrite => {
                Status::update(self, Computer::TotalWrite);
                self.disk_write = libraries::disk::disk_write().0
            },
            Computer::TotalRead => self.tdisk_read = libraries::disk::disk_read().1,
            Computer::TotalWrite => self.tdisk_write = libraries::disk::disk_write().1,
            Computer::NetReceive => self.network_receive = libraries::network::network_status().0,
            Computer::NetTransmit => self.network_transmit = libraries::network::network_status().1,
            Computer::Battery => self.battery = libraries::battery::get_battery().remaining_capacity,
            Computer::UpTime => self.uptime = libraries::uptime::get_uptime(),
            Computer::BootTime => self.boottime = libraries::boottime::boot_time(),
            Computer::RefreshAll =>{
                flush(self);
                log_handler::record_log(self.dictionary.clone());
            },
            Computer::FirstOperation => {
                self.first_operation = true;
                let new = log_handler::open_log().expect("Failed to find file");
                let temp = log_handler::readrecord_file(new);
                self.dictionary = temp;
            },
        }
    }

    fn subscription(&self) -> Subscription<Computer> {
        match self.first_operation{
            false => {
                iced::time::every(iced::time::Duration::from_secs(10)).map(|_| Computer::FirstOperation)
            },
            true => {
                iced::time::every(iced::time::Duration::from_secs(10)).map(|_| Computer::RefreshAll)
            },
        }
    }

    fn view(&self) -> Element<Computer> {
        container(
        column![
            row![text(" ")],
            row![button("Refresh").on_press(Computer::CpuUsage),
                text(format!("CPU status : {}%",self.cpu_usage))
            ].spacing(20),
            row![button("Refresh").on_press(Computer::MemoryUsed),
                text(format!("Memory : {}/{} Byte",self.memory_used,self.memory_total))
            ].spacing(20),
            row![button("Refresh").on_press(Computer::DiskRead),
                text(format!("Disk Input : {}/{}",self.disk_read,self.tdisk_read))
            ].spacing(20),
            row![button("Refresh").on_press(Computer::DiskWrite),
                text(format!("Disk Output : {}/{}",self.disk_write,self.tdisk_write))
            ].spacing(20),
            row![button("Refresh").on_press(Computer::NetReceive),
                text(format!("Network Received : {} B",self.network_receive))
            ].spacing(20),
            row![button("Refresh").on_press(Computer::NetTransmit),
                text(format!("Network Transmitted : {} B",self.network_transmit))
            ].spacing(20),
            row![button("Refresh").on_press(Computer::Battery),
                text(format!("Battery Status : {}%",self.battery*100.0)),
            ].spacing(20),
            row![button("Refresh").on_press(Computer::BootTime),
                text(format!("System booted since : {} ",self.boottime))
            ].spacing(20),
            row![button("Refresh").on_press(Computer::UpTime),
                text(format!("System running since : {} ",self.uptime))
            ].spacing(20),
        ].align_x(Left).width(Fill).height(Fill).spacing(10)
        ).align_x(Center).align_y(Center).into()
    }
}