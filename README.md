# ORUST
Official orust operting system build with rust by Enow Scott

# Minimal 64-bit Rust Kernel for x86 Architecture

In this project, we create a minimal 64-bit Rust kernel for a x86 architecture. We build upon the freestanding Rust binary from the previous post to create a bootable disk image that prints something to the screen.

This is open developing project on GitHub. If you have any problems or questions, please open an issue here. You can also leave comments.

## The Boot Process

When you turn on a computer, it begins executing firmware code that is stored in motherboard ROM. This code performs a power-on self-test, detects available RAM, and pre-initializes the CPU and hardware. Afterwards, it looks for a bootable disk and starts booting the operating system kernel.

On x86, there are two firmware standards: the “Basic Input/Output System“ (BIOS) and the newer “Unified Extensible Firmware Interface” (UEFI). The BIOS standard is old and outdated, but simple and well-supported on any x86 machine since the 1980s. UEFI, in contrast, is more modern and has much more features, but is more complex to set up (at least in my opinion).

Currently, we only provide BIOS support, but support for UEFI is planned, too. If you’d like to help us with this, check out the Github issue.

### BIOS Boot

Almost all x86 systems have support for BIOS booting, including newer UEFI-based machines that use an emulated BIOS. This is great, because you can use the same boot logic across all machines from the last century. But this wide compatibility is at the same time the biggest disadvantage of BIOS booting, because it means that the CPU is put into a 16-bit compatibility mode called real mode before booting so that archaic bootloaders from the 1980s would still work.

But let’s start from the beginning:

When you turn on a computer, it loads the BIOS from some special flash memory located on the motherboard. The BIOS runs self-test and initialization routines of the hardware, then it looks for bootable disks. If it finds one, control is transferred to its bootloader, which is a 512-byte portion of executable code stored at the disk’s beginning. Most bootloaders are larger than 512 bytes, so bootloaders are commonly split into a small first stage, which fits into 512 bytes, and a second stage, which is subsequently loaded by the first stage.

The bootloader has to determine the location of the kernel image on the disk and load it into memory. It also needs to switch the CPU from the 16-bit real mode first to the 32-bit protected mode, and then to the 64-bit long mode, where 64-bit registers and the complete main memory are available. Its third job is to query certain information (such as a memory map) from the BIOS and pass it to the OS kernel.

Writing a bootloader is a bit cumbersome as it requires assembly language and a lot of non-insightful steps like “write this magic value to this processor register”. Therefore, we don’t cover bootloader creation in this post and instead provide a tool named bootimage that automatically prepends a bootloader to your kernel.

If you are interested in building your own bootloader: Stay tuned, a set of posts on this topic is already planned!

### The Multiboot Standard

To avoid that every operating system implements its own bootloader, which is only compatible with a single OS, the Free Software Foundation created an open bootloader standard called Multiboot in 1995. The standard defines an interface between the bootloader and the operating system, so that any Multiboot-compliant bootloader can load any Multiboot-compliant operating system. The reference implementation is GNU GRUB, which is the most popular bootloader for Linux systems.

To make a kernel Multiboot compliant, one just needs to insert a so-called Multiboot header at the beginning of the kernel file. This makes it very easy to boot an OS from GRUB. However, GRUB and the Multiboot standard have some problems too:

- They support only the 32-bit protected mode. This means that you still have to do the CPU configuration to switch to the 64-bit long mode.
- They are designed to make the bootloader simple instead of the kernel. For example, the kernel needs to be linked with an adjusted default page size, because GRUB can’t find the Multiboot header otherwise. Another example is that the boot information, which is passed to the kernel, contains lots of architecture-dependent structures instead of providing clean abstractions.
- Both GRUB and the Multiboot standard are only sparsely documented.
- GRUB needs to be installed on the host system to create a bootable disk image from the kernel file. This makes development on Windows or Mac more difficult.

Because of these drawbacks, we decided to not use GRUB or the Multiboot standard. However, we plan to add Multiboot support to our bootimage tool, so that it’s possible to load your kernel on a GRUB system too. If you’re interested in writing a Multiboot compliant kernel, check out the first edition of this blog series.

### UEFI

(We don’t provide UEFI support at the moment, but we would love to! If you’d like to help, please tell us in the Github issue.)

## A Minimal Kernel

Now that we roughly know how a computer boots, it’s time to create our own minimal kernel. Our goal is to create a disk image that prints a “Hello World!” to the screen when booted. We do this by extending the previous post’s freestanding Rust binary.

As you may remember, we built the freestanding binary through cargo, but depending on the operating system, we needed different entry point names and compile flags. That’s because cargo builds for the host system by default, i.e., the system you’re running on. This isn’t something we want for our kernel, because a kernel that runs on top of, e.g., Windows, does not make much sense. Instead, we want to compile for a clearly defined target system.

### Installing Rust Nightly

Rust has three release channels: stable, beta, and nightly. The Rust Book explains the difference between these channels really well, so take a minute and check it out. For building an operating system, we will need some experimental features that are only available on the nightly channel, so we need to install a nightly version of Rust.

To manage Rust installations, I highly recommend rustup. It allows you to install nightly, beta, and stable compilers side-by-side and makes it easy to update them. With rustup, you can use a nightly compiler for the current directory by running `rustup override set nightly`. Alternatively, you can add a file called `rust-toolchain` with the content `nightly` to the project’s root directory. You can check that you have a nightly version installed by running `rustc --version`: The version number should contain `-nightly` at the end.

The nightly compiler allows us to opt-in to various experimental features by using so-called feature flags at the top of our file. For example, we could enable the experimental `asm!` macro for inline assembly by adding `#![feature(asm)]` to the top of our `main.rs`. Note that such experimental features are completely unstable, which means they might change at any time. Therefore, you should avoid using them for libraries or projects that are intended to be used by others.

### Cross-Compilation

To compile our kernel for the x86_64 architecture, we need a cross-compiler, i.e., a compiler that runs on one architecture but generates code for another one. Luckily, Rust makes cross-compilation really easy!

First, install the cross toolchain for our target system via rustup by running `rustup target add x86_64-unknown-none`. This adds a toolchain for the `x86_64-unknown-none` target, which is an x86_64 CPU without an operating system. The toolchain includes a linker and a C standard library that are compiled for our target system.

Next, we need to tell cargo to use the right target system. We do this by adding a file called `.cargo/config.toml` to the project’s root directory with the following content:

```toml
[build]
target = "x86_64-unknown-none"
```

With this configuration in place, cargo will use the x86_64-unknown-none target system for building the current project. Now we can compile our freestanding Rust binary for the x86_64 architecture with the following command:

[cargo build --release]

This will create a binary at target/x86_64-unknown-none/release/bootimage-blog_os.bin.

Bootable Disk Image 

To create a bootable disk image from our binary, we need to do two things:

    We need to create a partition table for the disk. This tells the bootloader where to find the kernel image on the disk.
    We need to copy the kernel image to the right location on the disk.

There are different partition table formats, but the most commonly used one is the Master Boot Record (MBR) format. An MBR consists of a 512-byte partition table followed by a small bootloader. The partition table contains four partition entries, each of which describes a partition on the disk. We only use one entry for simplicity.

Let’s use the bootimage tool to create a bootable disk image. First, install the tool via cargo:

[cargo install bootimage]

Now we can create a bootable disk image from our kernel binary with the following command:

[bootimage build --bin target/x86_64-unknown-none/release/bootimage-blog_os.bin]

This will create a disk image at target/x86_64-unknown-none/release/bootimage-blog_os.bin. You can write this disk image to a USB stick with dd and boot it on a real machine or in a virtual machine.
Conclusion

We’ve successfully created a minimal 64-bit Rust kernel for the x86 architecture that prints a “Hello World!” to the screen when booted. In the next step, we’ll learn about VGA text mode and use it to print text to the screen. Stay tuned!


This post is comprehensive, covering various aspects of creating a minimal 64-bit Rust kernel for the x86 architecture. It explains the boot process, BIOS boot, the Multiboot standard, and UEFI. Additionally, it provides practical steps for setting up the development environment, compiling the kernel, and creating a bootable disk image. Overall, it offers a solid foundation for anyone interested in kernel development with Rust on x86 architecture.
