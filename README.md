# Stellar Contract Observatory

Build safer Soroban upgrades with contract intelligence, diffing, and deployment verification.

A Rust-first toolkit for inspecting, comparing, and verifying Stellar / Soroban contract artifacts before you ship them.

> Stop guessing about smart contract changes. Inspect the WASM, compare the interface, verify compatibility, and confirm the deployed artifact matches what you expect.

## Why this project matters

- Smart contract upgrades are risky when interfaces change silently.
- Teams need deterministic answers: what changed, is it breaking, and does the deployed contract match the local artifact?
- This project gives you a structured, machine-readable contract intelligence layer for audits, CI, and deployment safety.

## What you can do with it

- Inspect contract artifacts without executing untrusted WASM
- Extract and normalize contract specifications
- Diff versions and classify changes as breaking, non-breaking, or informational
- Verify compatibility using explicit rules
- Check deployed contract identity against a local artifact
- Produce deterministic fingerprints for artifact and interface tracking

## Quickstart

```bash
cargo build --release
./target/release/stellar-contract-platform doctor
```

```bash
# Inspect an artifact (JSON)
stellar-contract-platform inspect contract.wasm --json

# Summarize the contract specification
stellar-contract-platform spec contract.wasm

# Diff and compare interfaces
stellar-contract-platform diff old.wasm new.wasm
stellar-contract-platform compat old.wasm new.wasm

# Verify the deployed artifact against local state
stellar-contract-platform verify artifact contract.wasm \
  --contract C... --rpc-fixture rpc.json --network testnet
```

## Who it's for

- Soroban developers shipping upgrades
- Security reviewers checking compatibility
- Protocol teams verifying artifact identity before release
- CI pipelines that need deterministic contract validation

## Why it stands out

- Offline-by-default inspection
- Deterministic interface fingerprinting
- Compatibility policy engine with documented rules
- Machine-readable JSON output for automation and CI
- Built for real engineering workflows, not just raw low-level tooling

## Project status

Early but practical. The project focuses on contract intelligence, compatibility verification, and reproducibility workflows for Soroban development.

## Documentation

- [Architecture](docs/architecture.md)
- [Getting started](docs/getting-started.md)
- [CLI reference](docs/cli.md)
- [Compatibility](docs/compatibility.md)
- [Verification](docs/verification.md)
- [Roadmap](docs/roadmap.md)

## License

Apache-2.0. See [LICENSE](LICENSE).

---

