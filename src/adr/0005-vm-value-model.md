# 0005. The VM uses BGI 1.622's value and address model

**Status:** In progress (decided 2026-10-22: "如果简单尝试后现在的没法和原版等价导致很多功能接入有问题，那就优先修").

## Context

BGI 1.622 values are raw 32-bit cells on an operand ring (sub_445480 pops,
sub_4454A0 pushes, the ring wraps at its size). A pointer is
`segment << 26 | offset`, translated by sub_48E8C0:

| segment | meaning |
|---|---|
| 0 | process globals (dword_56676C + offset) |
| 1 | the current thread's code region (BP modules appended; opcode 0x05 pushes `(pc_base + off) \| 0x04000000`, so string literals are code pointers) |
| 2 | the current thread's data region (opcode 0x04 pushes `(base - off) \| 0x08000000`) |
| 3 | object memory through the thread object's virtual +12 |
| 0x10..0x3F | heap blocks: five classes (below) |

Address 0 is null; any other untranslatable address is fatal:
「無効なポインタ [ $%.8X ] にアクセスしようとしました」.

Heap (sub_48E760 alloc, sub_48E800 free; tables at 0x503E78..0x503EE4):

| class | max size | slots | segments | shift (offset bits) |
|---|---|---|---|---|
| 0 | 0x1000 | 0x10000 | 0x10..0x13 | 12 |
| 1 | 0x10000 | 0x1000 | 0x14..0x17 | 16 |
| 2 | 0x100000 | 0x100 | 0x18..0x1B | 20 |
| 3 | 0x1000000 | 0x10 | 0x1C..0x1F | 24 |
| 4 | 0x4000000 | 0x20 | 0x20..0x3F | 26 |

Alloc takes the first class whose max size >= n and its lowest free slot v:
`((v & ((1 << (26 - s)) - 1)) << s) | ((base + (v >> (26 - s))) << 26)`.
Free needs the offset bits to be zero. The heap is process-wide.

The Rust VM instead (found 2026-10-22, senmomo replay with probes):

* typed stack values (`Value::Int/Str/Ptr/Func`); string literals are owned
  `String`s, functions `{program, offset}`;
* a flat buffer with its own tags: frames `0x12000000 | off`, code pointers
  `0x10000000 | off` (mapped onto the *global* window, so they read the wrong
  bytes), heap/aux tags from 0x20 — 0x10..0x3F are heap segments in the
  target;
* "shadow" typed values in memory (`mem_values`) so that stored pointers,
  functions and strings survive a load: 192k shadow loads in 6000 frames of
  senmomo (175k pointers, 16k functions, 1.4k strings);
* `eq` compares string *contents*; strings in arithmetic are copied into the
  Rust heap;
* the app receives strings only where a hand-made table
  (`normalize_graph_string_args`) converts pointers; elsewhere a string held
  in memory reaches the app as the text "0x...";
* per-game patches (`scenario_guarded_read_int`, `NativeAbiProfile`, the
  zero-filled frames of `store_base`).

Twelve selectors are blocked on this (80:20/21/40/41/44/60/70/71,
81:44/6A/6B, 92:14/15/F1).

## Decision

Move the VM to the target model in phases; after each phase the VM and app
tests pass and the senmomo replay renders the same frames.

1. **Addresses.** Target encoding everywhere: frames in segment 2, code in
   segment 1 (code bytes mapped), globals in segment 0, the heap with the
   five classes and target addresses; invalid pointers fatal with the target
   text. The flat backing buffer and the thread hand-over stay.
2. **Strings.** Literals are code pointers; natives read C strings from
   memory; the app boundary converts the parameters the target handlers use
   as strings, from a table generated from the 1.622 handlers (not by hand);
   `eq` compares addresses.
3. **Functions and cells.** Function values are code addresses; the stack
   holds raw cells; `mem_values` shadows go away.
4. **Blocked selectors.** 80:20/21 on the target heap, 80:40/41/44 code
   offsets, 80:70/71 globals, 81:44, 81:6A/6B, 92:14/15/F1.

## Consequences

Script-visible pointer values match the original, so scripts that store,
compare, offset or hand pointers to natives behave the same; the shadow and
per-game workarounds can be removed. The change touches most of
`crates/ethornell-vm/src/lib.rs` and the app's string arguments.

## Progress / next step

* 2026-10-22: model recovered (above), ADR written. Next: phase 1, starting
  with the frame segment (producers of `0x1200_0000` in `lib.rs`,
  `async_program.rs`, `debug.rs`, `host_request.rs`).
