# First-run sentence fallback / versioned native ABI correction

Fair baseline established: unrestricted desktop with original UserData/BGI.gdb
reaches title, but the SAME desktop with a clean user tree shows the same black
screen/three empty choices as Android. It is the first-run language selection,
not a restricted filesystem or GPU-specific branch. Previous title evidence
with existing saves does not validate new-user initialization.

Trace proved text was already empty in VM buffers before graphics. In the
confirmed later script81:DA consumes(destination,key,text), returns0 for absent
optional sentence replacement, and the script must then invoke81:21 fallback.
The old static Tayutama ABI had a null handler atDA; global output enforcement
discarded the built-in0. The script read a stale operand as success and rendered
the uninitialized empty destination. D8 registration also left two operands
unconsumed because no old-profile contract existed.

Fix: keep native_abi::lookup and all695 old handlers unchanged. Add explicit
NativeAbiProfile with Tayutama default and confirmed Amachoco profile:

* 81:DA three inputs, one synchronous output; returns actual0 when no optional
  substitution exists, causing original-script fallback (never fake success).
* 81:21 two inputs, no output; resolves CP932/native string, writes actual UTF-8
  bytes and matching Unicode shadow. Raw-byte tests, not shadow-only assertions.
* 81:D8 two inputs, one result as evidenced by script consumption. Optional
  sentence-table registration is not implemented; explicit failure0 consumes
  correct operands, does not claim table success or guess the database layout.

Profile is session-scoped, inherited by child/grandchild VMs. Effective ABI is
used by host argument snapshots, native frame consumption, output settling/
enforcement, return audit and cooperative-procedure checks. The profile's own
strict support is scoped to these implemented contracts; default builtins are
gated and cannot silently run those extensions. No global DA return override.

Selection is by TWO exact decoded code-resource fingerprints and lengths
(bootstrap and title), not graph encoding, Android target, arbitrary game ID,
or discovery of a selector. FNV64 here is a version identity hint, not a security
signature; IO policy remains enforced regardless. No game assets or executable
bytes are bundled, and no Windows executable is executed. Unknown revisions
keep old profile instead of blindly enabling a later contract. This is narrow
compatibility for the confirmed resource revision, not all Amachoco/BGI.

Full dispatch tests run_loaded cover sentinel+DA3in1out, D8two operand consumption
and real failure0,21UTF8bytes+shadow, host_enabledfalse/true, strictfalse/true,
main/child/grandchild inheritance, default-profile null registration, plus old
native table count tests. Restricted sample1200-frame CPU replay produces
readable language choices (Simplified Chinese / Traditional Chinese / Japanese)
and private407,323-byte initial save, with no automatic host answer/click/import.

Visually inspected screenshot:
`ethornell-profile-language-final.png` in approved temp directory. This is **not
yet title or dialogue/save-slot gameplay proof**; bottom explanatory sentence
still displays000001 and optional full sentence substitution is not implemented.
Parent must manually choose a visible language and validate subsequent screens.

Logs: `ethornell-native-profile-tests.log`,
`ethornell-profile-language-final.log`, `ethornell-language-full-tests.log`,
`ethornell-language-build.log`. No framework lifecycle/rendering rewrite made.

## Artifact

SHA-256 `ef516a1c81068f02b08959a42a51a8bc8b98492c62ac136eb0ea17727cd2ff7e`;
26,110,184 stripped bytes, built `2026-10-15T15:21:39.194815+00:00`.
Independent static ELF gate and all current recorded input hashes pass.
Seven-crate suite **676 passed, zero failed**:
ABI6/app406/archive15/audio6/render18/script22/VM203.
Three existing/opt-in app ignored and two explicit missing-testcase filters are
not counted as passes. Authorized-sample test was also run explicitly and passed.
Current source bundle SHA-256:
`7cf7690361b98165a40a39bf438c04ef1ea3fe9d102972e3018c69b128324d9a`;
tracked working-tree patch SHA-256:
`8130ac0db57b7bb77deec9ec20fcb4ad4a68e0e484880fe9ad9f967b2f6277f3`.
