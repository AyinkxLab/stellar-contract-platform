$ErrorActionPreference = "Stop"

Set-Location (Join-Path $PSScriptRoot "..")

function Invoke-Observatory {
    param(
        [Parameter(ValueFromRemainingArguments = $true)]
        [string[]] $Arguments
    )

    if (Get-Command stellar-contract-platform -ErrorAction SilentlyContinue) {
        & stellar-contract-platform @Arguments
    } elseif (Test-Path "target/debug/stellar-contract-platform.exe") {
        & "target/debug/stellar-contract-platform.exe" @Arguments
    } else {
        & cargo run -q -p observatory-cli -- @Arguments
    }
}

Write-Host "== doctor"
Invoke-Observatory doctor

Write-Host "`n== inspect"
Invoke-Observatory inspect demo/fixtures/new.wasm --json

Write-Host "`n== spec"
Invoke-Observatory spec demo/fixtures/new.wasm

Write-Host "`n== diff"
Invoke-Observatory diff demo/fixtures/old.wasm demo/fixtures/new.wasm

Write-Host "`n== compat (non-zero exit is expected for incompatible changes)"
$compatExit = 0
try {
    Invoke-Observatory compat demo/fixtures/old.wasm demo/fixtures/new.wasm
    $compatExit = $LASTEXITCODE
} catch {
    $compatExit = $LASTEXITCODE
}
Write-Host "(compat exit code: $compatExit)"

Write-Host "`n== fingerprint"
Invoke-Observatory fingerprint demo/fixtures/new.wasm --json
