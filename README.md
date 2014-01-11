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

PortMidi binding is build with the rustpkg tool :

```Shell
> rustpkg build portmidi
```

Examples are build too with rustpkg :

```Shell
> rustpkg build examples/portmidiex1
```
Test suite. Contains example of use.
```Shell
> rustpkg test portmidi
```

PortMidi on Linux, windows and OSX. The binding has been only tested on Linux

License
=======

This software is a binding of the PortMidi library which is provide under  PortMidi license a MIT like license.

This software is provide under the MIT license.

