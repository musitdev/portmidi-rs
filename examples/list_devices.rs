#![feature(env, old_io)]

extern crate "portmidi" as pm;

pub mod common;
use common::{get_devices, print_devices};

fn main() {
    match get_devices() {
        Err(e) => println!("{:?}", e),
        Ok(d) => print_devices(d)
    }
}

