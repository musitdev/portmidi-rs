extern crate portmidi;

#[test]
fn test_main() {
    let context = portmidi::PortMidi::new().unwrap();
    assert!(context.device_count() > 0);
    assert!(context.default_input_device_id().is_ok());
    assert!(context.default_output_device_id().is_ok());
    assert!(context.devices().unwrap().len() > 0);
    let mut in_port = context.default_input_port(1024).unwrap();
    let mut out_port = context.default_output_port(1024).unwrap();
    assert!(in_port.poll() == Ok(false));
    assert!(in_port.read() == Ok(None));
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
    assert!(out_port.write_events(msgs).is_ok());
}
