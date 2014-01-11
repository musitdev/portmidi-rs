///  build : rustpkg test portmidi

extern mod portmidi;

#[allow(type_overflow)]
#[cfg(test)]
mod tests {
    use portmidi::midi;
    use portmidi::time;
    use portmidi::util;

    struct Sequencer   {
        midi_notes: ~[midi::PmEvent],
        inport : ~midi::PmInputPort,
    }

    fn sequencer_callback(time:u64, data:&mut Sequencer)   {
        println!("sequencer_callback time:{:?}", time);
     //   let inport = *data.inport;
        
        while (match data.inport.poll() { midi::pmGotData => true, _ => false })    {
            // println!("portmidi input note {:?}", readMidi);
            match (data.inport.read())    {
                Ok(notes) => println!("portmidi read midi note {:?}", notes),
                Err(midi::pmNoError) => println!("portmidi read midi no note {:?}", midi::pmNoError),
                Err(err) => println!("portmidi read midi error {:?}", err)
            } 
        }

    }


    #[test]
    fn test_midiin() {
    	let error:midi::PmError = midi::initialize();
    	assert_eq!(error as int, midi::pmNoError as int);

    	let nbdevice : int = midi::count_devices();
    	println!("portmidi nb device {:?}", nbdevice);
    	let defdevin : int = midi::get_default_input_device_id();
    	println!("portmidi default input device {:?}", defdevin);
    	let defdevout : int = midi::get_default_output_device_id();
    	println!("portmidi default output device {:?}", defdevout);

        let ininfo = midi::get_device_info(defdevin);
        println!("portmidi default input device info {:?}", ininfo);

        let outinfo = midi::get_device_info(defdevout);
        println!("portmidi default output device info {:?}", outinfo);

        let mut inport : midi::PmInputPort = midi::PmInputPort::new(defdevin, 0);
        let inerror = inport.open();
        assert_eq!(inerror as int, midi::pmNoError as int);

        let mut outport : midi::PmOutputPort = midi::PmOutputPort::new(defdevout, 100);
        let outerror = outport.open();
        println!("portmidi new output device {:?}", outerror);
        assert_eq!(outerror as int, midi::pmNoError as int);

        let readMidi = inport.read();
        println!("portmidi input note {:?}", readMidi);
        match (readMidi)    {
            Ok(notes) => println!("portmidi read midi note {:?}", notes),
            Err(midi::pmNoError) => println!("portmidi read midi no note {:?}", midi::pmNoError),
            Err(err) => println!("portmidi read midi error {:?}", err)
        }

        let innote = inport.poll();
        assert_eq!(innote as int, midi::pmNoError as int);
        
        //send note
        let note1 = midi::PmEvent {
            message : midi::PmMessage {
                status : 1 | 0x90, //chanell and note on
                data1 : 36, //note number
                data2 : 90, // velocity              
            },
            timestamp : 0
        };
        let sendnoteerr = outport.write_event(note1);
        assert_eq!(sendnoteerr as int, midi::pmNoError as int);

        let note2 = midi::PmMessage {
            status : 1 | 0x80, //chanell and note off
            data1 : 36, //note number
            data2 : 0, // velocity              
        };
        let sendnote2err = outport.write_message(note2);
        assert_eq!(sendnote2err as int, midi::pmNoError as int);

        //test sequencer
        let data = Sequencer{midi_notes: ~[], inport: ~inport};
        let mut timer = time::PtTimer::Pt_start(1000, data, sequencer_callback);
        time::Pt_Sleep(10000);
        timer.Pt_Stop(); 



        //close out port
        let aborterr = outport.abort();
        assert_eq!(aborterr as int, midi::pmNoError as int);
        let outcloseerr = outport.close();
        assert_eq!(outcloseerr as int, midi::pmNoError as int);

        //close in port
        let incloseerr = inport.close();
        assert_eq!(incloseerr as int, midi::pmNoError as int);

        //terminate midi
    	let error:midi::PmError = midi::terminate();
    	assert_eq!(error as int, midi::pmNoError as int);
    }

   #[test]
    fn test_queue() {
        let mut queue : util::PmQueue = util::PmQueue::new();
        queue.create(32, 4);

        let readMidi = queue.dequeue();
        match (readMidi)    {
            Ok(notes) => println!("portmidi read midi note {:?}", notes),
            Err(midi::pmNoError) => assert_eq!(midi::pmNoError as int, midi::pmNoError as int),
            Err(err) => fail!("portmidi read midi error {:?}", err)
        }

        assert_eq!(queue.is_empty(), true);
        assert_eq!(queue.is_full(), false);

        let peek1 = queue.peek();
        match (peek1)   {
            None => assert_eq!(peek1, None),
            _ => fail!("queue.peek  bad result. not None"),
        }

        let enqueuerr = queue.enqueue (
            midi::PmMessage {
                status : 1 | 0x90, //chanell and note on
                data1 : 36, //note number
                data2 : 90, // velocity              
            }
        );
        assert_eq!(enqueuerr as int, midi::pmNoError as int);

        assert_eq!(queue.is_empty(), false);
        assert_eq!(queue.is_full(), false);

        let peek1 = queue.peek();
        match (peek1)   {
            Some(notes) => assert_eq!(notes.data1, 36),
            None => fail!("queue.peek2  bad result. None"),
        }

        assert_eq!(queue.is_empty(), false);
        assert_eq!(queue.is_full(), false);

        let readqueue = queue.dequeue();
        match (readqueue)    {
            Ok(notes) => assert_eq!(notes.data1, 36),
            Err(midi::pmNoError) => fail!("dequeue error no object found {:?}", readqueue),
            Err(err) => fail!("portmidi read midi error {:?}", err)
        }

        assert_eq!(queue.is_empty(), true);
        assert_eq!(queue.is_full(), false);

        let queudesterr = queue.destroy();
        assert_eq!(queudesterr as int, midi::pmNoError as int);
   }

   struct TestMutCallback   {
        data: int,
   }

    fn test_callback(time:u64, data:&mut TestMutCallback)   {
        data.data = data.data + 1;
        println!("testcallback time:{:?} data:{:?}", time, data);
        let cal: int = (time / 1000) as int;
        assert_eq!(data.data, cal);
    }

    #[test]
    fn test_timer() {
        let data : TestMutCallback = TestMutCallback{data: 0};
        let mut timer = time::PtTimer::Pt_start(1000, data, test_callback);
        assert_eq!(timer.Pt_Started(), true);

        println!("test_timer start time:{:?} ", timer.Pt_Time());

        time::Pt_Sleep(5000);
        timer.Pt_Stop();
        assert_eq!(timer.Pt_Started(), false);
        println!("test_timer end time:{:?} ", timer.Pt_Time());
        assert_eq!(timer.Pt_Time() >= 5000, true);
        assert_eq!(timer.Pt_Time() < 6000, true);
        assert_eq!(data.data, 0);
    }
}
