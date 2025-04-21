#!/usr/bin/env bash

set -eu

pushd ../../
cargo make install-scripts
popd
./gamejam