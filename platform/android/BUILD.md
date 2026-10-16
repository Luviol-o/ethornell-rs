# Android host implementation and local validation

## Status / authorization

This is a **dirty, local technical validation build**, not a public release.
The repository's workspace metadata says MPL-2.0, but that is not an audit of
every source file, reverse-engineering provenance, dependency, codec, font or
redistribution right. Public distribution is **blocked pending license/source
clearance**. No commercial reference, game, game save, system font or extracted
asset is authorized for publication by this work. No commit/push is performed.

Android ABI is defined exclusively by `include/ethornell_android.h` (v1, 14
functions). `include/ABI.md` documents thread, context and Surface ownership.
API26+, arm64 only; renderer and audio have their own independently delivered
implementations. Android app/JNI/APK testing is coordinated in TyranorNext,
not performed by this build script. Build success is not a gameplay claim.

## Build and outputs

Prerequisites: Rust 1.96.0 (actual tested version, not an MSRV promise), Cargo,
Python 3, `aarch64-linux-android` Rust target, NDK **28.0.13004108**, macOS or
Linux x86_64 NDK host tools. On Apple Silicon the NDK macOS host tag is still
`darwin-x86_64`. Linux host support is scripted, not executed in this session.

```sh
ANDROID_NDK_HOME=/path/to/ndk/28.0.13004108 bash platform/android/build.sh
```

The script selects the explicit API26 clang/clang++ target, fixed unwind,
max/common 16KB page sizes, RELRO/NOW and `--no-undefined`; Cargo uses `--locked`
and two jobs by default. It never enables winit Android activity glue.

* `target/aarch64-linux-android/release/libethornell.so`: unstripped debugging
  artifact, not the APK input. Old `libethornell_app.so` was an investigation
  probe and is never selected.
* `dist/android/arm64-v8a/libethornell.so`: `llvm-strip --strip-unneeded` APK
  candidate. **This exact byte sequence** must match the APK library hash.
* Adjacent `libethornell.provenance.json`: revision, dirty flag, full workspace
  source/build-input hashes, Cargo.lock/header hash, fixed flags/tool commands,
  strip tool version, stripped/unstripped hashes, time and publication blocker.
* `*.program-headers.txt`, `*.dynamic.txt`, `*.symbols.txt` adjacent to the
  unstripped library retain ELF evidence.

`verify_elf.py` checks AArch64, all 14 defined exports, no activity glue, LOAD
congruence/alignment and RELRO 16KB end boundary. TyranorNext's independent
`tools/ethornell/verify_native.py` additionally checks the exact ABI surface,
NEEDED and packaging/provenance. Static ELF success is not proof of 16KB
runtime loading, GPU, audio, JNI, or game compatibility.

## Implemented file/host boundary

* `host_runtime.rs`: serial, thread-owned runtime; no desktop event loop or
  unsafe Send/Sync. No Surface/pause/pending request halts narrative ticks.
  Monotonic ~16ms gate runs one scheduler pass, carries fractional milliseconds,
  keeps raw audio elapsed separate from the engine's >500ms reset rule.
* `file_policy.rs`: scoped capability boundary, opened resource/user directory
  descriptors, component `openat`/`O_NOFOLLOW`, fresh FD directory enumeration,
  64 levels / 100,000 visited entries / 10,000 result budget. No checked path is
  reopened by restricted path helpers. Windows desktop compiles the fail-closed
  policy stub; Windows host-driven sessions are not supported.
* Unqualified loose names use user overlay first, then resource when absent.
  True named archive arguments (including the empty first-entry form) bypass
  loose overlay collisions and are served by the indexed archive descriptor.
  `bgi-user\` and `bgi-resource\` explicitly select one root with no cross-root
  fallback. Getters return virtual roots rather than host UTF-8 paths encoded
  into SJIS. ASCII case folding and indexed `data01xxx.arc` family aliases are
  retained; ambiguous case-fold aliases are rejected (an exact match wins).
  Explicit archive-slot roots cannot be overridden by a conflicting file-slot
  virtual prefix. Canonical host root aliases are bound with `openat`/nofollow
  for **every** ancestor; directory iteration distinguishes EOF from errors.
  Named archive entries keep archive semantics. Bootstrap namespace
  selection cannot expand authorization to parent/cwd/absolute roots.
* ResourceManager is created from the exact authorized descriptors with
  `ResourceBudget::android()`: packed/decoded 256MiB each, cumulative index
  64MiB / 262,144 entries / 1,024 archives (host scan currently caps FDs at 512).
  Decoder budgets are checked **before** final allocations, including bootstrap.
* Write/delete/rename operate only inside user_root; both rename ends are
  resolved by user FDs. `.ethornell-*` path components are reserved against guest
  mutation/reads and hidden from enumeration, protecting the writer flock.
  Atomic writes are capped at 64MiB. Directory creation and write probes are
  safe user-root operations; copy/permissions/standalone removal remain explicit
  unsupported capabilities. External bitmap import/export are unsupported,
  not fabricated descriptor successes.
* No direct VM filesystem fallback. Shell/process/installer/file chooser/
  wallpaper are explicitly rejected; safe game queries such as 80:FD and
  81:3C continue through their normal APIs. Actual VM continuations handle
  message/confirmation requests once, using strings normalized in their owning
  VM (including child VM); the app never reads child VM memory.
* Runtime GPU/VM/audio errors poison ABI sessions; invalid/stale surface,
  response or clock rejection does not. Failed attach drops the incoming
  renderer before returning an error, so JNI may release the borrowed window.
  Clear/cancel removes deferred pointer activation and double-click history;
  black-bar release cancels rather than activating an old content coordinate.

## Exact user-data / mirror contract

The writable set is **all valid guest relative files under the explicit
user_root**, excluding any `.ethornell-*` component. It is not inferred as the
whole resource/game root, and no game-specific save format or PC compatibility
is claimed. A host mirror/export may copy only this separate user tree, excluding
reserved files, after a serial engine-thread barrier. It must reject links and
keep active-session mutual exclusion. `resource/UserData/*` is **not** copied or
migrated automatically. Normal resource reads may see those files as read-only
resources when no user overlay exists; explicit user-file reads do not import
them. To reuse an original save requires an explicit, user-authorized import
after format/compatibility investigation.

Every user write is synchronous on the engine thread: create missing parents
(sync each containing directory), temporary file, write, file `sync_all`, atomic
rename within the destination directory, then directory `sync_all`. There are
no background save writers. A command queued after the last write, or completed
destroy on that same engine thread, is an IO completion barrier. `pause` alone
does **not** prove previously queued work completed; await its thread ACK and
use an explicit ordered barrier before mirror/export. Audio workers do not
write user files. LMK/kill/abort/OOM does not promise flush or save success.

Before rename failure the old destination is preserved. After successful rename,
a directory-sync error means **commit occurred, durability uncertain**, never
"old save guaranteed preserved". Rename also reports post-commit sync uncertainty.
Filesystem/device implementations can weaken durability despite successful
syscalls; external removal and permission loss still fail at actual operations.

## Host tests / exclusions / limits

```sh
cargo test -p ethornell-app -p ethornell-android -p ethornell-archive \
  -p ethornell-vm -p ethornell-render -p ethornell-audio --lib --locked -j 2 -- \
  --skip game_root_is_exposed_as_an_absolute_user_data_root \
  --skip get_user_data_root_paths_resolve_as_loose_files
```

Two old user-files tests require a repository `testcase/` directory that is not
present; they are excluded by name, not marked passed. Existing slow whole-call
catalog / opt-in GPU tests remain ignored. Current exact totals and hashes are
in the final session handoff and provenance (not hardcoded into a moving source
document). These are macOS host tests, not Android-native unit execution.

New regression coverage includes reserved locks, full restricted
exists/size/read/write/list/mkdir/probe/rename/delete/reentry, virtual-root strict
selection, case-insensitive overlay, escapes, symlink parents, replaced root
paths, unwind scope restoration, real continuations, precise host routing,
elapsed carry, and injected attach readiness/audio failure dropping the borrow.

Remaining validation: actual driver/Surface stress, repeated detach and slow
native-call watchdog, 16KB device runtime, output routing/focus, long soak and
real save/load/manual sample matrix; Windows build is not executed here.
Host budgets are admission limits, not a promise that every upstream opcode
allocation/CPU duration or total process memory is bounded. Audio stop/volume/
panning old void APIs do not surface every backend queue error; checked host
play/pause APIs do. MPEG is video-only; complete SWF audio/live playback,
WMV/BMV/MP4/full IME, CDDA device behavior and arbitrary filesystem/desktop
integrations are not claimed. Silent mode is explicit rather than a masked
audio initialization failure. No blanket "all BGI games supported" statement.

### Sample desktop evidence (2026-10-15, local only)

The current-source macOS desktop CLI was rebuilt with `cargo build -p
ethornell-cli --release --locked -j 2` and run for 1,200 headless frames with
GBK encoding, no realtime delay, using a **separate staging copy** of the
authorized Amairo Chocolate Chinese sample. The result was inspected visually:
the Chinese title image/menu rendered, not merely exit code zero or a black
frame. This is the desktop baseline only: not Android rendering/audio evidence,
not a save/load loop or PC save-format interoperability claim. Staging changed
only `BGI.gdb`; the user's original ZIP and sample directory were untouched.
Logs, before/after hashes and screenshot remain in the approved temporary
validation directory, not in this source bundle. No sample assets are bundled.

## Source reproduction

Run `python3 platform/android/source_snapshot.py` after the final source build.
It creates a local source-only tarball, patch, and hashes under `dist/source/`.
The snapshot contains the self-contained workspace source/build files and lock
file (Cargo registry/git dependencies are still resolved by Cargo.lock, not
vendored or licensed for publication here). Excludes `src/raw`, commercial
reference, target/dist, games/saves, fonts, and binary/debug reference material.
Untracked implementation files are included. The patch alone cannot recreate
new untracked files; the snapshot is authoritative for those. The snapshot does
not make dirty source a public release or resolve licensing blockers.
