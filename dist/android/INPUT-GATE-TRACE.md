# SAVE script post-event gate observation

Actual Android1001evidence proves slotgroup2/item0 and actiongroup7/item0
produce/consumeBF10000006. No packing fix is justified. BF writes exactlythree
DWORDs; registry-exists result is separate operand output. Rawhit01's different
payload shape explains misleading group0/item2/7 diagnostic, not a script bug.
Decompiler symbolic expressions after calls may be contaminated: topSaveBE4
branches on actual userdataFileExists result, not selectedSlotIndex0 as Boolean.

New1002 diagnostic, same kind1/request0, onlyexistingDebugsentinel trace:

```
input_gate pc=<u32> frame=<u32> event=<u32> payload=<u32> parameter=<u32> group=<u32> item=<u32> running=<u32> gate=<u32>
```

Allfieldsu32decimal0..4294967295, exactorder, <=384B. Sharedtotal64with1001.
NoABIchange. After real90:BC state-buffer writes, onlyconfirmedAmachocoprofile
and outptr==(0x12000000|currentBase-0x64), readguestbase[-70/-6C/-68],
base[-44/-40], actualBCword0 andglobal1092. Zeroeventframesproduce none.
Noqueuefakeevent, changedpacking, extraDWORDoutput, gatewrite or fallbackinput.
This reveals whetherguest receives06/localgroupitem and whether1092blocks.

Host/VMtests and complete selected suite pass. Logs:
`ethornell-input-gate-full-tests.log`, `ethornell-input-gate-build.log` in
approvedtemp. Existing1001schema unchanged, no generalVM/graphtexttrace.

Artifact SHA-256
`9e2fefbf85c84bc89f953a01d72c2c0a2fd0369b7d04df4972ca866dc1770925`;
26,137,768strippedbytes, built`2026-10-15T21:00:25.439565+00:00`.
Independentstaticgate/recordedsourcehashespass. Seven-crate685passed/0failed,
same3ignored/2filterednotcountedpassed. SourcebundleSHA-256
`a9798b9a496f686ebfbd2df9e05d42e5542d1ebdd042afda503702c77f227f26`;
trackedpatchSHA-256
`c2b0f532688f5018cdc015209f590c38b228af989b82470edf2ba0f05734b1a0`.
