# DoggyWriter - Just Commands

# Default recipe (run when you type 'just')
default: run

# Build the project in debug mode
build:
    cargo build

# Run the application
run:
    cargo run

# Build optimized release version
release:
    cargo build --release

# Run tests
test:
    cargo test

# Check code without building
check:
    cargo check

# Format code
fmt:
    cargo fmt

# Run clippy linter
lint:
    cargo clippy

# Clean build artifacts
clean:
    cargo clean

# Install locally (puts binary in ~/.cargo/bin/)
install:
    cargo install --path .

# Publish to crates.io (make sure you're logged in with 'cargo login')
publish:
    @echo "Publishing to crates.io..."
    @echo "Make sure you've updated the version in Cargo.toml!"
    @echo "Make sure you've committed all changes!"
    cargo publish

# Build and run in release mode
run-release:
    cargo run --release

# Show help
help:
    @echo "DoggyWriter - Available commands:"
    @echo ""
    @echo "  just build        - Build the project"
    @echo "  just run          - Run the application"
    @echo "  just release      - Build optimized version"
    @echo "  just test         - Run tests"
    @echo "  just check        - Check code without building"
    @echo "  just fmt          - Format code"
    @echo "  just lint         - Run clippy linter"
    @echo "  just clean        - Clean build artifacts"
    @echo "  just install      - Install locally"
    @echo "  just publish      - Publish to crates.io"
    @echo "  just run-release  - Build and run in release mode"
