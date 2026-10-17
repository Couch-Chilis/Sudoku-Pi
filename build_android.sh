#!/usr/bin/env bash

set -euo pipefail

RUSTFLAGS="-C link-arg=-Wl,-z,max-page-size=16384" cargo ndk -t arm64-v8a -t x86_64 -o app/src/main/jniLibs build --package sudoku-pi --release
