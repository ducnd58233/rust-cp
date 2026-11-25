
.DEFAULT_GOAL := help

TEMPLATE := templates/solution.rs
BIN_DIR  := src/bin
BIN      := cf_$(name)
FILE     := $(BIN_DIR)/$(BIN).rs

input    ?= tests/input.txt
expected ?= tests/expected.txt

.PHONY: new run test build check clean help check-name lint fix setup

check-name:
	@test -n "$(name)" || { echo "Error: name is required"; exit 1; }

new: check-name
	@mkdir -p $(BIN_DIR)
	@if [ -f "$(FILE)" ]; then \
		echo "Error: $(FILE) already exists"; \
		exit 1; \
	fi
	@cp $(TEMPLATE) $(FILE)
	@echo "Created $(FILE)"

run: check-name
	@cargo run --quiet --release --bin $(BIN) $(if $(input),< "$(input)")

test: check-name
	@test -f "$(input)" || { echo "Input file not found"; exit 1; }
	@test -f "$(expected)" || { echo "Expected file not found"; exit 1; }
	@actual=$$(mktemp) || exit 1; \
	trap 'rm -f "$$actual"' EXIT; \
	cargo run --quiet --release --bin $(BIN) < "$(input)" > "$$actual" || exit 1; \
	awk -f scripts/compare.awk "$(expected)" "$$actual"

build: check-name
	@cargo build --release --bin $(BIN)

check: check-name
	@cargo check --bin $(BIN)

clean:
	@cargo clean

lint:
	@cargo clippy --all-targets

fix:
	@cargo clippy --fix --all-targets --allow-dirty

setup:
	@git config core.hooksPath .githooks
	@echo "[PASS] Setup completed"

help:
	@echo "Usage:"
	@echo "  make new name=1999a"
	@echo "  make run name=1999a"
	@echo "  make test name=1999a"
	@echo "  make test name=1999a input=tests/input.txt expected=tests/expected.txt"
	@echo "  make build name=1999a"
	@echo "  make check name=1999a"
	@echo "  make clean"
