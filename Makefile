RUSTPKG ?= rustpkg
RUST_FLAGS ?= -Z debug-info -O

all:
	$(RUSTPKG) $(RUST_FLAGS) install portmidi

examples: all
	$(RUSTPKG) $(RUST_FLAGS) install examples/portmidi
clean:
	rm -rf bin build lib
