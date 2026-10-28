# Why Stellar Contract Observatory

This page explains what this project is, what it is not, and how it differs from
the other Stellar/Soroban tools in the same ecosystem.

## One sentence

Stellar Contract Observatory is a **contract-intelligence layer for pre-deployment
safety**: inspect a compiled Soroban contract, extract and fingerprint its
interface, diff two versions, apply a compatibility policy, and verify that a
deployed artifact matches the one you built.

## What it is

- A Rust workspace and a `stellar-contract-platform` CLI.
- A bounded, non-executing WASM inspector.
- A `contractspecv0` parser with a canonical interface model and deterministic
  fingerprint.
- An interface diff with breaking / non-breaking / informational classification.
- A compatibility policy engine with documented rules.
- A fixture-backed deployment/verification path that compares contract identity
  by hash.
- A versioned JSON output layer for automation and CI.

## What it is not

- Not an AI coding assistant.
- Not an XDR/RPC decoding and inspection toolkit.
- Not a source-level vulnerability scanner.
- Not a build/deploy/invoke replacement for `stellar-cli`.
- Not a hosted service, and not a hardened public API edge.
- Not proof that a contract is secure.

## Boundary with the other projects

| Project | Primary input | Primary job | This project's boundary |
| ------- | ------------- | ----------- | ----------------------- |
| `AyinkxLab/ai-code-assistant` | Repositories and source files | AI-assisted development workspace | Observatory does not chat, review PRs, or manage repos. It analyzes compiled artifacts. |
| `StellarFoundry/stellar-devkit` | XDR, transactions, events, RPC responses, WASM metadata | Developer inspection and RPC workflow | Observatory does not decode arbitrary XDR or run RPC health checks. It focuses on contract interfaces, diffs, compatibility, and artifact identity. |
| `StellarFoundry/soroban-security-scanner` | Soroban/Rust **source code** | Source-level security rules | Observatory does not scan source. It compares compiled interfaces and verifies artifact identity. |
| `stellar-cli` | Source projects | Build, deploy, invoke | Observatory consumes the built artifact; it does not build or deploy. |

The overlap is intentional but narrow: all four care about Soroban contracts.
Observatory owns the **artifact-to-artifact** question: *what changed, is it
breaking, and is this the artifact I think it is?*

## Workflows it is designed for

1. **Pre-upgrade review** — diff the old and new WASM interface and list breaking
   changes before deploying.
2. **CI compatibility gate** — fail a pipeline when the change violates the
   configured compatibility policy.
3. **Artifact identity check** — compare a local build with the deployed code
   hash using a recorded RPC fixture.
4. **Interface fingerprinting** — track an interface across releases without
   depending on source layout.
5. **Event-shape review** — decode fixture events and check the emitted topics
   and payloads against the expected interface.

## Honest limitations

- No live RPC transport; deployment/verification are fixture-backed today.
- Verification is hash-based, not source-reproducible rebuild verification.
- No persistence; API keys and configuration are in-memory.
- The HTTP adapter is local/test grade.
- The project is pre-1.0 and not published to crates.io.

If a workflow needs something not listed here, treat it as planned work only
when an open issue says so.
