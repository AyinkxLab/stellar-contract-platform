# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Events: accept raw base64 `ContractEvent` XDR, either as a bare input or an
  `xdr` field on a fixture entry, and normalize it to the existing `EventRecord`
  shape. See `docs/events.md`.

## [0.1.0] - 2026-10-28

First tagged release. Linux x86_64, macOS arm64, and Windows x86_64 archives
with SHA256 checksums are attached to the GitHub Release. crates.io publication
is still pending.

### Added

- CI: report test coverage with `cargo-llvm-cov` on pull requests. The job
  writes a summary to the workflow run, uploads an `lcov.info` artifact, and
  keeps the coverage floor advisory (`continue-on-error`). See `docs/testing.md`.
- Deployment: opt-in, bounded, per-run `LedgerEntryCache` with cached
  `inspect_with_cache` and `deployed_wasm_with_cache` lookups; repeated ledger
  keys reuse one `getLedgerEntries` call. See `docs/deployment.md`.
- Typed JSON-RPC error taxonomy: `RpcError`/`RpcErrorKind` classify standard
  JSON-RPC codes, preserve unknown raw codes, and expose `rpc_kind`/`rpc_code`
  in error reports. See `docs/rpc-errors.md`.
- Project configuration: `observatory.yml`/`.yaml`/`.toml`/`.json` with strict
  validation and auto-discovery, plus `--config`. Selects the compatibility
  policy and API auth/rate-limit defaults, and provides a default network for
  `verify`/`deployment`. See `docs/configuration.md`.
- Developer platform layer:
  - `observatory-platform`: application service layer plus API-key
    authentication (SHA-256 hashed secrets, never serialized), hierarchical
    RBAC, fixed-window rate limiting, and strict platform configuration.
  - `observatory-api`: versioned REST API at `/api/v1` with public
    health/readiness/version endpoints and six contract endpoints (inspect,
    spec, diff, compatibility, fingerprint, security), versioned report
    envelopes, request IDs, rate-limit headers, and a generated OpenAPI document
    that is kept in sync with the routes by a test.
  - CLI: `api serve` and `api openapi`.
- Workspace with 16 crates covering inspection, specification, interface model,
  events, diff, compatibility, RPC, deployment, verification, audit, output,
  platform, API, and the CLI.
- `observatory-wasm`: bounded, non-executing WASM inspection (sections, imports,
  exports, memories, custom sections, code size) and SHA-256 artifact hashing.
- `observatory-spec`: bounded `contractspecv0` parsing into a normalized,
  parser-independent model (functions, structs, unions, enums, error enums,
  events) with validation.
- `observatory-interface`: canonical interface model with deterministic
  interface fingerprinting and a known-value test vector.
- `observatory-events`: event fixture decoding, normalization, and filtering.
- `observatory-diff`: deterministic interface diff with breaking, non-breaking,
  and informational classification.
- `observatory-compat`: documented, configurable compatibility policy returning
  compatible / incompatible / unknown with reasons.
- `observatory-rpc`: transport abstraction, endpoint validation, typed
  `getLatestLedger` / `getNetwork` / `getLedgerEntries` models, and a
  deterministic mock/fixture transport.
- `observatory-deployment`: deployed contract inspection and `ContractExecutable`
  WASM hash recovery from real ledger-entry XDR.
- `observatory-verify`: local-vs-deployed verification and `contractmetav0` /
  `contractenvmetav0` build metadata parsing and comparison.
- `observatory-audit`: bounded, deterministic interface/artifact heuristics with
  clearly labelled experimental rules.
- `observatory-output`: versioned JSON report envelopes and stable error reports.
- `observatory-cli`: `inspect`, `spec`, `events`, `diff`, `compat`,
  `fingerprint`, `deployment`, `verify`, and `doctor`, all with `--json`.
- CI: format, clippy, and tests on Linux, Windows, and macOS.
- Release: tag-driven Linux/macOS/Windows binaries with SHA256 checksums.
- Documentation: `docs/WHY_OBSERVATORY.md`, `docs/APPEAL.md`, and an offline
  `demo/` with deterministic old/new WASM fixtures.

### Changed

- Build: raised the workspace MSRV from 1.85 to 1.88 so the locked dependency set
  (`wasmparser` 0.259, `stellar-strkey` 0.0.18, `serde_with` 3.23) resolves and
  the MSRV CI job matches the workspace `rust-version`.
- The repository was renamed to `stellar-contract-platform`. The CLI binary
  (`stellar-contract-platform`), the `observatory-*` crates, and the product
  name remain unchanged; only the repository URL and clone instructions changed.
- Documentation: repository, installation, and Wave links now use
  `AyinkxLab/stellar-contract-platform`.

### Security

- Replaced the unsound, unmaintained `serde_yml` parser (which pulled in
  `noyalib` 0.0.5, RUSTSEC-2026-0333) with the maintained `serde_yaml_ng`
  fork. YAML project configuration parsing is unchanged.

### Documentation

- Installation: document the prebuilt archives and the working
  `cargo install --git` path (package `observatory-cli`, binary
  `stellar-contract-platform`). See `docs/installation.md`.
- Project status, final audit, issue backlog, and Wave board now match `main`.

### Notes

- No live HTTP RPC transport is shipped yet. Deployment and verification run
  against deterministic RPC fixtures.
- The audit crate is not a security auditor.
