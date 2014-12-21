extern crate portmidi;

#[test]
fn test_midiin() {
    portmidi::initialize().ok();

    let nbdevice = portmidi::count_devices();
    println!("portmidi nb device {}", nbdevice);
    let defdevin = portmidi::get_default_input_device_id();
    println!("portmidi default input device {}", defdevin);
    let defdevout = portmidi::get_default_output_device_id();
    println!("portmidi default output device {}", defdevout);

    let ininfo = portmidi::get_device_info(defdevin);
    println!("portmidi default input device info {}", ininfo);

    let outinfo = portmidi::get_device_info(defdevout);
    println!("portmidi default output device info {}", outinfo);

    let mut inport : portmidi::PmInputPort = portmidi::PmInputPort::new(defdevin, 0);
    let inerror = inport.open();
    assert_eq!(inerror as int, portmidi::PmError::PmNoError as int);

    let mut outport : portmidi::PmOutputPort = portmidi::PmOutputPort::new(defdevout, 100);
    let outerror = outport.open();
    println!("portmidi new output device {}", outerror);
    assert_eq!(outerror as int, portmidi::PmError::PmNoError as int);

    let read_midi = inport.read();
    println!("portmidi input note {}", read_midi);
    match read_midi    {
        Ok(notes) => println!("portmidi read midi note {}", notes),
        Err(portmidi::PmError::PmNoError) => println!("portmidi read midi no note {}", portmidi::PmError::PmNoError),
        Err(err) => println!("portmidi read midi error {}", err)
    }

    let innote = inport.poll();
    assert_eq!(innote as int, portmidi::PmError::PmNoError as int);

    //send note
    let note1 = portmidi::PmEvent {
        message : portmidi::PmMessage {
            status : 1 | 0x90, //chanell and note on
            data1 : 36, //note number
            data2 : 90, // velocity
        },
        timestamp : 0
    };
    let sendnoteerr = outport.write_event(note1);
    assert_eq!(sendnoteerr as int, portmidi::PmError::PmNoError as int);

    let note2 = portmidi::PmMessage {
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
    let incloseerr = inport.close();
    assert_eq!(incloseerr as int, portmidi::PmError::PmNoError as int);

    //terminate midi
    portmidi::terminate().ok();
}
