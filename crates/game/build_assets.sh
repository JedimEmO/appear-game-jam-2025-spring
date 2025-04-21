#!/usr/bin/env bash

set -eu

pushd ../../
cargo make assets
popd
./gamejam