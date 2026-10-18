# Native icon activation selection / enabled input scope

This candidate combines exact sentenceDA return byte-length/null query fix
with two isolated native icon-input corrections. It does not claim a completed
commercial save/load roundtrip.

1. Native root+1C (`descriptor.flags[4]`) requests selection of the successfully
   activated group/item. Prior press and deferred-release code queued6/7 events
   but never applied this current-item setter. New `select_activated_item` uses
   root flag and group selection-enabled independently from pointer/hover
   selection. Both activation timings invoke it. Existing exclusion and peer
   rules remain; no automatic selected index, event injection or flag forcing.
2. Pointer scope should use enabled Window/Virtual chain, not render opacity.
   Actual restricted sample toolbar object10048 has Window(606,545), childSAVE
   rect(633,687,49,28), eligible=true, mask=true, drawable=true, enabled=true,
   but Windowbase opacity0. The old display-visible test rejected all hits.
   Input now checks enabled/cycle/display-tree scope independently of alpha;
   render visibility remains unchanged, disabled controls still rejected.

Original structural regressions prove rootflag1+groupselectiontrue+
pointerselectionfalse startsBE=-1, then action selects item for immediate and
release timing; selected-hit exclusion still rejects the current selected item.
Full input test checks a zero-alpha Window processes down/deferred up with real
6/7 events while pointer hover itself does not select. Updated opacity test
keeps disabled/no-draw assertions and explicitly separates input from drawing.

Real scripted sample replay with ground-truth655,702 toolbar point confirms
press hit0/0, matching deferred release and actual BF10000006 after the fix.
No fabricated event or fake selected slot was supplied. **Replay still did not
enterusdtwnd:** script released/reconfigured toolbar afterward; thus rootflag
and opacity changes are proven source defects but not yet a confirmed cure for
the parent's10-slot hover-only behavior. Parent has a direct reproducible actual
SaveUI and must test new candidate there. Avoid calling this "slot fixed".

Candidate also includes exactDA native return semantics independently verified
from authorizedac_chinese.exe: UTF8byte count excludingNUL, destinationNULL
query, miss0/no-write. Full dispatch tests assert multibyte length and unchanged
zero-page/output on query/miss.

Logs in approved temp: `ethornell-slot-after-input-fix.log` (real toolbarBF6),
`ethornell-input-activation-tests.log`, `ethornell-slot-input-full-tests.log`,
`ethornell-slot-input-build.log`. Slot actual activation/thumbnail/file creation
and load into a new game session remain acceptance requirements. Parent's
clarification supersedes earlier suspected crash: repeated current attempts
remain live but do not save, so no native-crash claim.

Candidate SHA-256
`dc5dc2abe9ec2eca23ef783802ea326e49a50d7a4c97a4fa35d90c141ebeb553`;
26,128,136stripped bytes, built `2026-10-15T19:00:38.161709+00:00`.
Independent static gate and all recorded source hashes pass. Seven-crate host
suite **684passed, zero failed** (ABI6/app407/archive16/audio6/render18/script22/
VM209),3ignored and2explicit missing-testcase filters not counted passed.
Source tarball SHA-256
`5f5abfdf7c72030202343070861319de1165496550dc4348ad1b57979ff6f58d`;
working-tree patch SHA-256
`010a901efe1af85fea1b4bfaa2dc23a487fd61539c57715248ea81c09bd1f963`.
