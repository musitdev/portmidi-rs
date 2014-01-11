#[crate_id = "portmidi#0.0.1"];
#[comment = "PortMidi binding for Rust"];
#[license = "MIT"];
#[crate_type = "lib"];

///  build : rustpkg build portmidi
///  test : rustpkg test portmidi

extern mod extra;


pub mod midi;
pub mod time;
pub mod util;


