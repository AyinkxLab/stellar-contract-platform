# Drips Wave Readiness Notes

This document is an evidence-oriented maintainer checklist, not a claim of Drips
Wave approval. Repository approval is decided by the Stellar Wave organizers.

## Project scope

Stellar Contract Observatory is a Rust workspace for inspecting Soroban contract
artifacts, parsing and fingerprinting contract interfaces, classifying interface
changes, evaluating compatibility policies, and verifying deployed artifact
identity. It includes a CLI and an API/platform layer.

## Current readiness snapshot (2026-10-28)

- **MSRV:** Rust 1.88, tested in CI (`MSRV (1.88.0)`).
- **Dependencies:** `serde_yml` replaced with `serde_yaml_ng`; `noyalib` is no
  longer present. `cargo audit` reports 0 vulnerabilities (one allowed,
  non-vulnerability warning for the unmaintained `unsafe-libyaml`).
- **CI:** Format, Clippy, tests on Linux/Windows/macOS, docs, coverage, MSRV, and
  the RustSec audit are green on `main` after the readiness PR.
- **Release:** `v0.1.0` is published with Linux x86_64, macOS arm64, and Windows
  x86_64 archives plus SHA256 checksums:
  <https://github.com/AyinkxLab/stellar-contract-platform/releases/tag/v0.1.0>.
- **Backlog:** 122 open issues; the curated [Wave board](WAVE_BOARD.md) has 20
  open, independent, unimplemented issues (2,700 points).
- **Docs:** repository, install, and Wave links use `AyinkxLab/...`; completed
  issues #9, #102, and #112 are closed and removed from the board.
- **Positioning:** [Why Observatory](WHY_OBSERVATORY.md) documents the boundary
  with `ai-code-assistant`, `stellar-devkit`, and `soroban-security-scanner`.
- **Not done:** crates.io publication, live RPC transport, persistence, SDKs,
  webhooks, and a composite GitHub Action.

## Contribution integrity

Do not count commits from accounts controlled by the maintainer as independent
external contributions. Reviewers can correlate account ownership and activity
patterns, and sockpuppeting is a Wave terms violation. If there are no genuinely
independent contributors yet, say so and describe the outreach plan instead.

## Verified implementation and boundaries

Implemented capabilities are tracked in [Project Status](PROJECT_STATUS.md) and
the crate-level tests. The project is intentionally offline-first for artifact
inspection and does not execute untrusted WASM.

Known limitations must remain visible in project descriptions:

- Live RPC transport is not implemented; deployment and verification currently
  rely on deterministic fixtures.
- Verification is hash-based, not source-reproducible rebuild verification.
- API keys, users, audit events, and webhook-related state are not persisted.
- The HTTP adapter is intended for local/testing use, not as a hardened public
  edge server.
- The project is not published to crates.io.

Do not describe these capabilities as production-ready until the corresponding
implementation, tests, and documentation support that claim.

## Contributor readiness

Before adding issues to a Wave:

- Keep issues scoped to one reviewable change with explicit acceptance criteria.
- Include relevant modules, testing expectations, security considerations, and
  documentation requirements.
- Label complexity honestly; assign Wave points in the Drips dashboard.
- Ask contributors to request assignment before beginning work.
- Require deterministic tests and malformed-input tests for parser/decoder work.
- Require cargo fmt --all -- --check, Clippy with warnings denied, and workspace
  tests before merge.
- Update the changelog and project status when user-visible behavior changes.
- Never add placeholder implementations or claim unimplemented work is complete.

## Security and CI

- CI uses read-only repository permissions for build/test jobs.
- Dependency advisory checks run on manifest/lockfile changes and weekly via
  .github/workflows/security-audit.yml.
- Review the workflow result and resolve or explicitly document advisories before
  treating the audit as clean.
- Never commit credentials or secrets. Report suspected vulnerabilities through
  [SECURITY.md](../SECURITY.md), not a public issue.

## Drips application and appeal checklist

The official process is documented at
[Participating in a Wave](https://docs.drips.network/wave/maintainers/participating-in-a-wave/).

- The repository must be public and synced by the Drips Wave GitHub App.
- Apply to the relevant program from **Maintainers → Orgs and Repos**.
- Confirm the repository's current status in that dashboard; email is not the
  only place to check status.
- If rejected, use the dashboard's **Appeal** action. The documented rules state
  that the first appeal can be submitted two weeks after rejection, a declined
  appeal has a one-month cooldown, and each repository has at most three appeals.
- An appeal should list specific, merged or reviewable code changes, test/CI
  evidence, documentation improvements, and the ecosystem need they address.
- Do not imply that this checklist guarantees acceptance. Appeals require
  substantive improvements and are evaluated by the program organizers.
