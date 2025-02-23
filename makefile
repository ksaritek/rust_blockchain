.PHONY: tools
tools:
	rustup component add clippy

.PHONY: test
test:
	cargo test -- --nocapture

.PHONY: develop
develop:
	cargo build

.PHONY: release
release:
	cargo build --release

.PHONY: clean
clean:
	cargo clean

