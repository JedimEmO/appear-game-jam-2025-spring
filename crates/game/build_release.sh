#!/usr/bin/env bash

set -eu
cargo build --release
cp ../../target/release/gamejam .
