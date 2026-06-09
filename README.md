# OpenTelemetry Proto Rust Generator

This is an unmaintained fork of `open-telemetry/opentelemetry-proto` based on
the upstream `v1.10.0` tag.

The only purpose of this fork is to add a small Rust generator that uses
`prost-build` to generate Rust protobuf bindings from the checked-in
OpenTelemetry `.proto` files.

## What Was Added

- `Cargo.toml` and `Cargo.lock` for the Rust generator crate.
- `src/main.rs`, which compiles the OpenTelemetry proto files with `prost-build`.
- `flake.nix` and `flake.lock` for a Nix development shell and package.
- Generated Rust output goes to `gen/`, which is ignored by Git.

## Supported Platforms

The Nix flake supports:

- `aarch64-darwin`
- `aarch64-linux`
- `x86_64-linux`

Windows and Intel macOS are not supported.

## Generate Bindings

With Nix:

```sh
nix develop
cargo run
```

Or run the packaged generator:

```sh
nix run .
```

Without Nix, install Rust and `protoc`, then run:

```sh
cargo run
```

Generated files are written to `gen/`.

## Validate

```sh
cargo fmt --check
cargo check
nix flake check
```

When using Nix flakes from a Git worktree, new files must be tracked by Git
before Nix can see them.
