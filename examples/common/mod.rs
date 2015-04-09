use std::env;
use std::io::stdin;
use std::str;
use std::sync::{Arc, RwLock};
use std::thread;

use pm;

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

pub fn die() {
    println!("Please supply a device number:");
    println!("");
    match get_devices() {
        Err(e) => println!("Error: {:?}", e),
        Ok(d) => print_devices(d)
    }
    env::set_exit_status(1);
}

pub fn get_arg<T:str::FromStr>(index: usize) -> Option<T> {
    let mut args = env::args();
    args.nth(index).and_then(|s| s.parse().ok())
}

pub struct QuitWatcher(Arc<RwLock<bool>>);

impl QuitWatcher {
    pub fn new() -> QuitWatcher {
        QuitWatcher(Arc::new(RwLock::new(false)))
    }

    pub fn start(&self)  {
        let quit_lock = self.0.clone();
        thread::spawn(move || {
            println!("Press enter to quit");
            // read_line will block until enter is pressed
            stdin().read_line(&mut String::new()).ok().expect("Failed to read line");
            let mut quit = quit_lock.write().unwrap();
            *quit = true;
        });
    }

    pub fn is_running(&self) -> bool {
        let quit = self.0.read().unwrap();
        !*quit
    }
}

