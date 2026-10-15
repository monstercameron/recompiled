#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
python3 scripts/catalog.py export
cargo build --locked --release --target wasm32-unknown-unknown
wasm-bindgen --target web --out-dir docs/pkg target/wasm32-unknown-unknown/release/recompiled.wasm
