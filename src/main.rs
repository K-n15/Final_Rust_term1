use std::collections::HashMap;

use iced::{alignment::Horizontal::Left, widget::{button, column, container, row,text}, Alignment::Center, Element, Length::Fill, Subscription};
use systemstat::ByteSize;

mod libraries;
mod log_handler;

fn main() -> iced::Result {
    iced::application("Status Manager", Status::update, Status::view)
    .subscription(Status::subscription)
    .window_size(iced::Size { width: 500.0, height: 500.0 } )
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
    RefreshAll
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
    log : HashMap<String,Vec<String>>
}


fn flush(value: &mut Status){
    value.cpu_usage = libraries::cpuusage::cpu_usage();
    value.memory_total = libraries::memory::memory_usage().1;
    value.memory_used = libraries::memory::memory_usage().0;
    value.disk_read = libraries::disk::disk_read().0;
    value.disk_write = libraries::disk::disk_write().0;
    value.tdisk_read = libraries::disk::disk_read().1;
    value.tdisk_write = libraries::disk::disk_write().1;
    value.network_receive = libraries::network::network_status().0;
    value.network_transmit = libraries::network::network_status().1;
    value.battery = libraries::battery::get_battery().remaining_capacity;
    value.uptime = libraries::uptime::get_uptime();
    value.boottime = libraries::boottime::boot_time();
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
            Computer::RefreshAll =>{flush(self);},
        }
    }

    fn subscription(&self) -> Subscription<Computer> {
        iced::time::every(iced::time::Duration::from_secs(10)).map(|_| Computer::RefreshAll)
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
            row![button("Refresh All").on_press(Computer::RefreshAll)].spacing(20),
        ].align_x(Left).width(Fill).height(Fill).spacing(10)
        ).align_x(Center).align_y(Center).into()
    }
}