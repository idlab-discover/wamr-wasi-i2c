#!/bin/bash
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cargo build --target wasm32-wasip1 --manifest-path "${SCRIPT_DIR}/guest/Cargo.toml"
cp "${SCRIPT_DIR}/guest/target/wasm32-wasip1/debug/guest.wasm" "${SCRIPT_DIR}/host/test.wasm"
