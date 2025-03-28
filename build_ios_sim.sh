#/usr/bin/env bash

cargo rustc --crate-type staticlib --lib --release --target aarch64-apple-ios-sim

cp target/aarch64-apple-ios-sim/release/libsudoku_lib.a ios/Sudoku\ Pi/Frameworks/
