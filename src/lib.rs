#![crate_name = "portmidi"]
#![comment = "PortMidi binding for Rust"]
#![license = "MIT"]
#![crate_type = "lib"]
#![crate_type = "dylib"]

extern crate libc;
extern crate core;
extern crate serialize;

#[allow(non_camel_case_types)]
pub mod midi;
#[allow(non_snake_case_functions)]
pub mod time;
#[allow(non_camel_case_types)]
pub mod util;


