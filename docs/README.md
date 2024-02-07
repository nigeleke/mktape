# mktape

[![MIT License](https://img.shields.io/github/license/nigeleke/mktape?style=plastic)](https://github.com/nigeleke/mktape/blob/main/LICENCE.md)
[![Language](https://img.shields.io/badge/language-Rust-blue.svg?style=plastic)](https://www.rust-lang.org/)
[![Build](https://img.shields.io/github/actions/workflow/status/nigeleke/mktape/acceptance.yml?style=plastic)](https://github.com/nigeleke/mktape/actions/workflows/acceptance.yml)
[![Coverage](https://img.shields.io/codecov/c/github/nigeleke/mktape?style=plastic)](https://codecov.io/gh/nigeleke/mktape)
![Version](https://img.shields.io/github/v/tag/nigeleke/mktape?style=plastic)

  [Site](https://nigeleke.github.io/mktape) \| [GitHub](https://github.com/nigeleke/mktape) \| [API](https://nigeleke.github.io/mktape/api/mktape/index.html) \| [Coverage Report](https://nigeleke.github.io/mktape/coverage/index.html)


Create a [simh](https://opensimh.org/) tape file to be used on PDP-10.

Written by Nigel Eke, based on the perl script written by Will Senn which was, in turn, inspired by various Perl scripts and based on Hellwig Geisse's `mktape.c`.

__This project probably has very limited use to most people, but users for the PDP-x emulations may find it of interest.__

## Background

This project stems from the need to create a PDP-10 `tap` file to load source of another [project](https://nigeleke.github.io/monop) of mine, onto an emulated PDP-10.

As part of the research into making this work I was pointed to [this paper](https://opensimh.org/research-unix-7-pdp11-45-v2.0.pdf) and, in particular, the [perl script](https://www.tuhs.org/Archive/Distributions/Research/Keith_Bostic_v7/mktape.pl) referenced in Appendix A.

The [perl script](https://www.tuhs.org/Archive/Distributions/Research/Keith_Bostic_v7/mktape.pl) creates a pre-named `tap` file (`v7.tap`) from files `f0`..`f6`. More over, the input files have pre-defined block sizes for each of them.

This project generalises the script so that the user can:

  * specify the output file (required)
  * specify the input file(s), minimally one file
  * specify the input file's block-sizes (defaults to 1024 to align with the `tapewrite` default from the [tapeutils](https://github.com/brouhaha/tapeutils) repository).

## TODO:

  * Amend to create `backup` format files.
  * Amend to list contents of `tap` and `backup` files. 
  * Possibly amend to extract contents of `tap` and `backup` files. 
 
## Development pre-requisites

Normally I would use [nix](https://nixos.org/) to set up the development environment, but I found in this case, [rust](https://www.rust-lang.org/) didn't play well with some symlinks used by nix, so [rust](https://www.rust-lang.org/) is required.

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install cargo-tarpaulin
```

## Build & develop

```
> cargo build
```

Unit tests:
```
> cargo test --lib -- --test-threads=1
```

Integration tests require test files to be downloaded and stored in the `data/` folder. See the `README.md` in that folder.
```
> cargo test -- --test-threads=1
```

Note: 

  1. `--test-threads=1` is required as one of the test sets an environment variable whose presence my impact the other tests.

## Publish

```
> cargo build -r --bin mktape
> sudo cp target/release/mktape /usr/local/bin/
```

## Usage

Run `mktape --help` for additional help. In summary:

```
> mktape <TAPE> list
> mktape <TAPE> create <INPUT>[:<BLOCKSIZE]>...
```

where:

  * `<TAPE>` is `[-f <FORMAT>] <PATH>`
  * `<FORMAT>` is `tap` or `backup`, and defaults to `tap`
  * Default `<BLOCKSIZE>` is 1024 if not provided; The environment variable `MKTAPE_BLOCK_SIZE` can be set to change the default.
