# ethornell-rs

`ethornell-rs` is a reimplementation of the Ethornell / BURIKO General Interpreter (a.k.a. "BGI") visual novel engine.

The runtime detects the native game identifier from the comparison in
`system.arc:ipl._bp`. If detection is unavailable, the identifier falls back to
`Tayutama2TV`. For example:

```sh
cargo run -p ethornell-cli -- run --game /path/to/sun_flower
```

`--game` locates the resource archives, loose files, and user data.

Without arguments (a double-click), `ethornell` asks for the game folder;
`ethornell /path/to/sun_flower` plays it directly.

Builds for Windows (x86_64), Linux (x86_64 and AArch64), macOS (a universal
`Ethornell.app`), Android (APK) and iOS (unsigned IPA) are published on
[GitHub Releases](https://github.com/asukaminato0721/ethornell-rs/releases).
The `pre-release` entry is updated after successful branch pushes; `v*` tags
publish versioned releases.
See [`platform/`](platform/README.md) for building and using each platform,
including the Android and iOS apps.

The window uses the game's icon from `icon.png`, an `.ico` file, or the icon
resources embedded in its Windows executable, in that order.

For a GBK Chinese translation, also set `ETHORNELL_TEXT_ENCODING=gbk`.
This selects the encoding of text buffers passed to the graphics API, including
dialogue and speaker names. The default is Shift-JIS (`cp932`). It does not
change the encoding of resource names or BP system strings.
