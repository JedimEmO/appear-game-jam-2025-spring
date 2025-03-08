#!/usr/bin/env bash
set -eu

cargo build --release
cp target/wasm32-unknown-unknown/release/*.wasm ../game/assets/scripts/