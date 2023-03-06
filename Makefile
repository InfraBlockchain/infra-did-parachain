.PHONY: build
build:
	cargo b --profile production

.PHONY: update
update:
	cargo update

.PHONY: dev
dev:
	cargo run -- --chain=wisp-localdev --alice --tmp