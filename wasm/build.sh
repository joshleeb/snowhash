#!/usr/bin/env bash

set -e

release_flag=""
if [[ $* == *--release* ]]; then release_flag="--release"; fi

rm -rf dist

cargo +nightly build ${release_flag} --target wasm32-unknown-unknown \
    && wasm-bindgen target/wasm32-unknown-unknown/debug/snowhash_wasm.wasm --out-dir .
