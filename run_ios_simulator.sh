#/usr/bin/env bash

APP_NAME="Sudoku Pi"
BUNDLE_ID="nl.couch-chilis.sudoku-pi"

cargo bundle --target aarch64-apple-ios-sim
xcrun simctl boot "iPhone 14"  
open /Applications/Xcode.app/Contents/Developer/Applications/Simulator.app 
xcrun simctl install booted "target/aarch64-apple-ios-sim/debug/bundle/ios/$APP_NAME.app"
xcrun simctl launch --console booted "$BUNDLE_ID"
