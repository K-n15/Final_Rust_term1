use sysinfo::System;

pub fn boot_time()->String{
    let new = System::boot_time();
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