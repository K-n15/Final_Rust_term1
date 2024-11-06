use sysinfo::System;

pub fn get_uptime()->String{
    let new = System::uptime();
    return time_convert(new);
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