.PHONY: test
test:
	cargo test -- --nocapture

.PHONY: develop
develop:
	cargo build
