portmidi-rs
===========

[![Build Status](https://travis-ci.org/musitdev/portmidi-rs.svg?branch=master)](https://travis-ci.org/musitdev/portmidi-rs)
[![Documentation](https://img.shields.io/badge/rustdoc-hosted-blue.svg)](http://musitdev.github.io/portmidi-rs/portmidi/index.html)

High-level PortMidi bindings for Rust.

PortMidi website: http://portmedia.sourceforge.net/portmidi/

Installation
============

Add this to your `Cargo.toml`.
```toml
[dependencies]
portmidi = "^0.2"
```

Prerequisites
-------------

You need to make sure you have the PortMidi library installed.

On Ubuntu / Debian:
```sh
apt-get install libportmidi-dev
```

Arch Linux:
```sh
pacman -S portmidi
```

On OSX (Homebrew):
```sh
brew install portmidi
```
On OSX, if you get a linker error `ld: library not found for -lportmidi`, either,
 - make sure you have the Xcode Command Line Tools installed, not just Xcode, or
 - make sure you have the PortMidi library in your `$LIBRARY_PATH`, e.g. for Homebrew:

   ```sh
   export LIBRARY_PATH="$LIBRARY_PATH:/usr/local/lib"
   ```

Examples
========
Examples can be run by cloning the repository and running `cargo run --example <example name>`.
 * **`play`**: demonstrates midi output by playing Twinkle Twinkle Little Star (forever...)
 * **`monitor`**: demonstrate midi input
 * **`monitor-all`**: listens on all-input devices and uses threads and channels

Example: `cargo run --example play -- 1 --verbose`

Both `play` and `monitor` need a device number supplied, run them without an argument to get a list of the connected devices, e.g.

License
=======
Licensed under either of

    Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
    MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.

Contribution
============
Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
