#!/bin/sh
#
# Xcode Cloud runs this script after it has cloned the repository. Xcode Cloud
# images don't ship the Rust toolchain, so it is installed here; the library
# itself is built by `ios/scripts/build-rust.sh`, which runs as a pre-build
# script phase of the Xcode targets.

set -euo pipefail

cd "${CI_PRIMARY_REPOSITORY_PATH:-${CI_WORKSPACE:-.}}"

if ! command -v cargo >/dev/null 2>&1; then
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
    | sh -s -- -y --default-toolchain stable --profile minimal
fi

# The targets that are needed are added by the build script, which knows the
# platform that is being built.
. "$HOME/.cargo/env"
rustup --version
