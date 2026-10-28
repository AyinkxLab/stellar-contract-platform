# Testing

## The local gate

Run the same checks as CI before opening a pull request:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
```

## Test layout

- Unit tests live next to the code in `#[cfg(test)]` modules.
- CLI integration tests live in `crates/observatory-cli/tests/cli.rs` and spawn
  the built binary against deterministic fixtures.
- `observatory-testutil` builds fixtures from bytes and real `stellar-xdr`
  values; no checked-in binary blobs are required so tests stay reviewable.

## Determinism rules

- Unit tests must not touch the network.
- Fixtures are built deterministically; tests assert exact values.
- RPC behaviour is exercised through `MockTransport` and fixture JSON.

## Fixtures

| Fixture | Built by |
| ------- | -------- |
| Minimal WASM module | `observatory_testutil::minimal_module` |
| Module with a custom section | `observatory_testutil::module_with_custom_section` |
| Module with a contract spec | `observatory_testutil::spec::*` + `module_with_spec` |
| Event fixtures | `observatory-events` test helpers (base64 `ScVal`) |
| Ledger entries | `stellar-xdr` `LedgerEntryData` encoded in tests |

## Coverage

CI reports line coverage with [`cargo-llvm-cov`](https://github.com/taiki-e/cargo-llvm-cov),
which uses LLVM's source-based instrumentation. The `Coverage` job runs on every
pull request and on pushes to `main`:

```bash
mkdir -p target/coverage
cargo llvm-cov --workspace --all-features \
  --lcov --output-path target/coverage/lcov.info
cargo llvm-cov report --summary-only | tee target/coverage/summary.txt
```

The job:

- writes a readable summary to the GitHub Actions job summary (visible from the
  pull request checks page),
- uploads `target/coverage/lcov.info` and `target/coverage/summary.txt` as the
  `coverage` artifact,
- runs an advisory coverage floor with `continue-on-error: true`. The floor is
  `0` while a baseline is established, so coverage is **reported but not
  enforced**. Raise `COVERAGE_ADVISORY_MIN_LINES` in `.github/workflows/ci.yml`
  once a sensible baseline is agreed.

Run the same commands locally after installing the tool:

```bash
rustup component add llvm-tools-preview
cargo install cargo-llvm-cov --locked
cargo llvm-cov --workspace --all-features --open
```

`cargo llvm-cov` merges the workspace test run, so the numbers include unit,
integration, and CLI tests. Doc tests are not instrumented. Failed tests still
fail the job; only the coverage floor is advisory.

## Adding tests

- Decoding changes must include malformed-input tests.
- New diff/compat rules must include a positive and a negative case.
- New CLI flags must include an integration test asserting exit code and output.
