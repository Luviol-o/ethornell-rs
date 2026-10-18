# User-data capability alias fix / namespace boundaries

Device observation on fd5b candidate: session survived35s but showed actual
Japanese save-failure message, not a title. User root had only lock/harness
files. Local restricted reproduction confirmed:

* initial read-user `BGI.gdb` missing (expected for independent empty user tree)
* writable probe **`bgi-resource\UserData` denied**
* no write_file_bytes call occurred before B0:80 save-failure message

The game constructs UserData from its primary install root. The fix aliases
only **bgi-resource/UserData[/...]** for user-file capabilities to the separate
user_root/UserData. Probe actually creates the private UserData directory and
creates/removes a real FD-opened probe. mkdir, atomic writes, read-user,
user-directory metadata/list use the same alias. Case folding and containment/
symlink/internal-name checks stay active. Other resource paths stay unwritable.

Generic explicit resource read/metadata/enumeration remain unchanged: they
continue to read/list original resource/UserData, even after a private write to
the same filename. Explicit user-file APIs read private data only. Original
save bytes are **not copied/imported**. Cross virtual namespace requests in the
generic API remain rejected. Dedicated tests use differing original/private
content, private-only names, original-only names, listing separation, aliases,
resource immutability and escape/internal-name rejection.

After fix, local restricted sample1200-frame replay records:

* real writable probe succeeds
* real atomic write `bgi-resource\UserData\AmairoChocolate101.sud`
  **407,323 bytes** succeeds and is readable in private user tree
* no host-request message remains; no synthetic response is supplied
* peak VM capacity648,441,292 bytes /14VMs, within corrected profile

However, visually inspected CPU snapshot is **not title**: black background,
message UI and empty selection bars; final title_active=false and interpreter
loops `scrslct._bp`. Thus this fixes save authorization, **not** full playability.
Earlier1200-frame success did not check for a pending save-failure request;
the opt-in regression now explicitly fails on any unresolved host request and
asserts an actual nonempty expected save before succeeding. No liveness-only
or pending-message state is called title success.

Snapshot/log evidence in approved temp directory:
`ethornell-restricted-save-io.log`, `ethornell-restricted-userdata-title.log`,
`ethornell-restricted-userdata-title.png`,
`ethornell-restricted-postsave-state.log`, `ethornell-userdata-full-tests.log`,
`ethornell-userdata-build.log`. Local-only filesystem parameter logging is
cfg(test)+explicit authorized-sample environment only, not production tracing.

## Candidate

* SHA-256 `faafda7791527279a90264d19d984a1ad2c71047470269027bfcabb340506176`.
* 26,092,616 bytes, built `2026-10-15T14:37:55.533764+00:00`.
* Parent copied exact matching runtime/provenance for device visual acceptance.
  All recorded source hashes match; independent ELF gate passes.
* Host suite **653 passed, zero failed**:
  ABI6/app406/archive15/audio6/render18/VM202; three ignored app tests (sample
  test independently executed), two explicit absent-testcase filters unchanged.
* Source bundle SHA-256
  `f6c0cb9ede8ac28a38a253be75896a3b0a479b413e349543043b83b8b3176421`.
* Working-tree patch SHA-256
  `73082ad190197264ea4c456296821c43da15b12c42dda01b3ee5fe90fd705a69`.
* Generic explicit resource namespace restoration is in this candidate; tests
  do NOT assert private overlay for generic resource reads. User capability
  metadata/list are separate entry points, not a universal prefix rewrite.
