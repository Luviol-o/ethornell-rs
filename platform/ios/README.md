# iOS

A SwiftUI app with a game list and a player, around the engine library
`libethornell_mobile.a` (`crates/ethornell-mobile`). iOS / iPadOS 15 or later.

## Playing

1. Install `ethornell-ios-unsigned.ipa` from the releases after signing it:
   AltStore or SideStore sign it with your Apple ID, or use your own
   certificate. (Or build and run from Xcode, below.)
2. Copy the game folder (the one with the `.arc` files) into **Ethornell**
   with the Files app, or with Finder (iPhone > Files), or use the import
   button in the app.
3. Tap the game. Long-press it to choose the text encoding (GBK for Chinese
   translations) or to delete it.

| Gesture | Game input |
| --- | --- |
| tap, drag | left mouse button |
| two-finger tap | right click (the game menu) |
| two-finger vertical drag | mouse wheel (the backlog) |
| three-finger tap | close the game |

Saves are kept in the app's Application Support, apart from the game files.
A hardware keyboard works too.

## Building

```sh
platform/ios/build.sh          # simulator app: build/sim/Ethornell.app
platform/ios/build.sh device   # build/Ethornell-unsigned.ipa
```

Needs Xcode, rustup and XcodeGen (`brew install xcodegen`). The script
builds `libethornell_mobile.a` for device and simulator into
`Vendor/Ethornell.xcframework` and generates `Ethornell.xcodeproj` from
`project.yml`. To run on your own device, open the generated project, pick
your team under Signing, and run.

## Layout

| Path | |
| --- | --- |
| `Ethornell/` | Swift sources: launcher, player, game library, engine wrapper |
| `include/ethornell_ios.h` | the engine's C ABI (the Android ABI with a `UIView`) |
| `project.yml` | XcodeGen project |
