use pm;

use std::sync::{Arc, RwLock};

pub type QuitLock = Arc<RwLock<bool>>;

pub fn get_devices() -> pm::PortMidiResult<Vec<pm::DeviceInfo>> {
    try!(pm::initialize());
    let no = pm::count_devices();
    // use filter_map to discard None, and unwrap the Some(_)
    let devices = (0..no).filter_map(|i| pm::get_device_info(i))
                         .collect::<Vec<_>>();
    try!(pm::terminate());
    Ok(devices)
}

pub fn print_devices(devices: Vec<pm::DeviceInfo>) {
    println!("Id  Name                 Input? Output?");
    println!("=======================================");
    for d in devices.into_iter() {
        println!("{:<3} {:<20} {:<6} {:<6}", d.device_id, d.name, d.input, d.output);
    }
}
