# Leptos WASM Code Splitting - Filename Too Long Issue

This repository demonstrates a critical issue with Leptos lazy routes and WASM code splitting that causes "filename too long" OS errors on macOS.

## Issue Description

When using Leptos lazy routes, the generated filenames for lazy-loaded WASM chunks can exceed OS filename limits, particularly in release builds.

## Environment

- **OS**: macOS (Apple Silicon M3)
- **Leptos Version**: 0.8.5
- **cargo-leptos Version**: 0.2.42
- **Rust**: Latest nightly (configured via rust-toolchain.toml)

## Problem Details

The issue occurs when:
1. Multiple lazy routes are defined using the `LazyRoute` trait and `#[lazy_route]` macro
2. Routes have long struct names (e.g., `LazyEnterpriseResourcePlanningSystemIntegrationModule`)
3. Building with `cargo leptos build --release --split`
4. The `hashed = true` option is enabled in `Cargo.toml`

The combination of:
- Long type names from the lazy route structs
- WASM module splitting 
- Hash generation for cache busting
- Release mode optimizations

...results in generated filenames that exceed the 255 character limit on macOS filesystems.

## Reproduction Steps

1. Clone this repository
2. Install cargo-leptos: `cargo install cargo-leptos`
3. Run in debug mode (works): `just run debug`
4. Run in release mode (fails): `just run release`

## Expected Behavior

The build should complete successfully with split WASM modules having reasonable filename lengths.

## Actual Behavior

Debug builds work fine:
```bash
just run debug  # Success
```

Release builds fail with OS error:
```bash
just run release  # Fails with "Error: filename too long (os error 63)"
```

## Project Structure

The project includes:
- 9 standard lazy routes with normal names
- 10 additional lazy routes with intentionally long names to demonstrate the issue
- Each lazy route generates a separate WASM chunk when built with `--split`

## Workarounds

Current workarounds include:
1. Disabling code splitting (removes `--split` flag)
2. Using shorter struct names for lazy routes
3. Building in debug mode only

None of these are ideal for production applications that need code splitting for performance.

## Related Issues

This appears to be a systematic issue with how Leptos generates filenames for lazy-loaded WASM chunks, particularly when combining:
- Type name mangling
- Hash generation
- Module splitting
- Release optimizations

The issue is particularly acute on macOS due to its 255 character filename limit.