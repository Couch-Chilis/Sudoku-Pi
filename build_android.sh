#!/usr/bin/env bash

cargo ndk -t arm64-v8a -t x86_64 -o app/src/main/jniLibs build --package sudoku-pi --release
