#!/usr/bin/env python3
"""Run BGI 1.622 bitmap kernels under Unicorn and write reference vectors
for the ignored test bitmap_kernels::tests::kernels_match_the_target_vectors.

Usage: python3 -m venv v && v/bin/pip install unicorn pefile
       v/bin/python src/tools/emu_kernels.py OUT_DIR
       ETHORNELL_KERNEL_VECTORS=OUT_DIR cargo test -p ethornell-app kernels_match -- --ignored

Each vector file <kernel>_<n>.bin is little-endian u32 words
[kind, src_w, src_h, dst_w, dst_h, p0, p1, p2] followed by the source
pixels, the initial destination pixels and the destination after the
target ran (B, G, R, A bytes, rows packed).
"""
import os, random, struct, sys
import pefile
from unicorn import *
from unicorn.x86_const import *

EXE = "/Users/xmoe/Documents/千の刃濤、桃花染の皇姫 体験版/senmomo_trial/BGI.exe"
OUT = sys.argv[1]
pe = pefile.PE(EXE)
base = pe.OPTIONAL_HEADER.ImageBase
mu = Uc(UC_ARCH_X86, UC_MODE_32)
size = (pe.OPTIONAL_HEADER.SizeOfImage + 0xFFF) & ~0xFFF
mu.mem_map(base, size)
mu.mem_write(base, pe.get_memory_mapped_image())
STACK, HEAP, RET = 0x10000000, 0x20000000, 0x40000000
mu.mem_map(STACK, 0x100000)
mu.mem_map(HEAP, 0x4000000)
mu.mem_map(RET, 0x1000)

heap_top = HEAP

# Windows imports the kernels call are stubbed: each IAT slot points at a
# hooked address in STUBS.
STUBS = 0x50000000
mu.mem_map(STUBS, 0x1000)
stubs = {}


def stub(name, argc, impl):
    for entry in pe.DIRECTORY_ENTRY_IMPORT:
        for imp in entry.imports:
            if imp.name and imp.name.decode() == name:
                at = STUBS + 16 * len(stubs)
                mu.mem_write(at, b"\xC3")
                mu.mem_write(imp.address, struct.pack("<I", at))
                stubs[at] = (argc, impl)


def on_stub(uc, address, size, user):
    if address in stubs:
        argc, impl = stubs[address]
        sp = uc.reg_read(UC_X86_REG_ESP)
        ret, *args = struct.unpack("<%dI" % (argc + 1), uc.mem_read(sp, 4 * (argc + 1)))
        uc.reg_write(UC_X86_REG_EAX, impl(*args) & 0xFFFFFFFF)
        uc.reg_write(UC_X86_REG_ESP, sp + 4 * (argc + 1))
        uc.reg_write(UC_X86_REG_EIP, ret)


def set_rect(rect, left, top, right, bottom):
    mu.mem_write(rect, struct.pack("<4I", left, top, right, bottom))
    return 1


stub("SetRect", 5, set_rect)
mu.hook_add(UC_HOOK_CODE, on_stub, begin=STUBS, end=STUBS + 0xFFF)


def alloc(n):
    global heap_top
    at = heap_top
    heap_top += (n + 0xFFF) & ~0xFFF
    return at


def bitmap(width, height, pixels, fmt):
    """A 24-byte descriptor: pixels, stride, width, height, format, bpp."""
    data = alloc(max(1, width * height * 4) + 64)
    mu.mem_write(data, bytes(pixels))
    desc = alloc(24)
    mu.mem_write(desc, struct.pack("<6I", data, width * 4, width, height, fmt, 4))
    return desc, data


def call(address, stack_args, regs):
    sp = STACK + 0xF0000
    for value in reversed(stack_args):
        sp -= 4
        mu.mem_write(sp, struct.pack("<I", value & 0xFFFFFFFF))
    sp -= 4
    mu.mem_write(sp, struct.pack("<I", RET))
    mu.reg_write(UC_X86_REG_ESP, sp)
    for reg, value in regs.items():
        mu.reg_write(reg, value & 0xFFFFFFFF)
    mu.emu_start(address, RET)


rng = random.Random(1622)


def rand_pixels(n, alpha=None):
    out = bytearray()
    for _ in range(n):
        out += bytes(rng.randrange(256) for _ in range(3))
        out.append(rng.randrange(256) if alpha is None else alpha)
    return out


def emit(name, kind, sw, sh, dw, dh, params, src, dst_before, dst_after):
    with open(os.path.join(OUT, name + ".bin"), "wb") as f:
        f.write(struct.pack("<8I", kind, sw, sh, dw, dh, *[p & 0xFFFFFFFF for p in params]))
        f.write(bytes(src) + bytes(dst_before) + bytes(dst_after))


global_heap = heap_top

# kind 1: sub_418F60 (91:1B), ecx = attenuation, edx = destination,
# stack = [source, x factor, y factor].
cases = [(16, 12, 0, 0x8000, 0x8000), (16, 12, 64, 0x4000, 0xC000), (9, 7, 256, 0, 0),
         (20, 20, 128, 0x10000, 0x10000), (13, 5, 32, 0x1234, 0x0FED)]
for _ in range(10):
    cases.append((rng.randrange(2, 24), rng.randrange(2, 24), rng.randrange(257),
                  rng.randrange(0x10001), rng.randrange(0x10001)))
for n, (w, h, att, xf, yf) in enumerate(cases):
    heap_top = global_heap
    src = rand_pixels(w * h, 0)
    dst_before = rand_pixels(w * h)
    src_desc, _ = bitmap(w, h, src, 1)
    dst_desc, dst_data = bitmap(w, h, dst_before, 1)
    call(0x418F60, [src_desc, xf, yf], {UC_X86_REG_ECX: att, UC_X86_REG_EDX: dst_desc})
    after = mu.mem_read(dst_data, w * h * 4)
    emit(f"concentrate_{n}", 1, w, h, w, h, (att, xf, yf), src, dst_before, after)
    print("concentrate", n, w, h, att, hex(xf), hex(yf))

# sub_495640 fills the 16-bit weight table used by sub_495CA0.
call(0x495640, [], {})

# kind 2: sub_495680 (91:1C nearest), cdecl (destination, source, xs, ys).
# kind 3: sub_495880 (91:1C filtered), cdecl (source, xs, ys), ecx = dest.
scale_cases = [(8, 6, 0x10000, 0x10000), (8, 6, 0x18000, 0x20000), (16, 12, 0x8000, 0x8000),
               (16, 12, 0x6000, 0x5000), (10, 10, 0xC000, 0x9000), (12, 9, 0x24000, 0xA000),
               (7, 13, 0x3000, 0x28000), (20, 4, 0x10000, 0x8800)]
for _ in range(12):
    scale_cases.append((rng.randrange(1, 24), rng.randrange(1, 24),
                        rng.randrange(0x1000, 0x40000), rng.randrange(0x1000, 0x40000)))
for kind, address in ((2, 0x495680), (3, 0x495880)):
    for n, (w, h, xs, ys) in enumerate(scale_cases):
        for fmt in (1, 2):
            heap_top = global_heap
            dw, dh = (w * xs) >> 16, (h * ys) >> 16
            if dw == 0 or dh == 0:
                continue
            src = rand_pixels(w * h, None if fmt == 2 else 0)
            if fmt == 2:
                for i in range(0, len(src), 4 * 3):
                    src[i + 3] = 0
            dst_before = rand_pixels(dw * dh)
            src_desc, _ = bitmap(w, h, src, fmt)
            dst_desc, dst_data = bitmap(dw, dh, dst_before, fmt)
            if kind == 2:
                call(address, [dst_desc, src_desc, xs, ys], {})
            else:
                call(address, [src_desc, xs, ys], {UC_X86_REG_ECX: dst_desc})
            after = mu.mem_read(dst_data, dw * dh * 4)
            emit(f"scale{kind}_{n}_{fmt}", kind, w, h, dw, dh, (xs, ys, fmt), src, dst_before, after)
print("scales done")
