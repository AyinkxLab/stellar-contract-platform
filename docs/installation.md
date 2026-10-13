# Installation

## Requirements

- A recent stable Rust toolchain (see the workspace `rust-version`).
- No network access is required for inspection, diffing, or compatibility.

## With `cargo install`

Install or update the CLI binary directly from the repository:

```bash
cargo install --git https://github.com/StellarFoundry/stellar-contract-platform observatory-cli --locked
stellar-contract-platform doctor
```

`--locked` builds against the committed `Cargo.lock`. The package is named
`observatory-cli`; the installed binary is `stellar-contract-platform`. Pass
`--force` to reinstall when a binary is already present.

## From source

```bash
git clone https://github.com/StellarFoundry/stellar-contract-platform
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

Crates are not published to crates.io yet, so `cargo install observatory-cli`
does not work; use `cargo install --git` above until a crates.io release and
prebuilt binaries exist. The project is pre-1.0. Distribution and release
packaging are tracked as future work.
