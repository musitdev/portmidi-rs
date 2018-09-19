extern crate portmidi as pm;

use std::time::Duration;
use std::sync::mpsc;
use std::thread;

fn main() {
    // initialize the PortMidi context.
    let context = pm::PortMidi::new().unwrap();
    let timeout = Duration::from_millis(10);
    const BUF_LEN: usize = 1024;
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let in_ports = context
            .devices()
            .unwrap()
            .into_iter()
            .filter_map(|dev| context.input_port(dev, BUF_LEN).ok())
            .collect::<Vec<_>>();
        loop {
            for port in &in_ports {
                if let Ok(Some(events)) = port.read_n(BUF_LEN) {
                    tx.send((port.device(), events)).unwrap();
                }
            }
            thread::sleep(timeout);
        }
    });

    loop {
        let (device, events) = rx.recv().unwrap();
        for event in events {
            println!("[{}] {:?}", device, event);
        }
    }
}
