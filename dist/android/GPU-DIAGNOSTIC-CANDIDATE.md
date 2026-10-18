# Separate API26 GPU diagnostic candidate

This follow-up modifies only `crates/ethornell-android/src/lib.rs` diagnostic
classification/redaction. No renderer, adapter selection, backend preference,
limits, shader, presenter or game execution behavior changes. The validated App
`5c15c4219b61206487335df3d6c509016317c123f404b89c7c1210350de91d64` pair is
**not automatically replaced** while the parent runs other device acceptance.

Fixed safe GPU reasons:

* stage1: invalid dimensions / raw Surface creation failure
* stage2: compatible adapter unavailable
* stage3: unsupported requested limits (whitelisted limit name and decimal
  requested/available numbers only, bounded to 768 bytes)
* stage4: request_device failed
* stage5: no Surface formats/present modes/alpha modes or render-attachment usage
* stage6: uncaptured validation error
* stage7: device lost

No arbitrary driver text, shader source, adapter name, labels or paths are
returned. Diagnostic unit test verifies secret text is omitted. Existing
`renderer_limits` already selects downlevel_webgl2_defaults and checks adapter
limits; no speculative lower limits or fallback changes were made based only on
EGL log noise. Parent needs the actual stage before any graphics behavior fix.

Parent-reported device state for the prior 5c15 candidate:

* API36 physical arm64: four core fixture methods pass; actual RGBA checker,
  reattach/stale-generation continuation, script Closed, non-BMP/thread/handles.
* API35 actual 16KB: SJIS and GBK rounds both pass all four core methods; one
  optional supplied-game test skipped in each applicable suite.
* API36 authorized actual game GBK: create/attach and at least three GPU buffers
  pass (3.12s). Title visual, dialogue, audio and save/load are not established.
* API26: load/create pass but two Surface/GPU methods fail -7. EGL logs alone do
  not prove whether adapter/device/capability/validation stage failed.

The new candidate must be identified by its own adjacent provenance/hash and
retested on API26. It does not inherit device results merely because renderer
inputs are unchanged. Static checks and host tests are separate evidence.

## Artifact and host checks

* `dist/android/arm64-v8a/libethornell.so`, 26,067,944 bytes, stripped.
* SHA-256 `0145142a5a17aca80d89471c545b00f811e46e21a28bf5bd25612b6a6977d8f1`.
* Build UTC `2026-10-15T13:03:53.378313+00:00`.
* All recorded source hashes verified; independent verify_native static gate
  passes, 14 ABI / no activity glue / system NEEDED / 16KB LOAD+RELRO.
* Complete selected host suite **650 passed / zero failed**:
  ABI5/app404/archive15/audio6/render18/VM202. Same two old missing-testcase
  filters and two pre-existing ignored app tests, not counted as passed.
* Full-suite log `ethornell-gpu-diagnostic-full-tests.log`; build log
  `ethornell-gpu-diagnostic-build.log` in approved temp directory.
* Matching current source-only tarball SHA-256
  `754c98e506fe264ef4ac4f5b469dfc6300369da24d9dd37bc7947b0ed895912e`.
  Tracked patch unchanged; new/untracked ABI implementation is in the tarball.
* App lib/provenance remain the prior `5c15…` pair, not silently updated.
  Its provenance attests its original source; comparing it to this external
  tree now identifies exactly the deliberate ABI diagnostic-file difference.
