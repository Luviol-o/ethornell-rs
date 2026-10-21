# 0002. 0xD0 is the logic extension (grid field, space, search worker)

**Status:** In progress (started 2026-10-21)

## Context

The 0xD0 native table was carried as "legacy 3D" (`execute_legacy_3d` in
`crates/ethornell-vm/src/extended_opcodes.rs`): it pops the right number of
arguments, hands out handles for create calls and returns 0 for the rest.
RTTI in BGI 1.622 shows it is a strategy-game (SLG, 战棋) toolkit instead:

| Selectors | Class | Object | Handle list |
|---|---|---|---|
| D0:00–2D | `DCTELgclGrdFldMngr` : `DCTELgclFldMngr` — tile map, terrain cost tables, units, reachability/path fields | 0x498 bytes, ctor `sub_454940` | `dword_5669EC`, ids from `dword_5669E4++` |
| (per unit) | `DCTELgclGrdFld` : `DCTELogicalField` — one unit's movement field | created through `GrdFldMngr` vtbl +0x24 | owned by the unit record (+0x28) |
| D0:40–7A | `DCTELgclSpcMngr` — 3D space / body simulation | 0x170 bytes, ctor `sub_4559B0` | `dword_5669F8`, ids from `++dword_5669F0` |
| D0:80–8E | `DCTESLGEvaluator` — SLG AI, run on a worker thread | 0x160 bytes, ctor `sub_4583E0`; job record 0x78 | `dword_566A00`, ids from `++dword_5669FC` |

Every wrapper maps the internal status through `sub_497120`
(0 → 0, 0x80000000 → 1, 0x80000001 → 2, … 0xA0000006 → 22, −1/−2 kept, other → −1);
that small integer is what the script sees.

None of the local games (senmomo, Tayutama2) call 0xD0, so there is no runtime
oracle; behaviour is checked by unit tests written against the decompilation.

Evidence: `src/raw/bgi1622/d0.c` (279 functions), `d0.asm` (same functions plus
the vtables), handlers 0x477910–0x478A80, wrappers 0x497120–0x4983B0.
Float arguments arrive as 16.16 fixed and are multiplied by
`dbl_4EC968 = 1/65536` in the handler.

## Decision

1. Port the three classes for real, one at a time, in this order:
   grid field manager (+ per-unit field) → evaluator → space manager.
   The evaluator depends on the grid field; the space manager is independent.
2. Cleaned C: `src/engine/logic_grid_field.c`, `logic_slg_evaluator.c`,
   `logic_space.c`. Rust: a new module per class under `crates/ethornell-vm`.
3. **The evaluator's worker thread is not reproduced.** The target queues a
   job (`D0:88/8A/8D` with the async flag), returns 0, and `D0:87` reports
   14 ("busy") until the thread posts the result. The port runs the job to
   completion at submission and stores the result, so the first `D0:87`
   already sees it. A script that polls until "not busy" behaves the same,
   and replays stay deterministic. A script that relied on observing 14 at
   least once would differ; none is known.
4. Status codes are kept as the target's internal 0x8000xxxx/0xA000xxxx values
   inside the port and mapped through the `sub_497120` table at the boundary,
   so the C and Rust can be compared line by line.

## Consequences

* `execute_legacy_3d` is replaced selector by selector; the ABI table there
  (`legacy_3d_abi`) stays the source of argument counts until each group is
  ported.
* Without a game, ABI mistakes (argument order) are the main risk; each
  handler's pop order is taken from `d0.asm`, not from Hex-Rays, and recorded
  in the C file.

## Progress / next step

* [x] Raw export of the whole closure checked in (`src/raw/bgi1622/d0.*`).
* [ ] `logic_grid_field.c`: handler ABI table, `DCTELgclFldMngr` /
      `DCTELgclGrdFldMngr` layout and methods, `DCTELgclGrdFld`.
* [ ] Rust port of the grid field + tests.
* [ ] Evaluator (D0:80–8E).
* [ ] Space manager (D0:40–7A).
