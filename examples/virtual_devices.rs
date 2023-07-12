extern crate portmidi as pm;

use pm::MidiMessage;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

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

fn main() {
    // initialize the PortMidi context.
    let context = pm::PortMidi::new().unwrap();
    let context = Arc::new(context);
    let timeout = Duration::from_millis(10);

    let v_in = context.create_virtual_input("Virt In 1").unwrap();
    let v_out = context.create_virtual_output("Virt Out 1").unwrap();

    let con2 = Arc::clone(&context);
    thread::spawn(move || {
        let out_port = con2
            .output_port(con2.device(v_out.id()).unwrap(), 1024)
            .unwrap();
        println!("Playing... Connect Virt Out 1 to Virt In 1 to see midi messages on screen...");
        println!("(Note: Windows not supported: midi devices do have to be implemented drivers)");
        println!("Press Crtl-C to abort...");
        play(out_port, false);
    });

    let in_port = context
        .input_port(context.device(v_in.id()).unwrap(), 1024)
        .unwrap();

    while let Ok(_) = in_port.poll() {
        if let Ok(Some(event)) = in_port.read_n(1024) {
            println!("{:?}", event);
        }
        // there is no blocking receive method in PortMidi, therefore
        // we have to sleep some time to prevent a busy-wait loop
        thread::sleep(timeout);
    }
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
