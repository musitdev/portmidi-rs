portmidi-rs
===========

[![Build Status](https://travis-ci.org/samdoshi/portmidi-rs.svg?branch=master)](https://travis-ci.org/samdoshi/portmidi-rs)

[Documentation](http://samdoshi.github.io/portmidi-rs/portmidi/index.html)

High-level PortMidi bindings for Rust.

PortMidi website: http://portmedia.sourceforge.net/portmidi/

Installation
============

Add this to your `Cargo.toml`.
```toml
[dependencies]
portmidi = "*"
```

Prerequisites
-------------

You need to make sure you have the PortMidi library installed.

On Ubuntu / Debian:
```sh
apt-get install libportmidi-dev
```

On OSX (Homebrew):
```sh
brew install portmidi
```
On OSX, if you get a linker error `ld: library not found for -lportmidi`, make sure you have the PortMidi library in your `$LIBRARY_PATH`, e.g. for Homebrew:
```sh
export LIBRARY_PATH="$LIBRARY_PATH:/usr/local/lib"
```

Examples
========
Examples can be run by cloning the repository and running `cargo run --example <example name>`.
 * **`list_devices.rs`**: the simplest example, displays a list of your midi devices
 * **`twinkle_twinkle.rs`**: demonstrates midi output by playing Twinkle Twinkle Little Star (forever...)
 * **`monitor_device.rs`**: demonstrate midi input

Both `twinkle_twinkle.rs` and `monitor_device.rs` need a device number supplied, run them without an argument to get a list of the connected devices, e.g.

```
$ cargo run --example twinkle_twinkle
     Running `target/examples/twinkle_twinkle`
Please supply a device number:

Id  Name                 Input? Output?
=======================================
0   IAC Driver Bus 1     true   false 
1   RADIAS MIDI IN       true   false 
2   RADIAS KBD/KNOB      true   false 
3   IAC Driver Bus 1     false  true  
4   RADIAS MIDI OUT      false  true  
5   RADIAS SOUND         false  true  
An unknown error occurred

To learn more, run the command again with --verbose.
```
(ignore the `An unknown error occurred`, that is due to the example setting a non-zero exit code, when no device number is supplied)

```
$ cargo run --example twinkle_twinkle 5
     Running `target/examples/twinkle_twinkle 5`
Opening: RADIAS SOUND
Press enter to quit
```

License
=======
This software is provide under the MIT license. PortMidi is provided under it's own MIT-like license.

