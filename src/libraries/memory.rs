use systemstat::{saturating_sub_bytes, ByteSize, Platform};
use std::io::Error;

fn display_error(e:Error)->!{
    panic!("Error : {}",e)
}

pub fn memory_usage()->(ByteSize,ByteSize){
    let new = systemstat::System::new();
    let memo;
    match new.memory(){
        Ok(k)=> memo = k,
        Err(e)=> display_error(e),
    }
    let memo_used = saturating_sub_bytes(memo.total, memo.free);
    let memo_total = memo.total;
    return (memo_used,memo_total);
}