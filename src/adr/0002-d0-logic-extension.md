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
* [x] `src/engine/logic_grid_field.c`: all 19 grid selectors (D0:00–2D), their
      script argument order, the manager/unit/field layouts, the movement
      search, line of fire, directions. Handler argument order was derived
      mechanically from `d0.asm` (a small symbolic run over each handler's
      pops and pushes); the table is at the end of the C file.
* [x] Rust: `crates/ethornell-vm/src/logic/{mod,grid,spline}.rs`, wired in
      front of `execute_legacy_3d`. `CSpline` is ported exactly (coefficient
      form, 100-point cap) rather than reusing the app's animation spline,
      whose arithmetic differs.
* [x] `legacy_3d_abi` argument counts fixed for D0:67/6A/70/72/74/75 (they
      did not match the handlers and would have misaligned the stack).
* [ ] Evaluator (D0:80–8E): `DCTESLGEvaluator`, 0x453C20/0x4583E0..0x45BB60,
      uses the grid's sorted reachable list (MSVC `qsort`, not stable — port
      the CRT algorithm), `GrdFldMngr` vtbl+10/+14 and unit+0x2C caches.
* [ ] Space manager (D0:40–7A): `DCTELgclSpcMngr`, 0x455840..0x457A10,
      SSE float math; ctor `sub_4559B0` allocates 0x1800 bytes (64 bodies?).

Deviations from the target, all where it would corrupt memory or crash:
units never placed (x = -1) are skipped when marking occupancy and zones
(the target writes at index -(w+1)); a target height outside 0..31 in D0:28
is out of range (the target reads past a 32-entry stack array); D0:04 rejects
maps over 4M tiles or with a negative area; null output pointers are not
written.
