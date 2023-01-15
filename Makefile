ifeq ($(OS),Windows_NT)
	uname_S := Windows
else
	uname_S := $(shell uname -s)
endif

setup:
	cargo install cargo-sort
ifeq ($(uname_S), Darwin)
	brew install protobuf
else ifeq ($(uname_S), Linux)
	sudo apt install --yes protobuf-compiler
else
	echo "setup not supported on this OS"
endif

simple:
	cargo run --example simple --features="warp tokio"

clippy:
	cargo clippy --all-targets --all-features --workspace -- -D warnings

fmt:
	cargo fmt --check

cargo-sort:
	cargo sort --check

test:
	cargo test --all-features
