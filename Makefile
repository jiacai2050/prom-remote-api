

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
