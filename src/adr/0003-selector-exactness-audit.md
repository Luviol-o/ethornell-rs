# 0003. Every native selector matches BGI 1.622 exactly

**Status:** In progress (started 2026-10-22)

## Context

The implementation table in `native_call.rs` marked 374 of the 741 selectors
of the senmomo `BGI.exe` (BGI 1.622) as `Partial` and 319 as
`PortableEquivalent`. A spot check on 2026-10-21/22 showed both labels are
unreliable:

* most `Partial` notes describe the target without naming a gap — nobody had
  compared them line by line;
* several `PortableEquivalent` selectors were wrong: 81:0F (IsIconic) always
  returned 0, 81:65 minimized the window instead of locking its position,
  A0:86 never wrote its output, 90:78 and C0:18 panicked;
* many notes cite addresses of the older Tayutama2 build, whose globals and
  helpers differ from 1.622.

The user's requirement: no `Partial`; every selector behaves like the target.

## Decision

1. Audit all 741 selectors (not only the `Partial` ones) against the 1.622
   binary, one at a time, in table order. Evidence: `src/raw/bgi1622/
   native_all.c/.asm` (the closure of all nine tables) through
   `src/tools/raw_fn.py`, and the IDA database for anything outside the
   closure (render loop, WndProc, startup). The Rust side is located with
   `src/tools/rust_arm.py`.
2. A selector is done when its script-visible behaviour is the target's:
   argument pop order and validation, fatal errors, status codes, every
   memory write, state changes and their timing, integer/x87 arithmetic.
   Where the target calls an OS service (files, dialogs, registry, shell,
   CPUID, timers, MCI, DirectShow, Flash), the same algorithm runs over the
   host's equivalent service; when the host has no equivalent the selector
   behaves as the target does when that service fails. Such cases are named
   in the selector's notes.
   File-system and path services are the exception: on non-Windows hosts
   they run over the emulated Windows namespace of
   [ADR 0004](0004-windows-filesystem-emulation.md), not the failure path.
3. Fix, then `src/tools/promote.py` with notes citing 1.622 addresses, then
   mark the row `verified` in `0003-selector-audit.csv`.
4. Static `.data`/`.bss` values are read from the PE file, not IDA
   (`get_wide_dword` shows 0xFFFFFFFF for `.bss`).

## Consequences

* `PortableEquivalent` means verified against 1.622 by this audit.
* Tests run with dialogs suppressed (`host_dialog::host_dialogs_suppressed`).

## Progress / next step

Checklist: [`0003-selector-audit.csv`](0003-selector-audit.csv)
(`selector,handler,symbol,status`).

Done so far (2026-10-22): 80:00–1F (minus 0x03), 80:58/59/5A/65, 81:0F/65/68,
90:78, 90:F3, A0:86, C0:18. Cross-cutting fixes made on the way:

* **Clock** (`time.rs`, `native_thread.rs`): sub_499290 is a wrapping 32-bit
  clock starting at the host uptime (0 for replays); thread deadlines use
  wrapping add and the signed distance of sub_445630; 80:59 extends the
  deadline instead of resetting it; 80:05 reads the host monotonic clock.
* **Window lifecycle** (`sync_host_window_state`): WM_ACTIVATE/WM_SIZE order;
  Sys81:68 freezes the clock on deactivation; minimizing mutes the primary
  BGM/SE banks and the global DirectShow movie and restores them afterwards;
  minimize latch (80:0E/80:65); IsIconic (81:0F); centre lock (81:65).
* **Input pump** (`apply_runtime_input_event`): Knob-captured left/right
  presses only move the accumulator; wheel counts before a Knob takes it;
  horizontal wheel (142/143); WM_SYSKEYDOWN/UP (Alt+Enter, Alt+F4, F10, Alt
  recorded on release); WM_CLOSE counts; 4x4 double-click box; aggregates
  193/194 follow the class 0x100/0x200 tables; lazy release in sub_46E0A0.
* **Frame-time accumulator** (80:06/07), **processor record** (80:0A/0B,
  `cpu_record.rs`), full display-mode size table.

Open items found on the way (to fix with their selector groups):

* Knob: right press sets Knob+0x0C (sub_463CE0 hit = pointer-chain scope
  test, not geometry); the Rust Knob hit test and +0x0C flag need the Knob
  audit (91:D*).
* DCIPIcon presses are handled in the Rust input event; the target polls the
  input records in sub_448690 (90:B*).
* Modal dialogs freeze the clock (sub_4992E0(1)); the portable dialogs block
  the frame loop instead, so only gaps under 500 ms still count.
* **VM value/memory model (blocking 80:20/21 and every pointer-producing
  selector).** BGI 1.622 values are raw 32-bit cells; addresses are
  segment<<26 | offset (sub_48E8C0): 0 global, 1 code (opcode 0x05 pushes
  `(pc_base+off) | 0x04000000`), 2 thread data (opcode 0x04 pushes
  `(base-off) | 0x08000000`), 3 object, 0x10–0x3F heap. The heap
  (sub_48E760/sub_48E800) has five classes (≤4 KiB/64 KiB/1 MiB/16 MiB/64 MiB;
  65536/4096/256/16/32 slots; segment bases 0x10/0x14/0x18/0x1C/0x20), takes
  the lowest free slot of the first fitting class and returns
  ((slot & mask) << shift) | ((base + (slot >> (26-shift))) << 26); size 0 or
  over 64 MiB is fatal, a failed allocation is fatal. The Rust VM instead
  uses typed values (Str/Func/Ptr), tag 0x12 for frame memory, tag 0x10 for
  string literals and a bump/free-list heap with its own tags. Script-visible
  pointer values therefore differ. Waiting on the user's decision whether to
  rewrite the VM core to the target model.
* The VM answers many 0x81 selectors itself; several app `call_sys` arms for
  0x81 are dead (81:65/68/69 seen) — sort these out in the 0x81 pass.

Resource I/O: 80:30..39 and 80:3B are done (`resource_io.rs`, C in
`src/engine/resource_io.c`); 80:3A is deferred on ADR 0004 (host side). Fixed
on the way (2026-10-22): 80:32 resolves root + name unless absolute, no
longer creates parent directories, and pushes 1 for length 0; 80:33 builds
"dir\\file" / primary root + file without an absolute check, read-only files
are refused; 80:38 null name -> 0; 80:3A mode 4 is CSIDL_PROGRAM_FILES and
modes 0/4 report 1 even without a path; 80:3B pop order was wrong (target:
mode, initial dir, title, extension, description, buffer) and the 780-byte
buffer is now cleared first. senmomo replay still creates `UserData\` itself
(Sys80:28) before writing.

Continued 2026-10-22 (second pass): 80:3C..3F, 45..4C, 50, 52..54, 5C..5F
verified; 80:40/41/44 deferred (sub_4450B0 returns the code-segment offset
of the loaded module — blocked on the VM value model). Fixes:

* 80:3C: the VM rewrites the file operand to primary root + file before the
  host (sync dialog or async request) sees it; OK|CANCEL with the script's
  caption, Cancel -> quit question ("中止しますか？" on Japanese systems,
  sub_46FC50), YES -> 0. Shared host helper `retry_or_quit_prompt`.
* 80:3D kind 1 returns 0 without a media root; 80:3E stores "%s\\" as is.
* 80:3F ported into the VM (`sys80_3f_find_media`): drives taken once, 20
  tries per round (Sleep(50) only without drives), media root = drive +
  sub_465280(marker) (trailing "\\" only for one-level markers); new host
  hooks removable_drive_roots/path_exists/retry_or_quit_prompt/sleep_ms; the
  old heuristic search (game dir, cwd, caption as marker) is gone.
* 80:4A/4B: 1.622 fatal texts (無効なスレッドハンドル/メッセージ数); 4B no
  longer clamps to 0..256 (count < 1 fatal, no upper bound).
* 80:50 ends the thread's slice (handler status 1).
* 80:54: pushes wParam then lParam (was reversed), -1/-1 when cancelled,
  newest message wins and is not consumed (every waiter is signalled).
  Which Win32 messages the host synthesizes is tracked in ADR 0004.
* 80:52 only gates the main loop's idle/raster wait (sub_48D480): pacing,
  not script-visible.
* `cthread.c` cites Tayutama2 globals/strings in places (566894, 4EB9BC);
  1.622 uses 5668A8 (switch target), 5668AC/5668B0 (exclusive thread).

IDA: copy an existing `bgi.i64` from an older scratchpad
(`find /private/tmp/claude-501 -name bgi.i64`); scripts for xrefs /
decompile / asm ranges are easy to recreate (idat -A -S<script> bgi.i64).

80:61 (dword_565EA8 = fullscreen flag of sub_45F8A0) and 80:62 (1.622 fatal
text) verified.

Third pass (2026-10-22): 80:63/64/66..69, 6A..6F, 74, 78..7B verified;
80:60, 70, 71 deferred. Fixes:

* 80:60: size index / pixel mode stored (dword_506B88 = 2, dword_518614 = 0
  initially); windowed client size follows the mode while dword_507218 is
  set (initially 1), else the Sys81:64 size; Sys81:64 with a zero dimension
  follows the mode again and resizes a windowed window at once. Deferred
  for the graph reset on a real size change (sub_443100: filters +537,
  effectors +547 rebuilt when their screen geometry differs; windows +573
  re-centred at (-w/2, -h/2), children shifted) — do it with the 90/91
  window/filter/effector audit — and for pixel mode 0 (senmomo uses 1).
* Alt+Enter/hotkey toggle: sub_4619A0 only requests; the frame step
  (sub_4619B0 -> `apply_display_toggle_request`) runs it while Sys80:62 is
  enabled and no DirectShow movie plays, and posts event {1, fullscreen, 0}.
* 80:64 clears the input records (sub_46DF80) on show and hide.
* 80:6B: full engine restart (status 5): the VM resolves (root, archive)
  like sub_465650 (`bootstrap_root_and_archive`), halts, and
  `RuntimeEngine::take_restart` builds a new engine in `drive_frame`.
* 80:6D: WM_DROPFILES posts event {16, 0, 0}; an empty drop keeps the path.
* 80:6F: desktop = monitor video mode (GetAdapterDisplayMode), halved for
  spanned monitors, compared in double precision with the mode size.
* 80:70: index > 12 is fatal (無効なサイズ番号); always pushes 1. 80:70/71
  deferred: the global segment is still a `global_config` mirror beside
  `memory` (VM model question).
* 80:78..7B: path = validated root + BGI%.4d.cad; GetLocalTime (local, not
  UTC) seeds the integrity stream with wMilliseconds; 80:78 ignores a failed
  write; label error text from 1.622.
* Error texts from 1.622 for 80:60 (ピクセルモード/スクリーンサイズ) and
  80:67 (マウスカーソル形状番号).

80:80..85 verified (2026-10-22):

* BGI.gdb lives at validated root + "BGI.gdb" (load and save).
* Load accepts the config/user-data sizes the file declares (user data is
  zero-filled to 1 MiB) instead of rejecting non-1024/1 MiB layouts, and
  reads the pre-3.00 fixed layout (`install_legacy_global_data`; variant
  chosen by the flag record at 394248; status 2 when sizes do not add up,
  data installed anyway).
* The stored window position goes through `restore_window_position`
  (sub_46FE90: >= 1/8 of the window on some monitor, else sub_461B40 centre;
  (0, 0) in full screen). The app samples monitors, the current monitor,
  decorations and the outer position every frame.
* Save writes GetWindowRect left/top into header +20/+24 (was always 0).
* 80:82/83 fatal texts (無効なオフセット / 無効なデータサイズ, unsigned).

80:88..8B verified: 88 accepts an empty table name; 8A checks start >=
length (2) before the count checks (3), all unsigned; 8B pops index, name,
then the output DWORD (the old tests and a "swap if the name exists"
heuristic had name/output reversed) and leaves the output untouched on
failure.

Next: 80:90 onward in table order (`python3 src/tools/audit_mark.py --todo`).
