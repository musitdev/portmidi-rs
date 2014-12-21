extern crate portmidi;

#[test]
fn test_midiin() {
    let result = portmidi::initialize();
    assert_eq!(result, Ok(()));

    let nbdevice = portmidi::count_devices();
    println!("portmidi nb device {}", nbdevice);
    let defdevin = portmidi::get_default_input_device_id().unwrap();
    println!("portmidi default input device {}", defdevin);
    let defdevout = portmidi::get_default_output_device_id().unwrap();
    println!("portmidi default output device {}", defdevout);

    let ininfo = portmidi::get_device_info(defdevin);
    println!("portmidi default input device info {}", ininfo);

    let outinfo = portmidi::get_device_info(defdevout);
    println!("portmidi default output device info {}", outinfo);

    let mut inport = portmidi::InputPort::new(defdevin, 0);
    let result = inport.open();
    assert_eq!(result, Ok(()));

    let mut outport : portmidi::PmOutputPort = portmidi::PmOutputPort::new(defdevout, 100);
    let outerror = outport.open();
    println!("portmidi new output device {}", outerror);
    assert_eq!(outerror as int, portmidi::PmError::PmNoError as int);

    let read_midi = inport.read();
    println!("portmidi input note {}", read_midi);
    match read_midi    {
        Ok(Some(notes)) => println!("portmidi read midi note {}", notes),
        Ok(None) => println!("portmidi read midi no note"),
        Err(err) => println!("portmidi read midi error {}", err)
    }

    let result = inport.poll();
    assert_eq!(result, Ok(false));

    //send note
    let note1 = portmidi::MidiEvent {
        message : portmidi::MidiMessage {
            status : 1 | 0x90, //chanell and note on
            data1 : 36, //note number
            data2 : 90, // velocity
        },
        timestamp : 0
    };
    let sendnoteerr = outport.write_event(note1);
    assert_eq!(sendnoteerr as int, portmidi::PmError::PmNoError as int);

    let note2 = portmidi::MidiMessage {
        status : 1 | 0x80, //chanell and note off
        data1 : 36, //note number
        data2 : 0, // velocity
    };
    let sendnote2err = outport.write_message(note2);
    assert_eq!(sendnote2err as int, portmidi::PmError::PmNoError as int);

    //close out port
    let aborterr = outport.abort();
    assert_eq!(aborterr as int, portmidi::PmError::PmNoError as int);
    let outcloseerr = outport.close();
    assert_eq!(outcloseerr as int, portmidi::PmError::PmNoError as int);

    //close in port
    let result = inport.close();
    assert_eq!(result, Ok(()));

    //terminate midi
    let result = portmidi::terminate();
    assert_eq!(result, Ok(()));
}
