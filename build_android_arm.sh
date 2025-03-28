#/usr/bin/env bash

cargo ndk -t arm64-v8a -o app/src/main/jniLibs build --package sudoku-pi
