# Deployment inspection

`deployment inspect` looks up a deployed contract's instance entry and recovers
the executable WASM hash.

## How it works

1. The contract id (`C...` strkey) is decoded to its 32-byte hash.
2. A `LedgerKey::ContractData` key is built with `LedgerKeyContractInstance`.
3. `getLedgerEntries` is called through the configured transport.
4. The returned `LedgerEntryData` XDR is decoded; a WASM-backed instance yields
   `ContractExecutable::Wasm(hash)`, which is reported as hex.

Nothing about the RPC shape is invented; the key and entry are real Stellar XDR
types.

## Transports

The core does not hard-code a provider. Deployment uses the `Transport`
abstraction. This build ships a deterministic **fixture transport**; the live
HTTP transport is tracked as future work.

```bash
stellar-contract-platform deployment inspect C... \
  --rpc-fixture rpc.json --network testnet
```

`rpc.json` maps method names to results:

```json
{
  "getLedgerEntries": {
    "entries": [
      { "key": "AAAA...", "xdr": "AAAA...", "lastModifiedLedgerSeq": 123 }
    ]
  }
}
```

An error response is expressed as `{ "__error": { "code": -32000, "message": "..." } }`.

A method that is called several times can return results in order using
`{ "__sequence": [ result1, result2 ] }`. This is how interface verification
supplies the instance entry first and the code entry second.

## Deployed WASM retrieval

`deployment::deployed_wasm(client, wasm_hash_hex)` fetches the contract code
entry for a WASM hash and decodes the bytes. It builds a
`LedgerKey::ContractCode` key, calls `getLedgerEntries`, and decodes the
returned `LedgerEntryData`. It returns `None` when no code entry exists and
rejects hashes that are not 32 bytes or decode larger than the WASM size limit.

This enables interface comparison between a local artifact and the deployed
code (tracked as a follow-up).

## Ledger-entry cache

Repeated deployment lookups can reuse a small in-memory cache instead of issuing
a fresh `getLedgerEntries` call:

```rust
use observatory_deployment::{inspect_with_cache, deployed_wasm_with_cache, LedgerEntryCache};

let cache = LedgerEntryCache::with_default_capacity();
let info = inspect_with_cache(&client, contract_id, &cache)?;
let wasm = deployed_wasm_with_cache(&client, &hash, &cache)?;
```

The cache is **opt-in**: the existing `inspect` and `deployed_wasm` functions
keep their uncached behavior. Policy:

- Keyed by the base64-XDR ledger key for one process run.
- Bounded: `LedgerEntryCache::new(capacity)` retains at most `capacity` entries
  and evicts in insertion order. `with_default_capacity()` uses 128.
  A capacity of `0` disables caching.
- Negative results are cached too, so repeated lookups of a missing entry do not
  call the RPC again.
- Never persisted and never shared between runs, processes, or threads. Create a
  new cache per run and drop it when the run ends. `clear()` empties it.
- A cache hit returns the entry captured on the first call; it does not refresh
  ledger state. Use a fresh cache when a later lookup must observe new state.

`inspect_with_cache` and `deployed_wasm_with_cache` perform one transport call
for a repeated key. Unit tests assert this for both successful and missing
entries.

## Reported fields

`contract_id`, `wasm_hash`, `last_modified_ledger_seq`,
`live_until_ledger_seq`, and `found`.

## Limitations

- Only WASM-backed instances expose a hash; Stellar-asset contracts report no
  hash.
- Caching is opt-in and in-memory only; see the cache policy above.
- A live provider means a network call; keep such use out of unit tests.
