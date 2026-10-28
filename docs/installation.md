# Installation

## Requirements

- Rust 1.88 or newer (the workspace `rust-version`).
- No network access is required for inspection, diffing, or compatibility.

## Prebuilt binaries

Download the archive for your platform from the
[Releases page](https://github.com/AyinkxLab/stellar-contract-platform/releases):

| Platform | Archive |
| -------- | ------- |
| Linux x86_64 | `stellar-contract-platform-v0.1.0-x86_64-unknown-linux-gnu.tar.gz` |
| macOS arm64 | `stellar-contract-platform-v0.1.0-aarch64-apple-darwin.tar.gz` |
| Windows x86_64 | `stellar-contract-platform-v0.1.0-x86_64-pc-windows-msvc.zip` |

Each archive has a matching `.sha256` checksum file. Verify it before running
the binary.

## With `cargo install`

Install or update the CLI binary directly from the repository:

```bash
cargo install --git https://github.com/AyinkxLab/stellar-contract-platform observatory-cli --locked
stellar-contract-platform doctor
```

`--locked` builds against the committed `Cargo.lock`. The package is named
`observatory-cli`; the installed binary is `stellar-contract-platform`. Pass
`--force` to reinstall when a binary is already present.

## From source

```bash
git clone https://github.com/AyinkxLab/stellar-contract-platform
cd stellar-contract-platform
cargo build --release
```

The binary is written to `target/release/stellar-contract-platform`.

## Verifying the build

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
```

## Platform support

The CLI and libraries are pure Rust and build on Linux, macOS, and Windows. CI
runs the test suite on all three.

## Packaging

Prebuilt binaries are published on the Releases page. Crates are not published
to crates.io yet, so `cargo install observatory-cli` does not work; use
`cargo install --git` above until a crates.io release exists. The project is
pre-1.0.
