# Sudoku Pi

> Your new way to play Sudoku

<a href="https://apps.apple.com/us/app/sudoku-pi/id6467504425?itsct=apps_box_badge&amp;itscg=30200" style="display: inline-block; overflow: hidden; border-radius: 13px; width: 250px; height: 83px;"><img src="https://tools.applemediaservices.com/api/badges/download-on-the-app-store/black/en-us?size=250x83&amp;releaseDate=1696377600" alt="Download on the App Store" style="border-radius: 13px; width: 250px; height: 83px;"></a>
<a href="https://play.google.com/store/apps/details?id=nl.couch_chilis.sudoku_pi" style="display: inline-block; overflow: hidden; width: 240px; height: 71px;"><img alt="Get it on Google Play" src="https://appure.io/badges/playstore/png/en.png" height="60" /></a>

Sudoku Pi is a finger-friendly Sudoku game that takes a refreshing new approach
to how the game is played. Our aim is to minimize finger movements and to
prevent mistakes.

- Select numbers using a circular wheel.
- Numbers that cannot be filled in are disabled in the wheel by default.
- Effective note taking: Quickly "draw" multiple notes in a single movement.
- Select one of five difficulty levels.
- Highscore screen with random fortune quotes.
- No ads, forever.

The source code for Sudoku Pi is available for free under an open-source license
that grants you the right to make copies and make modifications, so long as your
copies are made available under the same terms.

If you want to support us, please buy the game in the
[App Store](https://apps.apple.com/us/app/sudoku-pi/id6467504425) or on
[Google Play](https://play.google.com/store/apps/details?id=nl.couch_chilis.sudoku_pi).

# Community

Please feel free to follow us or reach out to us!

- Mastodon: https://mastodon.gamedev.place/@couchchilis
- Discord: https://discord.gg/X3ZJvrNZw5

# Contributing

Feel free to open [issues](https://github.com/Couch-Chilis/Sudoku-Pi/issues) or
[pull requests](https://github.com/Couch-Chilis/Sudoku-Pi/pulls).

By opening a pull request, you grant Couch Chilis the right to use your
contribution in any way they see fit. This includes, but is not limited to,
distribution under the terms of the [General Public License v3](#license) as
well as in binary form for release to app stores.

We offer [paid bounties](https://github.com/Couch-Chilis/Sudoku-Pi/issues?q=is%3Aissue%20state%3Aopen%20label%3Abounty) for some issues!

# Build Instructions

## Desktop

0. Make sure you have the [Rust toolchain installed](https://rustup.rs/).
1. Use `cargo run` to run the game.

## Android

0. Make sure you have the Android SDK and NDK installed, as well as `cargo-ndk`.
1. Use the `build_android.sh` script to build the library.
2. Open this folder as a project in Android Studio. If the script above
   completed, it should compile, link, and run successfully.

## iOS

0. Make sure you have Xcode and a recent Rust toolchain installed.
1. Open `ios/Sudoku Pi.xcodeproj` in Xcode. It should compile, link, and run
   successfully.

The Xcode project is generated from `apple/project.yml` with
[XcodeGen](https://github.com/yonaskolb/XcodeGen):

```sh
brew install xcodegen
cd apple
xcodegen generate
```

# License

Sudoku Pi is copyright 2023-2026 by Couch Chilis and is available under the
[General Public License v3](LICENSE-GPLv3.md).

If you use the source code for your own project, we ask you kindly to consider
supporting us by buying the game in the [App Store](https://apps.apple.com/us/app/sudoku-pi/id6467504425).
