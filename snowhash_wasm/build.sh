#!/usr/bin/env sh

set -e

cargo +nightly build --target wasm32-unknown-unknown \
    && wasm-bindgen ../target/wasm32-unknown-unknown/debug/snowhash_wasm.wasm --out-dir .
