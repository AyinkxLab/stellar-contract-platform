# Events

`events` decodes a fixture of concrete emitted events and filters it. This is
different from the *declared* events in the contract specification.

## Fixture format

```json
{
  "events": [
    {
      "contractId": "C...",
      "ledger": 123,
      "txHash": "abcd",
      "type": "contract",
      "topics": ["<base64 ScVal>", "<base64 ScVal>"],
      "data": "<base64 ScVal>"
    }
  ]
}
```

A bare array is also accepted. `topics` and `data` are optional; `type` defaults
to `contract`. Topics and data are decoded as Stellar `ScVal` XDR and rendered to
JSON.

## Raw `ContractEvent` XDR

A base64 `ContractEvent` blob can be passed directly instead of a fixture
document. It can also appear as an `xdr` field on a fixture entry (`eventXdr` is
accepted as an alias):

```json
{
  "events": [
    {
      "xdr": "<base64 ContractEvent>",
      "ledger": 123,
      "txHash": "abcd"
    }
  ]
}
```

The contract id, event type, topics, and data are normalized exactly like a
fixture entry. `ledger` and `txHash` are optional metadata that only a fixture
can provide; a bare blob leaves them `null`. Malformed base64 or XDR returns a
structured decode error.

## Filtering

| Flag | Behavior |
| ---- | -------- |
| `--contract <ID>` | Exact contract id match. |
| `--topic <TEXT>` | Substring match against extracted topic text (symbols/strings) or rendered topics. |
| `--type <TYPE>` | Case-insensitive event type match. |
| `--limit <N>` | Applied last. |

## Limitations

- Declared events (specification) and emitted events (fixture) are distinct.
  Declared events describe *shape*; fixtures contain *instances*.
- Topics that are not symbols or strings cannot be text-filtered, but are still
  matched against their rendered JSON.
- Historical event retrieval over RPC is not implemented yet; fixtures are the
  supported offline path.
