extern crate portmidi;

#[test]
fn test_midiin() {
    let result = portmidi::initialize();
    assert_eq!(result, Ok(()));

    let nbdevice = portmidi::count_devices();
    println!("portmidi nb device {:?}", nbdevice);
    let defdevin = portmidi::get_default_input_device_id().unwrap();
    println!("portmidi default input device {:?}", defdevin);
    let defdevout = portmidi::get_default_output_device_id().unwrap();
    println!("portmidi default output device {:?}", defdevout);

    let ininfo = portmidi::DeviceInfo::new(defdevin);
    println!("portmidi default input device info {:?}", ininfo);

    let outinfo = portmidi::DeviceInfo::new(defdevout);
    println!("portmidi default output device info {:?}", outinfo);

    let mut inport = portmidi::InputPort::new(defdevin, 0);
    let result = inport.open();
    assert_eq!(result, Ok(()));

    let mut outport = portmidi::OutputPort::new(defdevout, 100);
    let result = outport.open();
    assert_eq!(result, Ok(()));

    let read_midi = inport.read();
    println!("portmidi input note {:?}", read_midi);
    match read_midi {
        Ok(Some(notes)) => println!("portmidi read midi note {:?}", notes),
        Ok(None) => println!("portmidi read midi no note"),
        Err(err) => println!("portmidi read midi error {:?}", err),
    }

    let result = inport.poll();
    assert_eq!(result, Ok(false));

    // send note
    let note1 = portmidi::MidiEvent {
        message: portmidi::MidiMessage {
            status: 1 | 0x90, // chanell and note on
            data1: 36, // note number
            data2: 90, // velocity
        },
        timestamp: 0,
    };
    let result = outport.write_event(note1);
    assert_eq!(result, Ok(()));

    let note2 = portmidi::MidiMessage {
        status: 1 | 0x80, // chanell and note off
        data1: 36, // note number
        data2: 0, // velocity
    };
    let result = outport.write_message(note2);
    assert_eq!(result, Ok(()));

    // close out port
    let result = outport.close();
    assert_eq!(result, Ok(()));

    // close in port
    let result = inport.close();
    assert_eq!(result, Ok(()));

    // terminate midi
    let result = portmidi::terminate();
    assert_eq!(result, Ok(()));
}
