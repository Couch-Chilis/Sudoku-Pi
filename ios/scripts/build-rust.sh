#!/bin/sh
#
# Builds the Rust static library that the Xcode targets link against.
#
# This runs as a pre-build script phase of both the iOS and the macOS targets,
# so that building the app also builds the library. The result is copied to
# `ios/build/libsudoku_pi.a`, which is where the target looks for it.

set -euo pipefail

# Xcode's build phases don't inherit the login shell's environment, so the Rust
# toolchain is added to the path here. `ci_scripts/ci_post_clone.sh` installs it
# on Xcode Cloud; locally, rustup puts the same file there.
if [ -f "$HOME/.cargo/env" ]; then
    . "$HOME/.cargo/env"
fi

# The Xcode project lives in `ios/`, so the repository root is its parent.
cd "$SRCROOT/.."

case "${PLATFORM_NAME:-iphoneos}" in
    iphoneos)
        rustup target add aarch64-apple-ios
        cargo build --locked --release --lib --target aarch64-apple-ios
        LIBRARY=target/aarch64-apple-ios/release/libsudoku_pi.a
        ;;
    iphonesimulator)
        rustup target add aarch64-apple-ios-sim
        cargo build --locked --release --lib --target aarch64-apple-ios-sim
        LIBRARY=target/aarch64-apple-ios-sim/release/libsudoku_pi.a
        ;;
    *)
        echo "Unsupported platform '${PLATFORM_NAME}'" >&2
        exit 1
        ;;
esac

# The library is copied to a fixed location, so that the targets' library search
# path doesn't depend on the platform that is being built.
mkdir -p "$SRCROOT/build"
cp "$LIBRARY" "$SRCROOT/build/libsudoku_pi.a"
