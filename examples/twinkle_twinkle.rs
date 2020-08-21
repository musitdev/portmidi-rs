extern crate portmidi as pm;

extern crate docopt;
extern crate rustc_serialize;

use std::thread;
use std::time::Duration;

use pm::MidiMessage;

static CHANNEL: u8 = 0;
static MELODY: [(u8, u32); 42] = [
    (60, 1),
    (60, 1),
    (67, 1),
    (67, 1),
    (69, 1),
    (69, 1),
    (67, 2),
    (65, 1),
    (65, 1),
    (64, 1),
    (64, 1),
    (62, 1),
    (62, 1),
    (60, 2),
    (67, 1),
    (67, 1),
    (65, 1),
    (65, 1),
    (64, 1),
    (64, 1),
    (62, 2),
    (67, 1),
    (67, 1),
    (65, 1),
    (65, 1),
    (64, 1),
    (64, 1),
    (62, 2),
    (60, 1),
    (60, 1),
    (67, 1),
    (67, 1),
    (69, 1),
    (69, 1),
    (67, 2),
    (65, 1),
    (65, 1),
    (64, 1),
    (64, 1),
    (62, 1),
    (62, 1),
    (60, 2),
];

const USAGE: &'static str = r#"
portmidi-rs: play-twinkle-twinkle

Usage:
    play [-v | --verbose] <device-id>

Options:
    -h --help       Show this screen.
    -v --verbose    Print what's being done

Omitting <device-id> will list the available devices.
"#;

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_device_id: i32,
    flag_verbose: bool,
}

fn print_devices(pm: &pm::PortMidi) {
    for dev in pm.devices().unwrap() {
        println!("{}", dev);
    }
}

fn main() {
    // initialize the PortMidi context.
    let context = pm::PortMidi::new().unwrap();

    // setup the command line interface
    let args: Args = docopt::Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|err| {
            print_devices(&context);
            err.exit();
        });

    let out_port = context
        .device(args.arg_device_id)
        .and_then(|dev| context.output_port(dev, 1024))
        .unwrap();
    play(out_port, args.flag_verbose).unwrap()
}

fn play(mut out_port: pm::OutputPort, verbose: bool) -> pm::Result<()> {
    for &(note, dur) in MELODY.iter().cycle() {
        let note_on = MidiMessage {
            status: 0x90 + CHANNEL,
            data1: note,
            data2: 100,
            data3: 0,
        };
        if verbose {
            println!("{}", note_on)
        }
        out_port.write_message(note_on)?;
        // note hold time before sending note off
        thread::sleep(Duration::from_millis(dur as u64 * 400));

        let note_off = MidiMessage {
            status: 0x80 + CHANNEL,
            data1: note,
            data2: 100,
            data3: 0,
        };
        if verbose {
            println!("{}", note_off);
        }
        out_port.write_message(note_off)?;
        // short pause
        thread::sleep(Duration::from_millis(100));
    }
    Ok(())
}
