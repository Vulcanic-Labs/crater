# Crater

Tool to manage the entire Vulcanic Labs tooling

# Installation

### Linux Distros Installation

1. Install the Rust toolchain with `rustup`

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Then use `cargo install` to get `crater`

```sh
cargo install --git https://github.com/Vulcanic-Labs/crater
```

3. Done! You're good to go!

```sh
crater -t template-repo-test
```

### Windows Installation

1. Install the Rust toolchain [rustup-init.exe](https://win.rustup.rs/x86_64)
then

```sh
rustup toolchain install stable-x86_64-pc-windows-gnu
```
then followed by this command

```sh
rustup default stable-x86_64-pc-windows-gnu
```
>If you're a Windows Subsystem for Linux user run the following in your terminal, then follow the onscreen instructions to install Rust.

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Install [MSYS2](https://www.msys2.org/)
In the MSYS2 terminal run this command

```sh
pacman --sync mingw-w64-x86_64-gcc
```

Add `C:\msys64\mingw64\bin` to system `PATH`

Then restart your machine to fully install GCC

3. Then use `cargo install` to get `crater`

```sh
cargo install --git https://github.com/Vulcanic-Labs/crater
```

4. Done! You're good to go!

```sh
crater.exe -t template-repo-test
```