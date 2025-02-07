#!/usr/bin/env bash

set -eu
cross build --release --target x86_64-pc-windows-gnu
cp ../../target/x86_64-pc-windows-gnu/release/gamejam.exe dist
strip dist/gamejam.exe
tar -h -czf gamejam.tgz dist