
all: build-rust

build-rust:
	cd rust/ && RUST_BACKTRACE=1 cargo run
