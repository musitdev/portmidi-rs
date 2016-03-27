extern crate portmidi as pm;

extern crate rustc_serialize;
extern crate docopt;

use std::time::Duration;
use std::thread;

const USAGE: &'static str = r#"
portmidi-rs: monitor-device example

Usage:
    monitor-device <device-id>

Options:
    -h --help   Show this screen.

Omitting <device-id> will list the available devices.
"#;

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_device_id: i32,
}

fn print_devices(pm: &pm::PortMidi) {
    for dev in pm.devices().unwrap() {
        println!("{}", dev);
    }
}

fn main() {
    // initialize the PortMidi context.
    let context = pm::PortMidi::new().unwrap();
    let timeout = Duration::from_millis(10);

    // setup the command line interface
    let args: Args = docopt::Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|err| {
        print_devices(&context);
        err.exit();
    });

    // get the device info for the given id
    let info = context.device(args.arg_device_id).unwrap();
    println!("Listening on: {}) {}", info.id(), info.name());

    // get the device's input port
    let in_port = context.input_port(info, 1024).unwrap();

    while let Ok(_) = in_port.poll() {
        if let Ok(Some(event)) = in_port.read_n(1024) {
            println!("{:?}", event);
        }
        // there is no blocking receive method in PortMidi, therefore
        // we have to sleep some time to prevent a busy-wait loop
        thread::sleep(timeout);
    }
}
