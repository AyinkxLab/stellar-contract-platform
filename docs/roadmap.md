# Roadmap

This roadmap lists real, unimplemented work derived from the current
architecture. It is not a promise of dates. Concrete tasks are tracked as
GitHub issues.

## Near term

- Live HTTP RPC transport behind the `Transport` trait, with explicit opt-in.
- Deployed WASM retrieval (`LedgerKeyContractCode`) and interface comparison of
  local vs. deployed code.
- Rename detection in the diff engine.

## Medium term

- GitHub Action and an `observatory.yml` configuration schema.
- Markdown report generation for inspection and compatibility results (the diff
  renderer already exists).
- Additional hash algorithms / selectable fingerprints.
- More audit rules, each with a documented rationale.

## Longer term

- Reproducibility workflow (rebuild and compare), distinct from metadata
  comparison.
- Multi-provider RPC support and caching.
- Fuzzing targets for the WASM, spec, event, and diff parsers.
- crates.io packaging for the workspace (the tag-driven binary release workflow
  is already in place).

## Non-goals

- Executing contract code.
- Acting as a security auditor.
- Replacing the Stellar CLI, SDKs, or binding generators.
