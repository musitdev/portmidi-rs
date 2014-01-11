#[crate_id = "portmidiex1#0.0.1"];
#[license = "MIT"];

#[feature(globs)];

extern mod portmidi;

use portmidi::midi;

#[main]
fn main() {
    println("hello?");
    let error:midi::PmError = midi::initialize();
}
