use iced::{alignment::Horizontal::Left, widget::{button, column, container, progress_bar, row,text}, Alignment::Center, Element, Length::Fill};
use systemstat::ByteSize;

pub mod infodump;

fn main() -> iced::Result {
    iced::application("Status Manager", Status::update, Status::view)
    .window_size(iced::Size { width: 700.0, height: 500.0 } )
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
    uptime : u64,
    boottime : u64
}

fn flush(value: &mut Status, _message: Computer){
    Status::update(value, Computer::CpuUsage);
    Status::update(value, Computer::MemoryUsed);
    Status::update(value, Computer::MemoryTotal);
    Status::update(value, Computer::DiskRead);
    Status::update(value, Computer::DiskWrite);
    Status::update(value, Computer::TotalRead);
    Status::update(value, Computer::TotalWrite);
    Status::update(value, Computer::NetReceive);
    Status::update(value, Computer::NetTransmit);
    Status::update(value, Computer::Battery);
    Status::update(value, Computer::UpTime);
    Status::update(value, Computer::BootTime);
}

impl Status{
fn update(&mut self, message: Computer) {
    let info = infodump::Status::getinfo();
    match message {
        Computer::CpuUsage => self.cpu_usage = info.cpu_usage,
        Computer::MemoryUsed => {
            Status::update(self, Computer::MemoryTotal);
            self.memory_used = info.memory_used
        },
        Computer::MemoryTotal => self.memory_total = info.memory_total,
        Computer::DiskRead => {
            Status::update(self, Computer::TotalRead);
            self.disk_read = info.disk_read
        },
        Computer::DiskWrite => {
            Status::update(self, Computer::TotalWrite);
            self.disk_write = info.disk_write
        },
        Computer::TotalRead => self.tdisk_read = info.tdisk_read,
        Computer::TotalWrite => self.tdisk_write = info.tdisk_write,
        Computer::NetReceive => self.network_receive = info.network_receive,
        Computer::NetTransmit => self.network_transmit = info.network_transmit,
        Computer::Battery => self.battery = info.battery.remaining_capacity,
        Computer::UpTime => self.uptime = info.uptime,
        Computer::BootTime => self.boottime = info.boottime,
        Computer::RefreshAll =>flush(self, message),
    }
}

fn view(&self) -> Element<Computer> {
    container(
    column![
        row![text(" ")],
        row![button("Refresh").on_press(Computer::CpuUsage),
            text(format!("CPU status as mean : {}",self.cpu_usage))
        ].spacing(20),
        row![button("Refresh").on_press(Computer::MemoryUsed),
            text(format!("Memory {}/{} Byte",self.memory_used,self.memory_total)),
            progress_bar(0.0..=self.memory_total.as_u64() as f32, self.memory_used.as_u64() as f32).width(100)
        ].spacing(20),
        row![button("Refresh").on_press(Computer::DiskRead),
            text(format!("Disk Input {}/{}",self.disk_read,self.tdisk_read))
        ].spacing(20),
        row![button("Refresh").on_press(Computer::DiskWrite),
            text(format!("Disk Output {}/{}",self.disk_write,self.tdisk_write))
        ].spacing(20),
        row![button("Refresh").on_press(Computer::NetReceive),
            text(format!("Network Received {} B",self.network_receive))
        ].spacing(20),
        row![button("Refresh").on_press(Computer::NetTransmit),
            text(format!("Network Transmitted {} B",self.network_transmit))
        ].spacing(20),
        row![button("Refresh").on_press(Computer::Battery),
            text(format!("Battery Status {}%",self.battery*100.0)),
            progress_bar(0.0..=100.0, self.battery*100.0).width(100)
        ].spacing(20),
        row![button("Refresh").on_press(Computer::BootTime),
            text(format!("System booted since {} ",time_convert(self.boottime)))
        ].spacing(20),
        row![button("Refresh").on_press(Computer::UpTime),
            text(format!("System running since {} ",time_convert(self.uptime)))
        ].spacing(20),
        row![button("Refresh All").on_press(Computer::RefreshAll)].spacing(20),
    ].align_x(Left).width(Fill).height(Fill).spacing(10)
    ).align_x(Center).align_y(Center).into()
}
}
fn time_convert(x:u64)->String{
    if x >= 3600{
        format!("{} hours",x/3600)
    } else if x >=60 {
        format!("{} minutes",x/60)
    } else {
        format!("{} seconds",x)
    }
}