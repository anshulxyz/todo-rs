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
	cargo build


.PHONY: test
test:
	DATABASE_URL='sqlite::memory:' cargo nextest run


.PHONY: cov
cov:
	cargo tarpaulin --color always --exclude-files 'entity/*' \
		--exclude-files 'migration/*' --ignore-tests --skip-clean --locked


.PHONY: run
run:
	DATABASE_URL='sqlite://tasks.sqlite3?mode=rwc' cargo run


.PHONY: clean
clean:
	cargo clean

.PHONY: install
install:
	cargo install cargo-nextest --version 0.9.20
	cargo install cargo-tarpaulin --version 0.20.1
	cargo build
