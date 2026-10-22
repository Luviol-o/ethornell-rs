# PlayStation Vita

Native GXM frontend on vita2d's context. BGI scene textures, affine quads,
clipping, nearest/linear sampling and alpha/add/subtract blending run on the
GPU. Glyphs are rasterized by the shared font code and uploaded as textures;
the player does not composite the scene on the CPU.

Requires VitaSDK with `libvita2d`, cargo-vita 0.2.2, and
nightly-2026-09-18 with rust-src:

```sh
./platform/vita/build.sh
```

The supplied `docker/Dockerfile` provides the same build environment.
Output: `player/target/armv7-sony-vita-newlibeabihf/release/ethornell_vita.vpk`.
Install using VitaShell. Copy the original game into
`ux0:data/ethornell/game/` and a Japanese/CJK TrueType font to
`ux0:data/ethornell/font.ttf`. Saves are separate at `ux0:data/ethornell/saves/`;
diagnostics are in `player.log`.

Left stick moves the pointer: a light tilt for buttons, full tilt crosses
the screen in under a second. Cross clicks, Circle sends Escape, Square
Space, Triangle Enter, L Control, and the D-pad arrow keys. Front touch
holds/releases the same left button as Cross. Start+Select requests the
game's normal close path. Confirmations appear over the game and accept
Cross/Circle. Structured text-entry dialogs, Flash and desktop integration
report unsupported status. Game memory demands must fit Vita's extended
memory allocation; the player requests a 320 MiB heap.

The player links VitaSDK's published libraries. GXM programs are embedded,
so no runtime shader compiler or extra firmware module is needed. Shader
binaries and their matching Cg sources come from
[xmoezzz/siglus_rs](https://github.com/xmoezzz/siglus_rs/tree/5f358ef1360b0439e92aed2b749e8f25d8721954/crates/siglus_scene_vm/src/render/vita/shaders),
under MPL-2.0. Triple vertex rings and deferred texture retirement follow
that port's GPU lifetime handling. SLP vectorization is disabled to avoid
Vita3K's paired 64-bit saturation issue; loop vectorization stays enabled.
