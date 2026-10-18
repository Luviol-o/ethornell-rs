# Confirmed-profile UTF-8 sentence table support

Parent confirmed ef516 can select Simplified Chinese, reach real title and
enter new game with street scene/name, but dialogue stayed000000/000001. That
candidate implemented only original-text fallback, insufficient for key-based
dialogue. Title success alone was not readable-game success.

Actual decoded jp/sc/tc._bsi format is: LEu32 group_count, then for each group
UTF8-NUL group_name, LEu32 entry_count, UTF8-NUL key/value pairs. The sc file
has10groups and consumes exactly1,030,814bytes. No translated assets or
game-specific numeric replacements are embedded in code. Profile-specific
81:D8 registers a completely parsed table transactionally in shared system81
state; 81:DA resolves exactly(group,key), writes UTF8+NUL and Unicode shadow,
and returns1 only for a real match. Missing key returns0 for real script fallback.
Null/zero registration clears; malformed registration returns0 preserving old
state. Parent/child access uses existing shared-state ownership.

Second root cause: resource GetFileSize returned compressed ARC length464,347,
while ReadFileBytes decoded1,030,814. Script passed the former length to D8,
thus truncating the table. `decoded_size_by_entry` now reads only a stack DSC
header from the retained authorized FD, checks packed/decoded budget/range,
and returns the same decoded payload length as the read API. No decoder parses
beyond caller length or silently trusts a packed-size buffer.

Third root cause: later native profile writes/copies UTF8 but old string
helpers re-encoded those strings into CP932 (including decimal replacement
entities for unrepresentable Simplified glyphs), then graphGBK decoded garbage.
Confirmed profile now retains UTF8 in its native string write/fixed buffer,
find/replace/strlen and string-table-length paths; validUTF8 native buffers are
read as such, otherwise raw CP932 still decodes by existing encoding path.
Tayutama default remains CP932, its695 static handlers unchanged. Runtime GBK
font selection also probes Simplified and Traditional representative glyphs
instead of accepting a Japanese-only face that lacks evenU+7B80.

Safety:16MiB sentence input,4096groups,262,144entries,64KiB string caps;
strictUTF8/NUL/bounds/duplicate keys+groups/trailing-byte rejection, transactional
install. Native pointer ranges checked by VM. No filesystem authorization is
added. Tests cover original synthetic table, truncated prefixes, invalidUTF8,
overflow/count/duplicate/trailing inputs, deterministic arbitrary-byte fuzz,
full-dispatch register/lookup/clear/failed-replacement preservation, rawUTF8
output+shadow and shared child visibility. Prior full-dispatch/profile tests
remain in suite. DSC decoded-size budget tests added.

Restricted1200-frame sample screenshot now contains actual Chinese introductory
sentence instead of000001; visual evidence `ethornell-sentence-profile.png` in
approved temp. **Known remaining text issues:** third language button currently
shows a pointer literal, and escaped\\n in introductory text is not yet rendered
as newline. No actual Android narrative dialogue or commercial slot roundtrip
is claimed from CPU snapshot/host tests. Parent will use the candidate with
already-chosen language to validate actual000000/000001 narrative replacements.

Commercial slot clarification: parent later observed no native exit on repeated
attempts, but save-slot center/action clicks produce no save/thumbnail. Treat as
unresolved input/slot behavior, not a confirmed native crash. It is not solved
by synthetic global-data serializer reopen tests.

Logs: `ethornell-sentence-full-tests.log`, `ethornell-sentence-build.log`,
`ethornell-sentence-profile.log` in approved temp directory. Translated table
assets/extractions/trace stay temporary and are never in source bundles.

## Candidate evidence

SHA-256 `474d307333f748e472d9381a4ca221aa332bb62bee6a9b21a9b10f98ffaad9f0`;
26,127,400stripped bytes, built `2026-10-15T16:51:40.869877+00:00`.
Independent ELF gate and recorded source hashes pass. Selected seven-crate host
suite **683passed, zero failed** (ABI6/app406/archive16/audio6/render18/script22/
VM209), same3ignored and2explicit missing-testcase filters not counted passed.
Source-only tarball SHA-256
`ca4d47ec80538b346c6bec3b0d7bc2265f097c417bdc071796da6325a3c0125c`;
working-tree patch SHA-256
`e528a633e9675b134585bb26b5c65f42cdd7b9a51a08113459ec6a0e1592ae48`.

## Later exact ABI evidence correction (source-only, after474 candidate)

Independent read-only researcher examined the authorizedac_chinese.exe SHA-256
`e1124697130ef381818b9266738e7326b839e5a367640f0cc93f59766ae9be93`:
dispatcher4977C0/table4FD368, D8handler497310/helper4A45A0;
DAhandler497340/helper4A47B0 copiesNUL-terminated payload and49738A..49739C
returns strlen **UTF8bytes excludingNUL**, not Boolean1. Null destination queries
length only. Miss0 does not write; empty payload0 follows script fallback.
Prior comment using4961E0 referred to a different baseline and is not evidence
for this executable. Profile is versioned; the observed payload has no magic/
version field. Transactional failure preservation/duplicate rejection are safe
implementation constraints, not claims about original unchecked native code.

DA return/null semantics corrected in source and full-dispatch unit verifies
multibyte byte count, null non-write, miss non-write. This change is **not in474**;
await current device feedback before next combined candidate build.
