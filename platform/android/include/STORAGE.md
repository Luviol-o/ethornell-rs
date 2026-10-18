# Host file access and user data

How the host-driven runtime (`crates/ethornell-app/src/host_runtime.rs`,
`file_policy.rs`) reads game files and writes user data, on Android and iOS.

## File/host boundary

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
