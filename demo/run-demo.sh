#!/usr/bin/env bash
set -u

cd "$(dirname "$0")/.."

run() {
  if command -v stellar-contract-platform >/dev/null 2>&1; then
    stellar-contract-platform "$@"
  elif [ -x target/debug/stellar-contract-platform ]; then
    target/debug/stellar-contract-platform "$@"
  elif [ -x target/debug/stellar-contract-platform.exe ]; then
    target/debug/stellar-contract-platform.exe "$@"
  elif command -v cargo >/dev/null 2>&1; then
    cargo run -q -p observatory-cli -- "$@"
  else
    echo "error: neither stellar-contract-platform nor cargo was found on PATH" >&2
    exit 1
  fi
}

echo "== doctor"
run doctor

echo
echo "== inspect"
run inspect demo/fixtures/new.wasm --json

echo
echo "== spec"
run spec demo/fixtures/new.wasm

echo
echo "== diff"
run diff demo/fixtures/old.wasm demo/fixtures/new.wasm

echo
echo "== compat (non-zero exit is expected for incompatible changes)"
run compat demo/fixtures/old.wasm demo/fixtures/new.wasm || true

echo
echo "== fingerprint"
run fingerprint demo/fixtures/new.wasm --json
