.PHONY: all
all: lintformat build test


.PHONY: lintformat
lintformat:
	cargo clippy
	cargo fmt


.PHONY: build
build:
	cargo build


.PHONY: test
test:
	DATABASE_URL='sqlite::memory:' cargo nextest run


.PHONY: cov
cov:
	cargo tarpaulin --color always --ignore-tests


.PHONY: run
run:
	DATABASE_URL='sqlite://tasks.sqlite3?mode=rwc' cargo run


.PHONY: clean
clean:
	cargo clean
