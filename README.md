
<p align="center">
  <img src="src/image.png" alt="alt text" />
</p>

[![CI](https://github.com/orust-org/orust-OS/actions/workflows/rust.yml/badge.svg)](https://github.com/orust-org/orust-OS/blob/main/.github/workflows/rust.yml)
![GitHub Release](https://img.shields.io/github/v/release/orust-org/orust-OS?label=latest%20release)
![GitHub Repo stars](https://img.shields.io/github/stars/orust-org/orust-OS?style=flat)
![GitHub commit activity](https://img.shields.io/github/commit-activity/m/orust-org/orust-OS)
[![Rust Embedded](https://img.shields.io/badge/Rust%20Embedded-Book-blue.svg)](https://docs.rust-embedded.org/book/)
[![rust-lang](https://img.shields.io/badge/rust%202.0-svg-badge.svg)](https://github.com/rust-lang/rust)

##
This is a baremetal operating system that can boot off of a USB stick on any BIOS-compatible machine, which is pretty amazing. I'm going to be running the Operating System using QEMU instead of booting a physical machine. Just to keep things simple. If you have questions, visit [Discussions](https://github.com/orust-org/orust-OS/discussions) or open an issue.

##### <a href="https://github.com/Blindspot22">Enow Scott</a>

## How to Install and Use.

Visit the [Installation Manual](https://github.com/orust-org/orust-OS/blob/main/docs/Installation-Manual.md)for guidance on how to Install and use Orust

## Where is the code?

The code for each post lives in separate sections. This makes it possible to see the intermediate state after each post.

The code for the latest post is available <a href="https://github.com/orust-org/orust-OS/">here.</a>.


You can find the branch for each post by following the (source code) link in the <a href="https://github.com/orust-org/orust-OS/tree/main?tab=readme-ov-file#posts">Post list</a> below. The branches are named post-XX where XX is the post number, for example post-03 for the VGA Text Mode post or post-07 for the Hardware Interrupts post. For build instructions, see the Readme of the respective branch.

## Posts

The goal of this project is to provide step-by-step tutorials in individual blog posts. We currently have the following set of posts:


### Bare Bones:

. <a href="https://os.phil-opp.com/freestanding-rust-binary/">A Freestanding Rust Binary</a> (<a href="https://github.com/orust-org/orust-OS">source code</a>)

. <a href="https://os.phil-opp.com/minimal-rust-kernel/">A Minimal Rust Kernel</a> (<a href="https://github.com/orust-org/orust-OS">source code</a>)

. <a href="https://os.phil-opp.com/vga-text-mode/">VGA Text Mode</a> (<a href="https://github.com/orust-org/orust-OS">source code</a>)

. <a href="https://os.phil-opp.com/testing/">Testing </a> (<a href="https://github.com/orust-org/orust-OS">source code</a>)
### Interrupts:

. <a href="https://os.phil-opp.com/cpu-exceptions/">CPU Exceptions</a> (<a href="https://github.com/orust-org/orust-OS">source code</a>)

. <a href="https://os.phil-opp.com/double-fault-exceptions/">Double Faults</a> (<a href="https://github.com/orust-org/orust-OS">source code</a>)

. <a href="https://os.phil-opp.com/hardware-interrupts/">Hardware Interrupts</a> (<a href="https://github.com/orust-org/orust-OS">source code</a>)

### Memory Management:

. <a href="https://os.phil-opp.com/paging-introduction/">Introduction to Paging</a> (<a href="https://github.com/orust-org/orust-OS">source code</a>)

. <a href="https://os.phil-opp.com/paging-implementation/">Paging Implementation</a> (<a href="https://github.com/orust-org/orust-OS">source code</a>)

. <a href="https://os.phil-opp.com/paging-introduction/">Heap Allocation</a> (<a href="https://github.com/orust-org/orust-OS">source code</a>)

. <a href="https://os.phil-opp.com/paging-introduction/">Allocator Designs</a> (<a href="https://github.com/orust-org/orust-OS">source code</a>)

### Multitasking:

. <a href="https://os.phil-opp.com/async-await/">Async/Await</a> (<a href="https://github.com/orust-org/orust-OS">source code</a>)

## License

This project, with exception of the blog/content folder, is licensed under either of

  . Apache License, Version 2.0 (LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0)
  
  . MIT license (LICENSE-MIT or https://opensource.org/licenses/MIT)

at your option.
