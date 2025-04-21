#!/usr/bin/env bash

set -eu
cargo build
cp ../../target/debug/gamejam .