#/usr/bin/env bash

cargo rustc --crate-type staticlib --lib --release --target aarch64-apple-ios

cp target/aarch64-apple-ios/release/libsudoku_pi.a ios/Sudoku\ Pi/Frameworks/
