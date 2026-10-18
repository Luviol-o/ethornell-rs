# Device policy create failure follow-up

Initial candidate `d84bfc1edef62ab6ecf638adbcc1a1e39209287b1323af369a52e071a332546e`
loaded via RTLD_NOW / exported ABI on parent-tested Android environments, but
real API36 fixture creation failed with -5 before GPU setup. Independent JNI
harness inspection confirmed separate readable/writable sibling roots.

Root bind had used O_RDONLY for all canonical ancestors. Android app sandbox
ancestors can grant traversal but deny directory listing. The repair uses
O_PATH|O_DIRECTORY|O_NOFOLLOW|O_CLOEXEC for Linux/Android ancestors, then opens
the final root O_RDONLY and compares its device/inode to the canonical target.
macOS retains its previous O_RDONLY traversal. Guest authorization, nofollow,
root separation and retained FD IO are unchanged.

Bounded ABI diagnostics expose only stage and errno, never a path:

| Stage | Meaning |
|---|---|
| 101 / 111 | resource / user canonical alias resolution |
| 102 / 112 | canonical target metadata |
| 103 / 113 | traversal anchor open |
| 104 / 114 | canonical ancestor component open |
| 105 / 115 | final root readable directory open |
| 106 / 116 | final descriptor metadata |
| 107 / 117 | final descriptor identity mismatch |
| 120 | resource/user containment overlap |
| 201 | writer lock file open |
| 202 | exclusive writer flock |
| 301 | initial resource tree walk |

New regressions cover canonical aliases preserving directory inode, safe
numeric diagnostic redaction, and execute-only ancestor behavior on Linux/
Android (that platform-specific test is not run by macOS host tests).
Logs: `ethornell-policy-device-fix-tests.log` and
`ethornell-policy-device-fix-build.log` in the approved OpenCode temp directory.
Parent must rerun instrumentation before calling the device issue resolved.

## Replacement candidate

* Built `2026-10-15T12:32:44.879372+00:00`, stripped 26,062,600 bytes.
* SHA-256 `5c15c4219b61206487335df3d6c509016317c123f404b89c7c1210350de91d64`.
* Matching library/provenance replaced only this session's exact previous pair
  in TyranorNext jniLibs after hash checks. Independent static validation passes.
* Host tests: **649 passed, zero failed** (ABI4/app404/archive15/audio6/render18/
  VM202), same two explicit missing-testcase filters and two existing ignored.
* Source bundle updated:
  `464b3d5968242c77c8654081441412e7a4ee521e0a286d190e9fbc8a96229773`.
  The untracked implementations live in the source bundle; tracked-only patch
  remains unchanged because these fixes are in new/untracked Rust files.
* Root guest permissions were not broadened. Linux-specific execute-only
  regression compiles for Linux/Android but is not executed on macOS.

## Parent device acceptance update

Parent reports the following actual instrumentation results for the frozen
`5c15c4219b61206487335df3d6c509016317c123f404b89c7c1210350de91d64` candidate:

* API36 physical arm64 device: all four core fixture methods pass; optional
  supplied-game method skipped.
* API35 actual 16KB-page environment: all four core fixture methods pass;
  optional supplied-game method skipped.
* These core methods include real RGBA checker quadrant readback, Surface
  reattachment, stale generation rejection followed by continued valid use,
  script-driven Closed, supplementary Unicode paths, thread and stale handles.
* API26: runtime load/ABI and create/non-BMP tests pass, but two methods fail
  at setSurface with GPU initialization status -7. **API26 rendering is not
  accepted.** This is no longer the root-binding/policy failure.

These are parent-reported device results, not devices driven by this Rust
implementation session. API26 adapter/backend/driver diagnostics are pending;
no renderer change or fallback relaxation has been made. Preserve this hash
while collecting logcat; any future graphics change requires rechecking the
already passing API35/16KB and API36 configurations. This is still synthetic
fixture evidence, not real-game compatibility or public-release clearance.
