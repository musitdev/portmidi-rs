extern crate portmidi;
extern crate rci;

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
                        },
                        portmidi::MidiMessage {
                            status: 0x80,
                            data1: 60,
                            data2: 0,
                        }];
        match out_port.write_events(msgs) {
            Ok(_) => println!("test_main) successfully wrote midi events"),
            Err(err) => println!("test_main) write error: {}", err),
        }
    }
}
