"""Local source-only reproduction bundle; never include reference/game assets."""
import hashlib
import io
import json
from pathlib import Path
import subprocess
import tarfile

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / "dist/source"
OUT.mkdir(parents=True, exist_ok=True)
SOURCE_SUFFIXES = {".rs", ".toml", ".lock", ".wgsl", ".glsl", ".vert", ".frag", ".h", ".hpp", ".c", ".cpp", ".sh", ".py", ".md", ".txt", ".json", ".yml", ".yaml"}


def include(name):
    path = Path(name)
    if any(part in {"target", "dist", "raw", ".git", "testcase", "reverse", "out"} for part in path.parts):
        return False
    return path.suffix.lower() in SOURCE_SUFFIXES and (
        path.parts[0] in {"crates", "platform", ".cargo"}
        or name in {"Cargo.toml", "Cargo.lock", "README.md", "LICENSE", "LICENSE.md", "LICENSE.txt"}
    )


names = subprocess.check_output(["git", "ls-files", "--cached", "--others", "--exclude-standard", "-z"], cwd=ROOT).decode().split("\0")
names = sorted(set(name for name in names if name and include(name) and (ROOT / name).is_file() and not (ROOT / name).is_symlink()))
hashes = {name: hashlib.sha256((ROOT / name).read_bytes()).hexdigest() for name in names}
with tarfile.open(OUT / "ethornell-android-source.tar.gz", "w:gz") as archive:
    for name in names:
        data = (ROOT / name).read_bytes()
        entry = tarfile.TarInfo("ethornell-rs/" + name)
        entry.size = len(data)
        entry.mode = 0o755 if name.endswith(".sh") else 0o644
        entry.mtime = 0
        archive.addfile(entry, io.BytesIO(data))
patch = subprocess.check_output(["git", "diff", "--binary", "HEAD", "--", *names], cwd=ROOT)
(OUT / "ethornell-android-working-tree.patch").write_bytes(patch)
manifest = {
    "revision": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip(),
    "dirty": True,
    "publication": "BLOCKED: local technical validation, license/source clearance outstanding",
    "dependencies": "Cargo.lock pinned registry/git sources; not vendored",
    "source_file_sha256": hashes,
    "artifacts_sha256": {name: hashlib.sha256((OUT / name).read_bytes()).hexdigest() for name in ["ethornell-android-source.tar.gz", "ethornell-android-working-tree.patch"]},
}
(OUT / "source-manifest.json").write_text(json.dumps(manifest, indent=2) + "\n")
print(json.dumps(manifest["artifacts_sha256"], indent=2))
