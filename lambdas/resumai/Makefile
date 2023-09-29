build:
	cargo lambda build --release --arm64 --output-format zip

.PHONY: all clean doc test deploy
all: doc
	cargo build

test:
	cargo test
	cargo fmt -- --check
	cargo clippy

clean:
	cargo clean

doc:
	cargo doc

local:
	cargo build
	cargo run