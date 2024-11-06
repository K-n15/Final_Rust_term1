use systemstat::{BatteryLife, Platform};

pub fn get_battery()->BatteryLife{
    let new = systemstat::System::new();
    let battery = new.battery_life().unwrap();
    return battery
}