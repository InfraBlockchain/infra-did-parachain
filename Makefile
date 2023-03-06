.PHONY: build
build:
	cargo b --release

.PHONY: update
update:
	cargo update

.PHONY: dev
dev:
	cargo run -- --chain=infradid-localdev --alice --tmp