# Contributing through a Drips Wave

This project participates in open-source contribution waves. This page explains
how work is scoped, how to claim an issue, and what to expect from maintainers.

## Complexity and points

Every issue carries exactly one complexity label:

| Label | Wave points | Meaning |
| ----- | ----------- | ------- |
| `difficulty/trivial` | 100 | Small, clearly bounded change with obvious acceptance criteria. |
| `difficulty/medium` | 150 | A standard feature or logic touching several parts of the codebase. |
| `difficulty/high` | 200 | Complex engineering: integrations or architectural changes. |

The label is our internal, honest estimate. The **actual point value is set in
the Drips Wave maintainer dashboard** when an issue is added to a Wave Program,
using the same three levels (Trivial 100 / Medium 150 / High 200). Complexity is
assigned honestly: easy work is not inflated and hard work is not underpriced. If
an issue is too large for a single cycle, it is split into smaller independent
issues rather than relabelled.

## Maintainer setup (required before any Wave)

Participation is free for maintainers, but the repository must be approved by the
Wave Program organizers (for example, the Stellar Wave Program):

1. Sign in at the Drips Wave app with the GitHub account that owns the org.
2. Go to **Maintainers → Orgs and Repos** and install the **Drips Wave GitHub
   App** on the organization.
3. Sync the public repositories and **apply this repository** to the relevant
   Wave Program (Stellar).
4. Wait for organizer approval. If declined, appeal from the app only (first
   appeal after two weeks, then a one-month cooldown, maximum three appeals; a
   rejected repository cannot be re-applied, only appealed).

Once approved, issues appear in the **Maintainers → Issues** dashboard, where
complexity/points are assigned and a subset is added to the Program. The Wave bot
then comments on each selected issue and applies the Program label (for example
`Stellar Wave`); applying that label on GitHub also adds an issue to the Program.
Only add a curated, Wave-sized subset with a points budget — see
[`WAVE_BOARD.md`](WAVE_BOARD.md).

> This section documents the process. It does not claim the repository has been
> approved; approval is a separate, human step.

## Claiming an issue

1. Pick an issue. Start with
   [`good first issue`](https://github.com/AyinkxLab/stellar-contract-platform/labels/good%20first%20issue)
   if you are new.
2. **Request assignment before starting.** Apply through the Wave dashboard or
   comment on the issue. Do not open a pull request for an issue you have not
   been assigned.
3. Wait for a maintainer to assign you. This avoids duplicate work.

## Working on an issue

1. Fork the repository and create a branch named for the change, for example
   `feat/diff-markdown` or `fix/spec-trailing-bytes`.
2. Read the issue's **Scope**, **Implementation guidance**, and **Acceptance
   criteria** sections; they define what "done" means.
3. Write tests alongside the change.
4. Run the local gate:

   ```bash
   cargo fmt --all -- --check
   cargo clippy --workspace --all-targets --all-features -- -D warnings
   cargo test --workspace
   ```

5. Open a pull request whose description includes `Closes #<issue-number>` and a
   short summary of how you verified the change.

## What maintainers provide

- A prompt initial response to new issues and pull requests during an active
  Wave.
- A review decision (approve, request changes, or close with an explanation).
- Clear, actionable feedback, and a merge once the acceptance criteria are met.
- Honest complexity and scope on every issue.

## During a Wave

- Assignment is the priority: applications are reviewed and contributors are
  assigned quickly so there is enough time to finish before the Wave ends.
- A pull request is reviewed and merged as normal once the acceptance criteria
  are met.
- If a high-quality pull request cannot be merged in time for reasons outside the
  contributor's control, the issue is marked resolved so the contributor is
  still rewarded.
- Unresolved issues stay in the Program and roll over to the next Wave unless
  explicitly removed.

## After a Wave

Both the maintainer and the assigned contributor may leave a **two-way review**
within 14 days of the issue being closed. Reviews are anonymous and build
reputation across the ecosystem.

## Security

Do not report or discuss a security vulnerability in a public issue or pull
request. Follow [`SECURITY.md`](../SECURITY.md) and use private reporting.

## Definition of done

An issue is complete when:

- The change is implemented and matches the acceptance criteria.
- Tests are added and the full local gate passes.
- Documentation and `CHANGELOG.md` are updated when behavior is user-visible.
- The pull request is reviewed and merged.

## Useful references

- [`CONTRIBUTING.md`](../CONTRIBUTING.md)
- [`docs/WAVE_BOARD.md`](WAVE_BOARD.md)
- [`docs/ISSUE_BACKLOG.md`](ISSUE_BACKLOG.md)
- [`docs/development.md`](development.md)
