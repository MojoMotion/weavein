#!/bin/bash
set -eu

cargo build --release --lib --target wasm32-unknown-unknown

wasm-bindgen target/wasm32-unknown-unknown/release/weavein.wasm \
--out-dir webapp --no-modules --no-typescript
