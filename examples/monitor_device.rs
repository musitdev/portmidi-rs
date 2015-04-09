#![feature(exit_status)]

extern crate portmidi as pm;

use std::thread::sleep_ms;

use pm::{PortMidiDeviceId, PortMidiResult};
use pm::PortMidiError::InvalidDeviceId;

pub mod common;

fn main() {
    // get the device number from the command line, or die()
    let device_id = match common::get_arg(1) {
        Some(id) => id,
        None => { common::die(); return; }
    };

    // run the monitor, print an error if it returns one
    if let Err(e) = monitor(device_id) {
        println!("Error: {:?}", e);
    }
}

fn monitor(device_id: PortMidiDeviceId) -> PortMidiResult<()> {
    // initialize portmidi
    try!(pm::initialize());

    // get the device and check it exists
    let device = try!(pm::get_device_info(device_id).ok_or(InvalidDeviceId));
    println!("Opening: {}", device.name);

    // open the input
    let mut input = pm::InputPort::new(device_id, 1024);
    try!(input.open());

    let qw = common::QuitWatcher::new();
    qw.start();

    while qw.is_running() {
        // read all the data in the buffer
        while let Some(event) = try!(input.read()) {
            // filter midi time stamps and active sensing
            if event.message.status != 248 && event.message.status != 254 {
                println!("{} {:?}", event.timestamp, event.message);
            }
        }

        // no more data? wait a little before trying again
        sleep_ms(50);
    }

    // close the input and terminate portmidi
    try!(input.close());
    pm::terminate()
}

