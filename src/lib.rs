//! `mktape` stems from the need to create a PDP-10 `tap` file to load source of another [project](https://nigeleke.github.io/monop) of mine, onto an emulated PDP-10.
//! As part of the research into making this work I was pointed to [this paper](https://opensimh.org/research-unix-7-pdp11-45-v2.0.pdf) and, in particular, the [perl script](https://www.tuhs.org/Archive/Distributions/Research/Keith_Bostic_v7/mktape.pl) referenced in Appendix A.
//! The [perl script](https://www.tuhs.org/Archive/Distributions/Research/Keith_Bostic_v7/mktape.pl) creates a pre-named `tap` file (`v7.tap`) from files `f0`..`f6`. More over, the input files have pre-defined block sizes for each of them.
//!
//! This project generalises the script so that the user can:
//! 
//!   * specify the output file (required)
//!   * specify the input file(s), minimally one file
//!   * specify the input file's block-sizes (defaults to 1024 to align with the `tapewrite` default from the [tapeutils](https://github.com/brouhaha/tapeutils) repository).
//!

mod args;
pub mod mktape;
pub mod result;
