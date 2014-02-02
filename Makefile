all: lib examples doc

lib:
	mkdir -p lib
	rustc --out-dir=lib src/portmidi/lib.rs

doc:
	mkdir -p doc
	rustdoc -o doc src/portmidi/lib.rs


examples: all
	rustc -o bin -L ./lib src/examples/portmidiex1/main.rs

clean:
	rm -rf bin lib doc
