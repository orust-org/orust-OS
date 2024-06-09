# Installation Manual

##### Before we go another further, it is important to note that this project is based on the @rust programming langauge so it won't run without you having rust locally on your machine.

## Install Rust:
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```
Verify the installation with
```sh
rustc --version
```
Output Example
```sh
rustc 1.65.0 (897e37553 2022-11-02)
```
##                                 Follow the instructions below to have the orust up and running.

# Mac_OS
Ensure to clone this repository first.
#### Install Homebrew if you don’t have it yet:
```sh
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```
#### Install QEMU using Homebrew:
```sh
brew install qemu
```
#### Verify the installation:
```sh
qemu-system-x86_64 --version
```