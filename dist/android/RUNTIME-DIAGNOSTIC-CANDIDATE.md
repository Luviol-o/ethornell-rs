# Combined GPU / runtime diagnostic candidate

This candidate extends the isolated GPU diagnostic revision with numeric VM
context on HostRuntime step failure. No unsupported API is enabled and no
renderer/backend/limits/normal frame execution changes are made.

Errors preserve their redacted class/fixed GPU stage and may append:
`stop=N pc=N offset=N group=N id=N`. Numbers are emitted from engine state,
then strictly parsed and bounded by ABI redaction; no dialogue, script name,
paths, driver text or shader source is exposed. Unknown PC/offset use u64::MAX;
unknown native group/id use u32::MAX.

Stop mapping: 0 unavailable, 1 Completed, 2 Yielded, 3 SwitchedThread,
4 SchedulerPassEnded, 5 InterpreterTerminated, 6 WaitingForInput,
7 WaitingForInputOrTime, 8 WaitingForTime, 9 WaitingForProcedure,
10 WaitingForHost, 11 QuantumExhausted, 12 WatchdogExceeded,
13 UnknownOpcode, 14 UnknownDispatch, 15 Error.

Host capability rejection records the attempted opcode before returning Err,
not the preceding successful call. Error paths alone add context; normal
successful frames are unchanged. Runtime diagnostics can narrow the reported
real-game black frame/early close without inventing success or bypassing errors.

Parent independently executed six-crate host suite: **651 passed, zero failed**
(ABI6/app404/archive15/audio6/render18/VM202), same two missing-testcase filters
and two existing app ignored. Parent log:
`ethornell-rust-parent-acceptance.log` in approved OpenCode temp directory.
The implementation session also ran app/ABI tests before that parent run;
there were no simultaneous Cargo builds/tests. Neither host test result is
Android device acceptance for the new candidate.

Existing production APK still contains the prior validated 5c15 candidate until
parent explicitly chooses the new diagnostic runtime. Its physical/16KB core
fixture results do not transfer automatically. Real-game short startup produced
black screenshot and longer observation exited early; title/playability and
save/load compatibility remain unproven. API26 rendering failure also remains
unresolved pending the new fixed-stage diagnostic.

## Built numeric-context candidate

* `b75e644e01c450bec784816cad1f3ca5b98837cdcc5ce366b2dcf4021617371c`
* 26,070,312 bytes, built `2026-10-15T13:34:19.123201+00:00`.
* Independent ELF gate and recorded input hashes passed at build time.
* Parent copied this candidate and verified exact APK bytes; real-game fatal
  reproduces, but initially the Java safety filter removed numeric fields.
  Parent is testing its corrected numeric whitelist; no renderer/compatibility
  behavior was changed in this candidate.

## Additional source-only investigation (not yet another runtime)

An explicit opt-in host test runs the authorized sample for 1,200 checked
frontend frames under actual HostRuntime policy/font scope, strict=false,
GBK, separate newly-created private user root and explicit silent audio mode.
It passes in 67.12s. It is a CPU/VM/IO reproduction, **not** Android Surface/
system-font/real-audio reproduction. Source sample is read-only and no original
save is imported. Log: `ethornell-restricted-amairo-repro.log`.

Prepared but not rebuilt: optional numeric `cause` from bounded recent VM trace
classification (0 absent, 1 unsupported, 2 missing resource, 3 memory bounds,
4 stack underflow, 5 font/glyph, 6 audio/sound, 7 budget/limit, 8 other).
No trace text is exposed. App/ABI tests pass for the source-only diagnostic
addition; await b75 device selector before choosing further code changes.
This deliberate source delta is not attested by the prior b75 provenance and
must be rebuilt before delivery. No new App runtime was copied by this session.
