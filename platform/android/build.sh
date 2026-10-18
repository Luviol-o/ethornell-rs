#!/usr/bin/env bash
# Builds libethornell_mobile.so for each ABI with cargo-ndk, then the APK with Gradle.
#
#   platform/android/build.sh            # debug APK
#   VARIANT=release platform/android/build.sh
#   ABIS=arm64-v8a platform/android/build.sh
#
# Needs: rustup, cargo-ndk (installed when missing), a JDK 17+, the Android
# SDK (ANDROID_HOME) with an NDK (ANDROID_NDK_HOME, or the newest under
# $ANDROID_HOME/ndk).
set -euo pipefail

ANDROID_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$ANDROID_DIR/../.." && pwd)"
VARIANT="${VARIANT:-debug}"
ABIS="${ABIS:-arm64-v8a x86_64}"
API="${ANDROID_API:-26}"
JNI_LIBS="$ANDROID_DIR/app/src/main/jniLibs"

export ANDROID_HOME="${ANDROID_HOME:-${ANDROID_SDK_ROOT:-$HOME/Library/Android/sdk}}"
[[ -d "$ANDROID_HOME" ]] || { echo "ANDROID_HOME is not set" >&2; exit 1; }
if [[ -z "${ANDROID_NDK_HOME:-}" ]]; then
  ANDROID_NDK_HOME="$(ls -d "$ANDROID_HOME"/ndk/*/ 2>/dev/null | sort -V | tail -n 1)"
  ANDROID_NDK_HOME="${ANDROID_NDK_HOME%/}"
fi
[[ -f "${ANDROID_NDK_HOME:-}/source.properties" ]] || { echo "no NDK found; set ANDROID_NDK_HOME" >&2; exit 1; }
export ANDROID_NDK_HOME
case "$(uname -s)" in
  Darwin) HOST_TAG=darwin-x86_64 ;;
  Linux) HOST_TAG=linux-x86_64 ;;
  *) echo "build on macOS or Linux" >&2; exit 1 ;;
esac
NDK_BIN="$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/$HOST_TAG/bin"

command -v cargo-ndk >/dev/null || cargo install cargo-ndk --locked
targets=()
for abi in $ABIS; do
  case "$abi" in
    arm64-v8a) targets+=(aarch64-linux-android) ;;
    x86_64) targets+=(x86_64-linux-android) ;;
    *) echo "unsupported ABI: $abi" >&2; exit 1 ;;
  esac
done
rustup target add "${targets[@]}" >/dev/null

# 16 KB pages (required from Android 15), full RELRO, and no undefined symbols.
LINK_FLAGS='-C link-arg=-Wl,-z,max-page-size=16384 -C link-arg=-Wl,-z,common-page-size=16384 -C link-arg=-Wl,-z,relro -C link-arg=-Wl,-z,now -C link-arg=-Wl,--no-undefined'
export CARGO_TARGET_AARCH64_LINUX_ANDROID_RUSTFLAGS="$LINK_FLAGS"
export CARGO_TARGET_X86_64_LINUX_ANDROID_RUSTFLAGS="$LINK_FLAGS"

rm -rf "$JNI_LIBS"
abi_args=()
for abi in $ABIS; do abi_args+=(-t "$abi"); done
cd "$ROOT"
cargo ndk "${abi_args[@]}" -P "$API" -o "$JNI_LIBS" build -p ethornell-mobile --release --locked

for abi in $ABIS; do
  test -f "$JNI_LIBS/$abi/libethornell_mobile.so"
done
if [[ " $ABIS " == *" arm64-v8a "* ]]; then
  python3 "$ANDROID_DIR/verify_elf.py" "$JNI_LIBS/arm64-v8a/libethornell_mobile.so" "$NDK_BIN/llvm-readelf"
fi

cd "$ANDROID_DIR"
task="assemble$(printf %s "${VARIANT:0:1}" | tr a-z A-Z)${VARIANT:1}"
./gradlew --no-daemon ":app:$task"
ls -1 "$ANDROID_DIR/app/build/outputs/apk/$VARIANT/"*.apk
