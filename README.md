# Ambient Getting Started Tutorial

Ambient is a game engine written in Rust, focusing on modding and multiplayer features.

In this tutorial, we will write a simple shooting game from scratch, and in the process, we will show you the basic features of the engine, providing resources such as documentation, reference, examples, games, etc. Hopefully this will give you a good idea of how to use Ambient to make your own game.

## Prerequisites

Rust 1.67 or later is required to build Ambient. You can install Rust by following the instructions on [rustup.rs](https://rustup.rs/).

Rust programming experience is not required to follow this tutorial, but it is recommended. If you are new to Rust, you can learn the basics of the language on [rust-lang.org](https://www.rust-lang.org/learn).

After installing `rust`, you should be able to use command line tool including `rustup` and `cargo`.

You need to run `rustup target add --toolchain stable wasm32-wasi` in your Terminal. This will install the WebAssembly target for the WASI platform, which is required to build Ambient.

Then you can use the `cargo install` command to install Ambient:

```shell
cargo install --git https://github.com/AmbientRun/Ambient.git --rev ad24915e58 --locked --force ambient
```

Done!

> If you prefer other methods for installation, see [here](https://ambientrun.github.io/Ambient/user/installing.html).

It's also recommended to setup your IDE for a better Ambient dev experience (see [here](https://ambientrun.github.io/Ambient/user/setting_up_ide.html)).

## Tutorial structure

This tutorial is divided into several chapters. Each of them comes with a full Ambient project code with explainations. You can find them in each branch of this repository, or you can click the links below to see:

- [Chapter 1: Project Structure](https://github.com/AmbientRun/Tutorial/tree/chapter-1-project-structure)
- [Chapter 2: Colliders and Async](https://github.com/AmbientRun/Tutorial/tree/chapter-2-async)
- [Chapter 3: Spawn query](https://github.com/AmbientRun/Tutorial/tree/chapter-3-spawn-query)
- [Chapter 4: Message](https://github.com/AmbientRun/Tutorial/tree/chapter-4-message)
- [Chapter 5: Input from users](https://github.com/AmbientRun/Tutorial/tree/chapter-5-input)
- [Chapter 6: Move character with physics](https://github.com/AmbientRun/Tutorial/tree/chapter-6-physics-move-character)
- [Chapter 7: Model import](https://github.com/AmbientRun/Tutorial/tree/chapter-7-model-import)
- [Chapter 8: Animation](https://github.com/AmbientRun/Tutorial/tree/chapter-8-animation)
- [Chapter 9: Bind a gun](https://github.com/AmbientRun/Tutorial/tree/chapter-9-bind-gun)
- [Chapter 10: Bind the camera](https://github.com/AmbientRun/Tutorial/tree/chapter-10-bind-cam)
- [Chapter 11: UI](https://github.com/AmbientRun/Tutorial/tree/chapter-11-ui)
- [Chapter 12: Raycast](https://github.com/AmbientRun/Tutorial/tree/chapter-12-raycast)
