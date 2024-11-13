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
#### Run orust:
Navigate into the orust directory through your terminal or IDE:

1. Add llvm-tools
```sh
rustup component add llvm-tools-preview
```
2. Install the bootimage.
```sh
cargo install bootimage
```
#### Build and Run orust:

```sh
cargo build
```
```sh
cargo run
```
# Windows
Ensure to clone this repository first.

#### Download the QEMU installer:
Go to the [official QEMU website](https://www.qemu.org/download/#windows) and download the Windows installer.
#### Run the installer:
Follow the instructions in the installation wizard to complete the setup.
#### Add QEMU to your system PATH:

    1. Right-click on "This PC" or "Computer" on the desktop or in File Explorer and select "Properties".

    2. Click on "Advanced system settings".

    3. In the System Properties window, click on the "Environment Variables" button.

    4. In the Environment Variables window, under "System variables", find the "Path" variable and select it, then click "Edit".

    5. In the Edit Environment Variable window, click "New" and add the path to your QEMU installation (e.g., C:\Program Files\qemu).
#### Verify the installation:
Open a new Command Prompt or PowerShell and run:
```sh
qemu-system-x86_64 --version
```
#### Run orust
Navigate into the orust folder through your IDE:

1. Add llvm-tools
```sh
rustup component add llvm-tools-preview
```
2. Install the bootimage.
```sh
cargo install bootimage
```
#### Build and Run orust:

```sh
cargo build
```
```sh
cargo run
```
# Linux(Ubuntu/Debian)
Ensure to clone this repository first.

#### Install QEMU:
```sh
sudo apt install qemu qemu-kvm
```
#### Verify the installation:
```sh
qemu-system-x86_64 --version
```
#### Run orust
Navigate into the orust directory through your terminal or IDE:

1. Add llvm-tools
```sh
rustup component add llvm-tools-preview
```
2. Install the bootimage.
```sh
cargo install bootimage
```
#### Build and Run orust:

```sh
cargo build
```
```sh
cargo run
```
