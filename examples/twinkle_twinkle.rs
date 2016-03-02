extern crate portmidi as pm;

use std::thread::sleep_ms;

use pm::{MidiMessage, PortMidiDeviceId, Result};
use pm::PmError::PmInvalidDeviceId;

pub mod common;

static MIDI_CH: u8 = 0; // == midi channel 1

static MELODY: &'static [(u8, u32)] = &[(60, 1), (60, 1), (67, 1), (67, 1), (69, 1), (69, 1),
                                        (67, 2), (65, 1), (65, 1), (64, 1), (64, 1), (62, 1),
                                        (62, 1), (60, 2), (67, 1), (67, 1), (65, 1), (65, 1),
                                        (64, 1), (64, 1), (62, 2), (67, 1), (67, 1), (65, 1),
                                        (65, 1), (64, 1), (64, 1), (62, 2), (60, 1), (60, 1),
                                        (67, 1), (67, 1), (69, 1), (69, 1), (67, 2), (65, 1),
                                        (65, 1), (64, 1), (64, 1), (62, 1), (62, 1), (60, 2)];

fn main() {
    // get the device number from the command line, or die()
    let device_id = match common::get_arg(1) {
        Some(id) => id,
        None => {
            common::die();
            return;
        }
    };

    // run twinkle_twinkle, print an error if it returns one
    if let Err(e) = twinkle_twinkle(device_id) {
        println!("Error: {:?}", e);
    }
}

fn twinkle_twinkle(device_id: PortMidiDeviceId) -> Result<()> {
    // initialize portmidi
    try!(pm::initialize());

    // get the device and check it exists
    let device = try!(pm::DeviceInfo::new(device_id).ok_or(PmInvalidDeviceId));
    println!("Opening: {}", device.name);

    // open the output
    let mut output = pm::OutputPort::new(device_id, 1024);
    try!(output.open());

    let qw = common::QuitWatcher::new();
    qw.start();

    let mut iter = MELODY.iter().cycle();

    while qw.is_running() {
        if let Some(&(n, d)) = iter.next() {
            let dur: u32 = 400 * d;

            // send the note on
            try!(output.write_message(note_on(MIDI_CH, n)));
            sleep_ms(dur - 100);

            // send the note off
            try!(output.write_message(note_off(MIDI_CH, n)));
            sleep_ms(100);
        }
    }

    // close the input and terminate portmidi
    try!(output.close());
    pm::terminate()
}

fn note_on(channel: u8, note: u8) -> MidiMessage {
    let status = (9 & 0b00001111) * 16 + channel;
    MidiMessage {
        status: status,
        data1: note,
        data2: 100,
    }
}

fn note_off(channel: u8, note: u8) -> MidiMessage {
    let status = (8 & 0b00001111) * 16 + channel;
    MidiMessage {
        status: status,
        data1: note,
        data2: 0,
    }
}
