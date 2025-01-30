# Teeny Sokoban!
## An extremely tiny command like game written in rust

![image](https://cloud-fkwvk3die-hack-club-bot.vercel.app/0image.png)

### NOTE! ONLY `x86_64-linux` IS SUPPORTED, WINDOWS HAS DIFFERNT SYSTEM CALLS AND IS THEREFORE INCOMPATIBLE

Teeny Sokoban is a [sokoban](https://en.wikipedia.org/wiki/Sokoban) genre game, meaning it involves pushing boxes onto the correct spots in order to clear the level.
The game was written in rust, and does (almost) everything possible to make it as small of a binary as it can be. A stripped down version with only 1 level was created to be able to fit in a qr code.
Most of the code is safe rust, however due to being written in a `#![no_std]` context, lots of unsafe code and assembly is used.
The one dependency: `rustix`, was added because unfortunately I could not otherwise figure out how to put the terminal in raw mode.

## Gameplay
When you run the program, you start at level one. You can chose another level by passing the `-l` or `--level` flag
Your player is the little smiley face! You can use `WASD` to move him, or use `R` to reset the level
When you clear a level by pushing all the boxes onto the green goals, the next level will automatically start.

## Building Steps
You can play the game by just running cargo run. This will compile the `qrrust-bloated` executable, but not shrink and minify it.

The minification and shrinking steps use the [nix package manager](https://nixos.org/) to compile the executable and minify it. This makes it way more reproducable than a simple makefile or bash script, however you will need to have `nix` installed and the experimental `flakes` feature enabled.

You can run the game in all of it's tiny glory by running `nix run`, or build a binary with `nix build`
