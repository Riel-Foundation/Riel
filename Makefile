.PHONY: build strip clean install uninstall all

build:
	cargo build --release > /dev/null 2>&1

dev:
	cargo build > /dev/null 2>&1
	strip target/debug/riel
	sudo cp target/debug/riel /usr/local/bin/
strip:
	strip target/release/riel

install:
	sudo cp target/release/riel /usr/local/bin/

clean:
	cargo clean

uninstall:
	rm /usr/local/bin/riel

all: build strip install clean