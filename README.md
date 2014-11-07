rust-portmidi
=========


PortMidi bindings for Rust

This is a Rust binding for PortMidi lib.

PortMidi website  : http://portmedia.sourceforge.net/portmidi/

The following PortMidi lib are accessibles :
portmidi.h : midi functions
porttime.h : timer function. it's a rust implementation not a C binding.
pmutil.h : PmQueue functions



Installation
============

Use `cargo build` to compile library, to compile and run tests use `cargo test`.

PortMidi works on Linux, windows and OSX. The binding has been only tested on Linux
It's reported to work on Mac.

Documentation
=======
Auto generated documentation can be found at rust-ci: http://www.rust-ci.org/musitdev/rust-portmidi/doc/portmidi/

License
=======

This software is a binding of the PortMidi library which is provide under  PortMidi license a MIT like license.

This software is provide under the MIT license.

