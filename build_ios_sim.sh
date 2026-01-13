#/usr/bin/env bash

cargo rustc --crate-type staticlib --lib --target aarch64-apple-ios-sim

cp target/aarch64-apple-ios-sim/debug/libsudoku_pi.a ios/Sudoku\ Pi/Frameworks/
