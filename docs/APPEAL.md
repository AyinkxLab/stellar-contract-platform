# Drips Wave Appeal Summary

This page prepares an appeal for a rejected repository application. It does not
guarantee reconsideration. Appeals are decided by the Stellar Wave organizers.

## Before submitting

1. Open **Maintainers → Orgs and Repos** in the Drips Wave app.
2. Confirm the status of `AyinkxLab/stellar-contract-platform`:
   - **Pending** — wait; there is nothing to appeal yet.
   - **Approved** — add issues from the dashboard.
   - **Rejected** — note the date and any stated reason.
3. Timing rules:
   - First appeal: **no earlier than two weeks after rejection**.
   - Declined appeal: **one-month cooldown**.
   - Maximum **three appeals** per repository.
4. Submit **only through the app**; email/Discord appeals are ignored.

## What changed since the last review

| Concern | Change | Evidence |
| ------- | ------ | -------- |
| CI was red | MSRV raised to 1.88; `serde_yml` replaced with `serde_yaml_ng`; RustSec clean | CI and `Dependency Security Audit` on `main` |
| Docs pointed at the old owner | Repository, install, and Wave links now use `AyinkxLab/...` | README, `Cargo.toml`, `docs/installation.md`, `docs/WAVE_BOARD.md` |
| Docs claimed 130 issues and CI success | Counts and CI status updated; completed issues closed | `docs/PROJECT_STATUS.md`, `docs/FINAL_AUDIT.md`, `docs/ISSUE_BACKLOG.md` |
| Wave board contained completed work | #9, #102, #112 closed; board rebuilt with open, independent issues | `docs/WAVE_BOARD.md` |
| No installable artifact | Tag-driven release workflow; Linux/macOS/Windows archives with SHA256 | `v0.1.0` release |
| Possible overlap with other repos | Scope boundary documented | `docs/WHY_OBSERVATORY.md` |

## Short appeal text (adapt as needed)

> Since the previous review, Stellar Contract Observatory has changed in five
> concrete ways:
>
> 1. **CI is green.** The MSRV is now Rust 1.88, and the unmaintained
>    `serde_yml`/`noyalib` dependency was replaced with `serde_yaml_ng`. Format,
>    Clippy, tests on Linux/Windows/macOS, docs, coverage, MSRV, and the RustSec
>    audit all pass on `main`.
> 2. **The docs match the repository.** Install and Wave links now use
>    `AyinkxLab/stellar-contract-platform`; test and issue counts are current.
> 3. **The backlog is accurate.** Completed issues #9, #102, and #112 were
>    closed. The curated Wave board contains 20 open, independent, unimplemented
>    issues.
> 4. **There is a release.** `v0.1.0` publishes Linux, macOS, and Windows
>    archives with SHA256 checksums.
> 5. **The scope is distinct.** This is a contract-intelligence layer for
>    pre-deployment safety: inspect, fingerprint, diff, and verify compiled
>    Soroban artifacts. It is not an AI assistant, an XDR/RPC devkit, or a
>    source-level vulnerability scanner.
>
> Evidence:
> - Repository: https://github.com/AyinkxLab/stellar-contract-platform
> - CI: https://github.com/AyinkxLab/stellar-contract-platform/actions
> - Release: https://github.com/AyinkxLab/stellar-contract-platform/releases/tag/v0.1.0
> - Status: https://github.com/AyinkxLab/stellar-contract-platform/blob/main/docs/PROJECT_STATUS.md
> - Scope: https://github.com/AyinkxLab/stellar-contract-platform/blob/main/docs/WHY_OBSERVATORY.md
> - Wave board: https://github.com/AyinkxLab/stellar-contract-platform/blob/main/docs/WAVE_BOARD.md

If the review cited a specific concern, answer it first and link the change.

## Contribution integrity

Do **not** present commits from maintainer-controlled accounts as independent
external contributions. Reviewers can correlate account ownership, email
domains, and activity patterns. If there are no genuine independent contributors
yet, say so and describe the outreach plan.

## Do not claim

- That the repository is approved, funded, or guaranteed points.
- That crates.io packages exist.
- That the platform layer is a hardened production service.
- That the tool proves a contract is secure.
- That the repository has significant external adoption.

## Still worth doing

- Publish crates to crates.io (16-crate dependency order; separate workflow).
- Get genuinely independent contributors or documented real usage.
- Add live RPC transport and SSRF hardening (#13, #58).
