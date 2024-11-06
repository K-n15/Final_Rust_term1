use sysinfo::Networks;

pub fn network_status()->(u64,u64){
    let networks = Networks::new_with_refreshed_list();
    let mut total_receive = 0;
    let mut total_transmit = 0;
    for (_interface_name, data) in &networks {
        total_receive += data.total_received();
        total_transmit += data.total_transmitted();
    }
    total_transmit /= networks.len() as u64;
    total_receive /= networks.len() as u64;
    return (total_receive,total_transmit)
}