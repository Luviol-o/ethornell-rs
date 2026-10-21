#!/usr/bin/env python3
"""List the Rust match arms / branches that handle one native selector.

Usage: python3 src/tools/rust_arm.py 80:24 [--context N]

Searches crates/*/src for `(0x80, 0x24)`-style tuples (any case/spacing) and
`0x24 =>` arms inside per-group dispatchers, printing N lines after each hit.
"""
import re
import sys
from pathlib import Path

root = Path(__file__).resolve().parents[2]
group, ident = (int(x, 16) for x in sys.argv[1].split(":"))
ctx = int(sys.argv[sys.argv.index("--context") + 1]) if "--context" in sys.argv else 25
pat = re.compile(
    r"\(\s*0x0*%x\s*,\s*0x0*%x\s*\)|\(\s*0x0*%x\s*,\s*[^)]*\b0x0*%x\b[^)]*\)\s*=>"
    % (group, ident, group, ident),
    re.I,
)
for path in sorted(root.glob("crates/*/src/**/*.rs")):
    if "native_call.rs" in path.name or "native_contract_reference" in path.name:
        continue
    lines = path.read_text(errors="replace").split("\n")
    for i, line in enumerate(lines):
        if pat.search(line) and "#[test]" not in "".join(lines[max(0, i - 3):i]):
            rel = path.relative_to(root)
            print(f"===== {rel}:{i + 1}")
            print("\n".join(lines[i:i + ctx]))
