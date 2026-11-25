
.DEFAULT_GOAL := help

# Arguments
platform ?= codeforces
name     ?=
input    ?= tests/input.txt
expected ?= tests/expected.txt

# Paths
PLATFORM_DIR := platforms/$(platform)
TARGET       := $(name)
ACTUAL       := tests/actual.txt

# Platform configuration
ifeq ($(platform),leetcode)
    TEMPLATE := templates/leetcode.rs
    SRC_DIR  := $(PLATFORM_DIR)/tests
else
    TEMPLATE := templates/stdin.rs
    SRC_DIR  := $(PLATFORM_DIR)/src/bin
endif

FILE := $(SRC_DIR)/$(TARGET).rs

.PHONY: new run test build lint fix setup clean help validate

# Validation
validate:
	@test -n "$(name)" || { echo "[ERROR] name is required"; exit 1; }
	@test -f "$(PLATFORM_DIR)/Cargo.toml" || { echo "[ERROR] Unknown platform: $(platform)"; exit 1; }

# Generate solution
new: validate
	@mkdir -p "$(SRC_DIR)"
	@test ! -e "$(FILE)" || { echo "[ERROR] File already exists"; exit 1; }
	@cp "$(TEMPLATE)" "$(FILE)"
	@echo "[CREATED] $(FILE)"

# Execute and test
ifeq ($(platform),leetcode)

run test: validate
	@cargo test --release -p "$(platform)" --test "$(TARGET)"

build: validate
	@cargo test --release -p "$(platform)" --test "$(TARGET)" --no-run

else

run: validate
	@cargo run --quiet --release -p "$(platform)" --bin "$(TARGET)" < "$(input)"

test: validate
	@cargo run --quiet --release -p "$(platform)" --bin "$(TARGET)" < "$(input)" > "$(ACTUAL)"
	@awk -f scripts/compare.awk "$(expected)" "$(ACTUAL)"

build: validate
	@cargo build --release -p "$(platform)" --bin "$(TARGET)"

endif

# Code quality
lint:
	@cargo clippy --workspace --all-targets

fix:
	@cargo clippy --fix --workspace --all-targets --allow-dirty

# Project setup
setup:
	@mkdir -p tests
	@touch tests/input.txt tests/expected.txt
	@git config core.hooksPath .githooks
	@echo "[PASS] Setup completed"

# Cleanup
clean:
	@cargo clean
	@rm -f "$(ACTUAL)"

# Help
help:
	@echo "Usage: make <command> [platform=<platform>] [name=<problem>]"
	@echo ""
	@echo "Commands:"
	@echo "  new    Generate solution from template"
	@echo "  run    Execute solution"
	@echo "  test   Compare expected and actual output"
	@echo "  build  Compile solution"
	@echo "  lint   Run Clippy"
	@echo "  fix    Auto-fix Clippy issues"
	@echo "  setup  Initialize development environment"
	@echo "  clean  Remove build artifacts"
	@echo "  help   Show available commands"
	@echo ""
	@echo "Default platform: codeforces"
	@echo ""
	@echo "Examples:"
	@echo "  make new name=1999a"
	@echo "  make test name=1999a"
	@echo "  make new platform=leetcode name=1"
	@echo "  make test platform=leetcode name=1"
