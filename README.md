Oxide | Operating System built with Rust
========================================

[![Travis CI](https://img.shields.io/travis/iBelieve/oxide/master.svg)]()
[![Dependencies](https://img.shields.io/librariesio/github/iBelieve/oxide.svg)]()
[![GitHub tag](https://img.shields.io/github/tag/iBelieve/oxide.svg)]()
[![GitHub issues](https://img.shields.io/github/issues/iBelieve/oxide.svg)]()
[![Maintenance](https://img.shields.io/maintenance/yes/2017.svg)]()

My ramblings in the world of OS development using the [Rust programming language](http://rust-lang.org).

### Dependencies

 * Rust, installed using https://www.rustup.rs
 * [Xargo](https://github.com/japaric/xargo)
 * xorriso
 * autoconf
 * automake

### Setup

Build the cross-compiler toolchain and GRUB using:

    ./build_tools.sh

Use the latest nightly build of rust:

    rustup override add nightly

### Resources Used

 * Philipp Oppermann's [Writing an OS in Rust](http://os.phil-opp.com/) series of blog posts
 * Eric Kidd's [Bare Metal Rust](http://www.randomhacks.net/bare-metal-rust/) blog posts
