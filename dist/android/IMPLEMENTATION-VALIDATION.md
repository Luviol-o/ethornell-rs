# Local implementation / validation handoff — 2026-10-15

## Artifact

* Stripped runtime: `arm64-v8a/libethornell.so`, **26,063,304 bytes**.
* SHA-256: `d84bfc1edef62ab6ecf638adbcc1a1e39209287b1323af369a52e071a332546e`.
* Provenance: `arm64-v8a/libethornell.provenance.json`, build UTC
  `2026-10-15T12:08:38.410464+00:00`, baseline revision
  `5556c36b53c9af29a19ba507ffa431b19300f0e7`, **dirty=true**.
* API26 / arm64 / NDK28.0.13004108 / Rust1.96.0 / unwind / 16KB / NOW+RELRO /
  no-undefined, then NDK llvm-strip `--strip-unneeded`. Exact tool commands,
  flags, header/Cargo.lock/source hashes and unstripped hash are in provenance.
* Copied only this library and matching provenance to authorized TyranorNext
  `engine/src/main/jniLibs/arm64-v8a/`; neither destination existed beforehand.
  Header is byte-identical to `engine/src/main/cpp/ethornell_android.h`.
* `build.sh` local validation and independent TyranorNext
  `tools/ethornell/verify_native.py` both pass; see
  `arm64-v8a/static-validation.json` for full ELF/symbol/dynamic evidence.
  There are 14 defined ABI functions, no NativeActivity/android_main glue,
  16KB LOAD congruence/alignment and RELRO end, and allowed system NEEDED.

**Not public release:** license and source-publication clearance unresolved.
Dirty-source public release gate is intentionally not bypassed. No commit,
push, public issue, app install or game asset publication performed by this
session. Device/APK validation belongs to the parent's independent acceptance.

## Review findings closed in source

| Finding | Implementation / regression evidence | Remaining limits |
|---|---|---|
| Final reader/list/tree TOCTOU | Resource scan and recursion use retained directory FDs, openat/nofollow, bounded enumeration; restricted path helpers return None, not authorized Paths. Root replacement test reads/writes original FDs. | Not a sandbox against another privileged same-UID process moving already-open directories; Android isolated process alone is not security isolation. |
| External bitmap import | Restricted 90:C4/C5 reject unsupported incomplete descriptor/export semantics. | No claim of this feature. |
| Wallpaper command bypass | B0:F0 rejects at host routing and native handler; final host_dialog function also refuses under policy. | Desktop behavior preserved. |
| Writer lock replacement | All `.ethornell-*` path components reserved; write/delete/rename both ends denied; enumerations hide them; flock remains held after attempts in test. | Same-UID external non-engine mutation is outside guest policy. |
| Overlay consistency / virtual roots | exists/size/read resolve user overlay consistently; strict explicit roots; root in archive slot supported. Tests cover case, differing content/size, no cross-root fallback. | No automatic original save migration. |
| Case/family compatibility | Bounded FD case resolution, exact first, ambiguous folded alias rejected; loaded archive family alias and empty-first-entry lookup tested. | Only ASCII folding follows desktop contract. |
| Precise capability dispatch | 80:FD,81:3C and other game/query calls fall through; shell/installer/chooser/input modal unsupported calls Err. Actual message/confirm uses VM continuation. | No arbitrary desktop integration. |
| UserData mkdir/probe | mkdirat in user root; FD probe; SysApi probe override. Full restricted native mkdir/write/read/list/rename/delete/reentry test passes. | Copy/permissions/directory removal explicit unsupported. |
| Windows regression | Unix-only policy implementation is cfg-isolated; non-Unix host methods fail closed without Unix imports. | Windows native build not executed; Windows host-driven unsupported. |
| Archive allocation bypass | ResourceBudget::android at final manager construction, including bootstrap; 15 archive tests pass. | Does not bound every upstream VM/image/audio opcode allocation or process RSS. |
| Public Rust step safety | HostRuntime.step installs file policy and shared font internally; external callers do not need ABI scope. | Other public methods do not execute guest filesystem logic. |
| Atomic write durability semantics | File sync, rename, directory sync; parent mkdir sync; post-rename sync errors state commit/durability uncertainty. | No false promise that old destination survives after successful rename. |
| Attach ownership | Candidate commits only after audio readiness; injected failure drops candidate before returning; proposed generation is not consumed. | Actual Surface stress/device/audio fault injection acceptance remains parent's. |
| Timing | Sub-ms carry, raw audio elapsed independent from >500ms engine normalization; 60/90/120Hz test bounds error and single-pass gating. | No native-call preemption guarantee. |
| Input cancellation | Clear held queues, deferred graph activation, release suppression, double-click history. Outside release cancels, never UP at old content point. | Real multitouch/physical input matrix remains unverified here. |
| Stale work poisoning | Stale surface/request/clock errors are rejected but do not poison; tests show a later step still works. Genuine VM/GPU/audio/policy errors and panic poison. | ABI error classification still uses internal error categories/messages, not a public Rust typed error enum. |
| CPU font hot path | Explicit session FontArc for run/bootstrap/headless/desktop+Android CPU/GPU; glyph path no repeated disk IO; CPU coverage/raster budget recorded and checked inside scope. | OEM font coverage remains device-dependent and errors explicitly. |
| Duplicate presenter / audio errors | Single shared presenter; checked host audio play/pause and frame driver propagate Result; required missing backend fails before VM. | Legacy stop/volume/panning void APIs cannot propagate all backend queue errors. |
| Named archive collision/empty lookup | True named archive reads bypass loose files, include first entry for empty name and family aliases; collision fixture passes. | None for this contract. |
| Conflicting virtual root prefixes | Cross-root file prefix rejected even when archive slot explicitly chooses root; exists/size/read tests. | Same-root full and relative names accepted. |
| Directory read errors | readdir resets/checks errno for EOF versus IO error, closes DIR/FD via RAII. | Injected kernel EIO not executed. |
| Root ancestor nofollow | Canonical host alias resolved once; every resulting ancestor opened from / via openat/O_DIRECTORY/O_NOFOLLOW. Retain FDs throughout session. | Host supplies authorization; no automatic parent/cwd expansion. |
| 81:3C directory semantics | Existing SysApi.directory_exists joined with file_exists at VM 81:3C; app uses safe FD metadata. VM202 tests pass. | No new trait/protocol. |

## Host test result

`647 passed, 0 failed`: ABI3, app403, archive15, audio6, render18, VM202.
Run command is in `platform/android/BUILD.md`; log:
`/private/var/folders/f7/8jzwqrf15g56w446cgw2txb40000gn/T/opencode/ethornell-final-host-tests.log`.

Two old tests are explicitly filtered (not passed):
`game_root_is_exposed_as_an_absolute_user_data_root` and
`get_user_data_root_paths_resolve_as_loose_files` require the absent repository
`testcase/`. Two existing app tests remain ignored: optional game_input_repro
(requires authorized writable game path) and the slow full selector audit.
Warnings from existing recovered/runtime code remain; this is not warning-free
or full workspace/Windows/device validation.

## Desktop sample evidence

The desktop CLI was rebuilt from the implementation tree and run with GBK,
1,200 headless frames, realtime=0, dedicated temporary staging. Actual PNG was
visually inspected: Chinese title/menu renders. This is limited desktop baseline
evidence, **not** Android success, PC save compatibility, playable route, or audio
validation. Only staging/BGI.gdb changed; original sample and ZIP untouched.
Latest non-strict log still records existing stubs **91:00, 91:09, 81:00,
81:D8**. Thus the title baseline must not be described as zero-stub/strict
compatibility. Final rebuilt desktop executable SHA-256:
`78621ca5874ab82244a1eed3b590587da3afd6013cce96b942fa01b2c1e9313d`.

Temporary evidence names: `ethornell-current-source-amairo.log`, `.png`,
`-before.json`, `-after.json` in the approved OpenCode temp directory. Screenshot
SHA-256 `43f008cd50fdb05ed50f2bec8bbde10d8ef6ddcc031629bb6601434727305398`.

## Storage and source package

Precise writable tree and synchronous flush/barrier contract are in
`platform/android/BUILD.md`: separate user_root only, exclude internal names;
never mirror resource root or automatically import original UserData. A serial
engine command ACK after writes/destroy completes synchronous IO, not an
unacknowledged pause callback. Post-rename sync failure is commit uncertainty.

Local source bundle `dist/source/ethornell-android-source.tar.gz` contains 173
source/build files, no raw/target/dist/game/font/binary reference. Cargo.lock and
untracked implementation sources included; dependencies remain Cargo.lock-pinned,
not vendored. `source-manifest.json` contains file/artifact hashes; working-tree
patch is alongside it. Neither artifact grants public redistribution rights.
Source tarball SHA-256:
`a134363ec5707f3113d2805fd76e61fabd1b72c91cbb3b0c4aad97b0c654d6e7`.
Working-tree patch SHA-256:
`c9a48d3ab682787b10f42bc1199098bf894e0ea609a894f0f399bbb460760edf`.
This handoff file is generated evidence under dist, not a changed compilation
input. The source hashes and copied library/provenance were checked unchanged
after final desktop execution.
