#!/usr/bin/env bash

set -eu
cross build --release --target x86_64-unknown-linux-musl
cp ../../target/x86_64-unknown-linux-musl/release/gamejam dist
strip dist/gamejam
tar -h -czf gamejam.tgz dist
