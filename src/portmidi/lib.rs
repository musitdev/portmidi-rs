#[crate_id = "github.com/musitdev/rust-portmidi#portmidi:0.1"];
#[comment = "PortMidi binding for Rust"];
#[license = "MIT"];
#[crate_type = "lib"];
#[crate_type = "dylib"];

///  build : rustpkg build portmidi
///  test : rustpkg test portmidi

//extern mod extra;
extern crate extra;  //= "extra#0.10-pre"
extern crate serialize;

pub mod midi;
pub mod time;
pub mod util;


