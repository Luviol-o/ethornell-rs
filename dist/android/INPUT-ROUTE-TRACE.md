# Opt-in bounded Android SAVE route observation

No ABI signature change. AndroidHostRuntime configbit1 (flags2) is now ONLY
bounded input-route diagnostics: new_explicit VMtrace=false, debug_graph=false,
trace_render_tree=false. Desktop CLI trace remains its own existing behavior.

Host kind1/code1001/request_id0 exact line:

```
input_route object=<u32> group=<u32> item=<u32> root=<u32> flags=<u32> current=<u32> pending=<u32> pointer=<0|1> hit=<0|1> press=<0|1> up=<0|1> event6=<0|1>
```

All numbers decimal; signed fields reinterpretedu32, absent group/item/current/
pending use4294967295. root isdescriptor.flags[4], flags isitem flags. event6
marks actualBF consumption of10000006, not injected or fabricated. Producer
points: real press candidates, deferred release candidates, nonemptyBFpoll.
Per-session total64events, each<=384bytes. No paths, resource names, dialogue,
script strings or arbitrary upstream text. Defaultdisabled produces none.

Parent engineowner gates flags2 only for Debug non-symlink regular zero-byte
uid-owned private sentinel `files/ethornell_debug/.ethornell-input-trace`, with
strict directory/file mode checks. Release ignores sentinel. Activity accepts
only exact numeric schema and repeats64event limit; tagEthornellInputTrace.
Rust does not implement an Intent, exporteddiagnostic endpoint or guest path.

This is a diagnostic candidate to capture actual SAVEgroup2 pointer routing,
not a claim that commercial slots are fixed. Prior activation/input-enable
corrections and exactDAbyte-length semantics retained. App cannot use olddc5
runtime for this protocol; pair new candidate/provenance with engine changes.
Intermediate parent test briefly saw missing method while code was being
edited; subsequent complete seven-crate suite passes, no compile-broken final.
Logs: `ethornell-route-trace-tests.log`, `ethornell-route-trace-build.log` in
approvedtempdirectory.

Artifact SHA-256
`0e1b2d296c4e512d16e068add70cf0d6fd3f7c0bc42109e6f5a83964cbca4424`;
26,137,192stripped bytes, built`2026-10-15T20:00:55.874337+00:00`.
Independent static gate/recorded source hashes pass. Seven-crate suite
**685passed/0failed**: ABI6/app408/archive16/audio6/render18/script22/VM209;
same3ignored and2explicitmissing-testcasefilters not counted passed.
Source tarball SHA-256
`29345f254409d7897ee64a7a04cc9c31fec7bf00994908ab63eaf2f8d2adfd16`;
tracked patch SHA-256
`9821f645dbf7a2a83b2683bf7ef91e9599c609e2dc877400ab67eae922dea0e8`.
