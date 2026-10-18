# Measured VM admission-profile correction

The real-game device failure `status=-9 stop=10 pc=942 offset=2075 group=176
id=128` did **not** mean WaitingForHost was fatal: is_fatal excludes it. Recent
report context described the last successful suspension, while the following
VM capacity check returned the actual error.

Exact reproduction using restricted HostRuntime CPU/VM/IO and production
before/after budget checks failed after frame247:

* vm_count 14 (limit64)
* private VM memory/capacity 606,385,420 bytes
* shared heap bytes 41,482,812; capacity 42,055,488 bytes
* journal entries11; retained capacity384 bytes; covered range33,563,863 bytes
* shared generation8 and oldest consumed generation8
* combined capacity **648,441,292 bytes (~618.4MiB)** exceeds prior512MiB

This is not just 32 children times initial16MiB. Each VM retains expanded shared
heap mirrors. A measurement-only1200-frame run (2GiB temporary test ceiling,
never a production setting) peaks at648,441,292 and14VMs, not unbounded growth.

Production correction: retain512MiB profile for Android physicalRAM below4GiB
or sysconf failure; use a bounded768MiB profile on >=4GiB devices. Sysconf pages
and page-size multiplication is checked. Max64VMs and1,000,000 journal entries
unchanged. This provides approximately24% headroom over the measured workload,
not an unlimited budget or a promise of process RSS/headroom. Renderer/audio/
decoded assets have other limits and memory pressure remains a validation risk.
VM layout, pointers, shared synchronization and continuation behavior are not
rewritten in this failure fix. Lower-memory devices may still legitimately reject
this sample with cause7; they are not silently admitted.

Tests: repaired-profile restricted1200-frame run passes including before/after
checks; peak remains648,441,292/14VMs. No fake/default response was supplied to
host requests. Physical-class boundaries, unknown RAM conservative default,
fixed high-RAM ceiling, exact-byte budget boundary and count rejection tested.
The new optional authorized-sample test is ignored in normal suites and executed
explicitly with approved sample path and a new temporary user root. It is
CPU/VM/IO evidence, not actual Android GPU/font/audio/title/save evidence.

Diagnostics use existing optional `cause=0..8`; current error's explicit VM
budget string takes precedence and becomes cause7 rather than looking only in
recent VM trace. No origin field. GPU fixed-stage diagnostics retained. No
unsupported capability, strictness or guest path rule is relaxed.

Evidence logs in approved OpenCode temp directory:
`ethornell-restricted-amairo-budget-repro.log` (true512MiB failure),
`ethornell-restricted-amairo-peak.log` (measurement),
`ethornell-restricted-amairo-budget-fixed.log` (repaired profile),
`ethornell-budget-profile-tests.log`, `ethornell-budget-profile-build.log`.

## Delivered candidate

* SHA-256 `fd5b43ce646ac297e8ae2ee318b5d4c2da5a64ea522076bb7f4c8265304da2ab`.
* 26,098,728 stripped bytes, built `2026-10-15T14:01:11.750137+00:00`.
* Parent independently copied this runtime/provenance pair into App for testing;
  implementation session did not overwrite that pair after parent copy.
* Independent static gate and recorded source hashes pass.
* Normal selected host suite **652 passed, zero failed**:
  ABI6/app405/archive15/audio6/render18/VM202; **three ignored** app tests now
  include the new opt-in authorized-sample test, executed separately and passed.
  Same two absent-testcase filters remain explicitly not passed.
* Current source-only tarball SHA-256
  `00520ffa805773eda5cecd05b1de0100db01d86e13048342f8404ad3d3aa6be9`;
  working-tree patch SHA-256
  `c28b939c26417339ae71aa1f87c2efa54e2e92969aad830d4f4edf3ed4ccf63e`.
* Device results from previous hashes do not automatically apply; parent is
  checking game35s, actual16KB regression and API26 GPU reason for this hash.
