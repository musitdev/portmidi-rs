#![crate_name = "portmidi-example1"]
#![license = "MIT"]

extern crate portmidi;

#[main]
fn main() {
    let error = portmidi::initialize();
    println!("res {}", error);
}
