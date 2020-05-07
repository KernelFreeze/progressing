#!/usr/bin/env bash

set -e

cargo fmt -- --check

# build
CUR_DIR="$(dirname "$(pwd)"/"${0}")"
"${CUR_DIR}/build.sh"

# test
cargo test
cargo run --example loops
cargo run --example simple
