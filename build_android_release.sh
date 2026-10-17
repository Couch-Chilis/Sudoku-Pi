#!/usr/bin/env bash
#
# Just ensure KEYSTORE_PASSWORD env is set.

set -euo pipefail

./build_android.sh

KEYSTORE_PATH="$HOME/Nextcloud/android-keystore.jks" KEY_ALIAS=release-key KEY_PASSWORD="$KEYSTORE_PASSWORD" KEYSTORE_PASSWORD="$KEYSTORE_PASSWORD" ./gradlew :app:bundleRelease
