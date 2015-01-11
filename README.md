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

License
=======
This software is provide under the MIT license. PortMidi is provided under it's own MIT-like license.

