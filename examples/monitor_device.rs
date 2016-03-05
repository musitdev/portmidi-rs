extern crate portmidi as pm;

fn main() {
    let context = pm::PortMidi::new().unwrap();
    let info = context.default_input_device_id().and_then(|id| context.device_info(id));
    println!("{:?}", info);
    let mut in_port = context.default_input_port().unwrap();

    while let Ok(avail) = in_port.poll() {
        if let Ok(Some(event)) = in_port.read() {
            println!("{:?}", event);
        }
    }
}
