build:
	cargo build --release

run: build
	sudo ./target/release/fortipass-cli

test: build
	cargo test

clean:
	cargo clean

.PHONY: build run clean
