#!/usr/bin/env sh

set -e

cargo fmt -- --check

cargo build --all-features
cargo build --all-features --release
