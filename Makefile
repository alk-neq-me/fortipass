build:
	cargo build --release

run: build
	sudo ./target/release/fortipass-cli


clean:
	cargo clean

.PHONY: build run clean
