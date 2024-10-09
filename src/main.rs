use iced::{alignment::Horizontal::Left, widget::{button, column, container, progress_bar, row,text}, Alignment::Center, Element, Length::Fill};
use systemstat::ByteSize;

pub mod infodump;

fn main() -> iced::Result {
    iced::application("Status Manager", update, view)
    .window_size(iced::Size { width: 700.0, height: 400.0 } )
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

fn update(value: &mut Status, message: Computer) {
    let info = infodump::Status::getinfo();
    match message {
        Computer::CpuUsage => value.cpu_usage = info.cpu_usage,
        Computer::MemoryUsed => {
            update(value, Computer::MemoryTotal);
            value.memory_used = info.memory_used
        },
        Computer::MemoryTotal => value.memory_total = info.memory_total,
        Computer::DiskRead => {
            update(value, Computer::TotalRead);
            value.disk_read = info.disk_read
        },
        Computer::DiskWrite => {
            update(value, Computer::TotalWrite);
            value.disk_write = info.disk_write
        },
        Computer::TotalRead => value.tdisk_read = info.tdisk_read,
        Computer::TotalWrite => value.tdisk_write = info.tdisk_write,
        Computer::NetReceive => value.network_receive = info.network_receive,
        Computer::NetTransmit => value.network_transmit = info.network_transmit,
        Computer::Battery => value.battery = info.battery.remaining_capacity,
        Computer::UpTime => value.uptime = info.uptime,
        Computer::BootTime => value.boottime = info.boottime,
    }
}

fn view(value: &Status) -> Element<Computer> {
    container(
    column![
        row![text(" ")],
        row![button("CPU usage").on_press(Computer::CpuUsage),
            text(format!("CPU status as mean : {}",value.cpu_usage))
        ].spacing(20),
        row![button("Memory").on_press(Computer::MemoryUsed),
            text(format!("Memory {}/{} Byte",value.memory_used,value.memory_total)),
            progress_bar(0.0..=value.memory_total.as_u64() as f32, value.memory_used.as_u64() as f32).width(100)
        ].spacing(20),
        row![button("Disk Input").on_press(Computer::DiskRead),
            text(format!("Disk Input {}/{}",value.disk_read,value.tdisk_read))
        ].spacing(20),
        row![button("Disk Output").on_press(Computer::DiskWrite),
            text(format!("Disk Output {}/{}",value.disk_write,value.tdisk_write))
        ].spacing(20),
        row![button("Network Receive").on_press(Computer::NetReceive),
            text(format!("Network Received {} B",value.network_receive))
        ].spacing(20),
        row![button("Network Transmit").on_press(Computer::NetTransmit),
            text(format!("Network Transmitted {} B",value.network_transmit))
        ].spacing(20),
        row![button("Battery").on_press(Computer::Battery),
            text(format!("Battery Status {}%",value.battery*100.0)),
            progress_bar(0.0..=100.0, value.battery*100.0).width(100)
        ].spacing(20),
        row![button("Refresh").on_press(Computer::BootTime),
            text(format!("System booted since {} ",time_convert(value.boottime)))
        ].spacing(20),
        row![button("Refresh").on_press(Computer::UpTime),
            text(format!("System running since {} ",time_convert(value.uptime)))
        ].spacing(20),
    ].align_x(Left).width(Fill).height(Fill).spacing(10)
    ).align_x(Center).align_y(Center).into()
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