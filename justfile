#!/usr/bin/env -S just --justfile

# Default recipe to list available commands
default:
    @just --list

# Build debug with code splitting
build-debug:
    cargo leptos build --split

# Build release with code splitting
build-release:
    cargo leptos build --release --split

# Run the project with specified target
run target:
    just build-{{target}}
    cd target/{{target}} && ./leptos-lazy-routes-issue

# Serve in development mode with code splitting
serve-debug:
    cargo leptos serve --split

# Serve in release mode with code splitting
serve-release:
    cargo leptos serve --release --split

# Clean build artifacts
clean:
    cargo clean
    rm -rf dist/
    rm -rf target/site/