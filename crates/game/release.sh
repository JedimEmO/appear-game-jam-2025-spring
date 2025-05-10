#!/usr/bin/env bash

set -eu

cargo clean
cross build --release --target x86_64-pc-windows-gnu
cp ../../target/x86_64-pc-windows-gnu/release/gamejam.exe dist

cargo clean
cargo build --release
cp ../../target/release/gamejam dist

strip dist/gamejam.exe
strip dist/gamejam
tar -h -czf gamejam.tgz dist

tar -h -czf gamejam.tgz dist
