# Rust Shell Makefile

.PHONY: build test run enhanced demo clean help

help:
	@echo "Available targets:"
	@echo "  build     - Build the shell"
	@echo "  test      - Run tests"
	@echo "  run       - Run basic shell"
	@echo "  enhanced  - Run enhanced shell"
	@echo "  demo      - Run demo script"
	@echo "  clean     - Clean build artifacts"
	@echo "  help      - Show this help"

build:
	@echo "Building Rust Shell..."
	cargo build

test:
	@echo "Running tests..."
	cargo test

run:
	@echo "Starting basic shell..."
	cargo run

enhanced:
	@echo "Starting enhanced shell..."
	cargo run -- --enhanced

demo:
	@echo "Running demo..."
	chmod +x examples/demo.sh
	./examples/demo.sh

clean:
	@echo "Cleaning build artifacts..."
	cargo clean
	rm -rf target/ 