include .env
export

.PHONY: all
all: format build test


.PHONY: format
format:
	cargo fmt


.PHONY: lint
lint:
	cargo clippy --all --all-targets --all-features


.PHONY: build
build:
	DATABASE_URL=$(DATABASE_URL) cargo build


.PHONY: test
test:
	DATABASE_URL='sqlite::memory:' cargo test


.PHONY: run
run:
	DATABASE_URL=$(DATABASE_URL) cargo run


.PHONY: clean
clean:
	cargo clean


.PHONY: migrate
migrate:
	DATABASE_URL=$(DATABASE_URL) sea-orm-cli migrate up

