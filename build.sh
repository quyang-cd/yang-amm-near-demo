#!/bin/bash
TARGET="${CARGO_TARGET_DIR:-./target}"
set -e
cd "$(dirname $0)"

cargo build --all --target wasm32-unknown-unknown --release
cp $TARGET/wasm32-unknown-unknown/release/yang_amm_near_demo.wasm ./res/
cp $TARGET/wasm32-unknown-unknown/release/fungible_token.wasm ./res/
