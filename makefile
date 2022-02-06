run:
	cargo run --release

build:
	cargo build --release

test:
	cargo test

check:
	cargo check --all --tests

lint:
	cargo clippy --all-targets