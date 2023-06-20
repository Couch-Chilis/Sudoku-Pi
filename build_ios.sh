#/usr/bin/env bash

pushd lib
cargo rustc --crate-type staticlib --lib --release --target aarch64-apple-ios
popd

cp target/aarch64-apple-ios/release/libsudoku_lib.a ios/Sudoku\ Pi/Frameworks/
