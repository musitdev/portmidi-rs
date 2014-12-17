extern crate portmidi;

#[main]
fn main() {
    let error = portmidi::initialize();
    println!("res {}", error);
}
