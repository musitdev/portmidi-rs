extern crate portmidi;
extern crate rci;

use std::sync::Arc;
use std::thread;

#[test]
fn test_main() {
    let ci = rci::Ci::new();
    // Check if the test runs in a continous integration environment.
    // The test will fail in Travis CI because the test runner has no
    // permissions to access `/dev/snd/seq`:
    //
    // ```sh
    // running 1 test
    //
    // ALSA lib seq_hw.c:457:(snd_seq_hw_open) open /dev/snd/seq failed: Permission denied
    //
    // test test_main ... FAILED
    // ```
    if ci.is_none() {
        let context = portmidi::PortMidi::new().unwrap();
        assert!(context.device_count() > 0);
        assert!(context.default_input_device_id().is_ok());
        assert!(context.default_output_device_id().is_ok());
        assert!(context.devices().unwrap().len() > 0);

	// creating virtual ports on windows not possible that way (only through drivers)
	if ! cfg!(windows) {
            assert_eq!(context.virtual_device_count(), 0);
            let v_in = context.create_virtual_input("Virt in").unwrap();
            context.create_virtual_output("Virt out").unwrap();
            assert_eq!(context.virtual_device_count(), 2);
	    context.delete_virtual_device(v_in.id()).unwrap();
            assert_eq!(context.virtual_device_count(), 1);
        }

        let mut in_port = context.default_input_port(1024).unwrap();
        let mut out_port = context.default_output_port(1024).unwrap();
        match in_port.poll() {
            Ok(flag) => println!("test_main) midi events available: {}", flag),
            Err(err) => println!("test_main) poll error: {}", err),
        };
        match in_port.read() {
            Ok(Some(event)) => println!("received midi event: {:?}", event),
            Ok(None) => println!("test_main) no midi event available"),
            Err(err) => println!("test_main) read error: {}", err),
        };
        let msgs = vec![portmidi::MidiMessage {
                            status: 0x90,
                            data1: 60,
                            data2: 127,
                            data3: 0,
                        },
                        portmidi::MidiMessage {
                            status: 0x80,
                            data1: 60,
                            data2: 0,
                            data3: 0,
                        }];
        match out_port.write_events(msgs) {
            Ok(_) => println!("test_main) successfully wrote midi events"),
            Err(err) => println!("test_main) write error: {}", err),
        }
    }
}

#[test]
fn test_threads() {
    let ci = rci::Ci::new();
    const BUF_LEN: usize = 1024;
    if ci.is_none() && false {
        let context = portmidi::PortMidi::new().unwrap();
        let context = Arc::new(context);
        let reader = thread::spawn({
            let context = context.clone();
            move || {
                let mut in_port = context.default_input_port(BUF_LEN).unwrap();
                match in_port.poll() {
                    Ok(flag) => println!("test_threads) midi events available: {}", flag),
                    Err(err) => println!("test_threads) poll error: {}", err),
                }
                match in_port.read() {
                    Ok(Some(event)) => println!("test_threads) received midi event: {:?}", event),
                    Ok(None) => println!("test_threads) no midi event available"),
                    Err(err) => println!("test_threads) read error: {}", err),
                }
            }
        });
        let writer = thread::spawn(move || {
            let mut out_port = context.default_output_port(BUF_LEN).unwrap();
            let msgs = vec![portmidi::MidiMessage {
                                status: 0x90,
                                data1: 60,
                                data2: 127,
                                data3: 0,
                            },
                            portmidi::MidiMessage {
                                status: 0x80,
                                data1: 60,
                                data2: 0,
                                data3: 0,
                            }];
            match out_port.write_events(msgs) {
                Ok(_) => println!("test_threads) successfully wrote midi events"),
                Err(err) => println!("test_threads) write error: {}", err),
            }
        });
        reader.join().unwrap();
        writer.join().unwrap();
    }
}

#[test]
fn test_types() {
    let message = portmidi::MidiMessage::from(0x007F3C81);
    // NoteOn
    assert_eq!(message.status, 0x81);
    // Key: Note 60
    assert_eq!(message.data1, 60);
    // Velocity: 127
    assert_eq!(message.data2, 127);
}
