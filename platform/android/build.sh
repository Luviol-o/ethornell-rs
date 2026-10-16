#!/bin/bash
# Reproducible API26/arm64 host library; never invokes winit activity glue.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
NDK="${ANDROID_NDK_HOME:-$HOME/Library/Android/sdk/ndk/28.0.13004108}"
test -f "$NDK/source.properties"
grep -Eq 'Pkg.Revision *= *28\.0\.13004108' "$NDK/source.properties" || { echo 'NDK 28.0.13004108 is required' >&2; exit 1; }
case "$(uname -s)" in
  Darwin) HOST_TAG=darwin-x86_64 ;;
  Linux) HOST_TAG=linux-x86_64 ;;
  *) echo 'Supported build hosts: macOS and Linux' >&2; exit 1 ;;
esac
BIN="$NDK/toolchains/llvm/prebuilt/$HOST_TAG/bin"
test -x "$BIN/aarch64-linux-android26-clang"
export ETHORNELL_BUILD_HOST_TAG="$HOST_TAG"
export CC_aarch64_linux_android="$BIN/aarch64-linux-android26-clang"
export CXX_aarch64_linux_android="$BIN/aarch64-linux-android26-clang++"
export AR_aarch64_linux_android="$BIN/llvm-ar"
export CC="$CC_aarch64_linux_android" CXX="$CXX_aarch64_linux_android" AR="$AR_aarch64_linux_android"
export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER="$CC"
unset RUSTFLAGS CARGO_ENCODED_RUSTFLAGS
export CARGO_TARGET_AARCH64_LINUX_ANDROID_RUSTFLAGS='-C panic=unwind -C link-arg=-Wl,-z,max-page-size=16384 -C link-arg=-Wl,-z,common-page-size=16384 -C link-arg=-Wl,-z,relro -C link-arg=-Wl,-z,now -C link-arg=-Wl,--no-undefined'
cd "$ROOT"
cargo build -p ethornell-android --target aarch64-linux-android --release --locked -j "${CARGO_BUILD_JOBS:-2}"
LIB="$ROOT/target/aarch64-linux-android/release/libethornell.so"
"$BIN/llvm-readelf" -lW "$LIB" > "$LIB.program-headers.txt"
"$BIN/llvm-readelf" -dW "$LIB" > "$LIB.dynamic.txt"
"$BIN/llvm-nm" -D "$LIB" > "$LIB.symbols.txt"
if grep -q 'android_main' "$LIB.symbols.txt"; then echo 'unexpected activity glue' >&2; exit 1; fi
python3 "$ROOT/platform/android/verify_elf.py" "$LIB" "$BIN/llvm-readelf"
DIST="$ROOT/dist/android/arm64-v8a"
mkdir -p "$DIST"
cp "$LIB" "$DIST/libethornell.so"
"$BIN/llvm-strip" --strip-unneeded "$DIST/libethornell.so"
python3 "$ROOT/platform/android/verify_elf.py" "$DIST/libethornell.so" "$BIN/llvm-readelf"
cp "$ROOT/platform/android/include/ethornell_android.h" "$DIST/ethornell_android.h"
python3 - "$ROOT" "$DIST/libethornell.so" "$NDK" "$LIB" "$BIN/llvm-strip" <<'PY'
import sys,pathlib,subprocess,hashlib,json,datetime,os
root,lib,ndk,unstripped,strip=map(pathlib.Path,sys.argv[1:])
def cmd(*args): return subprocess.check_output(args,cwd=root,text=True).strip()
tracked=cmd('git','ls-files','--cached','--others','--exclude-standard').splitlines()
# Exclude commercial reverse-engineering reference, output, and game/font
# assets. Include every source/build input in the Cargo workspace, not target.
def allowed(name):
 p=pathlib.PurePosixPath(name)
 return not any(part in {'target','dist','raw','.git','testcase','reverse','out'} for part in p.parts) and p.suffix.lower() not in {'.so','.dll','.exe','.ttf','.ttc','.otf','.arc','.sud','.gdb','.cad','.bcd','.zip','.7z'}
files={name:hashlib.sha256((root/name).read_bytes()).hexdigest() for name in tracked if allowed(name) and (root/name).is_file()}
manifest={'abi':1,'target':'aarch64-linux-android','api':26,'ndk':'28.0.13004108','rustc':cmd('rustc','--version'),
 'source_revision':cmd('git','rev-parse','HEAD'),'dirty':bool(cmd('git','status','--porcelain')),
 'source_file_sha256':files,'cargo_lock_sha256':hashlib.sha256((root/'Cargo.lock').read_bytes()).hexdigest(),
 'header_sha256':hashlib.sha256((root/'platform/android/include/ethornell_android.h').read_bytes()).hexdigest(),
 'build_command':'cargo build -p ethornell-android --target aarch64-linux-android --release --locked -j 2',
 'rustflags':os.environ['CARGO_TARGET_AARCH64_LINUX_ANDROID_RUSTFLAGS'], 'host_tag':os.environ['ETHORNELL_BUILD_HOST_TAG'],
 'cc':os.environ['CC'], 'cxx':os.environ['CXX'], 'ar':os.environ['AR'],
 'strip_command':[str(strip),'--strip-unneeded','libethornell.so'], 'strip_version':cmd(str(strip),'--version'),
 'unstripped_library_sha256':hashlib.sha256(unstripped.read_bytes()).hexdigest(),
 'library_sha256':hashlib.sha256(lib.read_bytes()).hexdigest(),'library_bytes':lib.stat().st_size,
 'built_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),
 'publication':'BLOCKED: dirty local technical validation; license/source publication clearance not established',
 'validation':'ELF/link checks only; not an Android game/device compatibility claim'}
lib.with_suffix('.provenance.json').write_text(json.dumps(manifest,indent=2)+'\n')
print(json.dumps({k:manifest[k] for k in ['library_sha256','library_bytes','source_revision','dirty']},indent=2))
PY
