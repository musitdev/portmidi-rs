extern crate portmidi as pm;

extern crate rustc_serialize;
extern crate docopt;

const USAGE: &'static str = r#"
portmidi-rs: monitor-device example

Usage:
    monitor-device <id>

Options:
    -h --help   Show this screen.

Omitting <id> will list the available devices.
"#;

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_id: u32,
}

fn print_devices(pm: &pm::PortMidi) {
    for dev in pm.devices().unwrap() {
        println!("{}", dev);
    }
}

fn main() {
    let context = pm::PortMidi::new().unwrap();
    let args: Args = docopt::Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|err| {
        print_devices(&context);
        err.exit();
    });
    let info = context.device(args.arg_id as i32).unwrap();
    println!("Listening on: {}) {}", info.id(), info.name());

    let mut in_port = context.input_port(info).unwrap();
    while let Ok(_) = in_port.poll() {
        if let Ok(Some(event)) = in_port.read() {
            println!("{:?}", event);
        }
    }
}
