//! This is a safe and idiomatic re-implementation of some LV2 libraries, with the goal of being
//! compatible to existing hosts written in C and providing an easy-to-use interface for plugin
//! implementors.
//!
//! ## Why "re-implemention", not "wrapping" or "binding"?
//!
//! In most cases, adaptations of C libraries for Rust are split into two crates: One provides a raw
//! interface to the library and one adapts them for idiomatic use. The reasoning behind this is
//! that some may want to directly interact with library for extra performance and therefore,
//! translation and adaptation should be split.
//!
//! Except, in the case of LV2, this does not really make sense: The LV2 library only consists of
//! type defintions and the only provided functions are inlined utility functions, which make little
//! to no sense to use in Rust. I therefore translated the struct and function definitions and built
//! Rust-style utilities around them. This get's closer to a re-implementation of the library than
//! to a binding and therefore, I named this crate "re-implementation".
//!
//!## What works, what doesn't?
//!
//!Currently, only the core library is implemented: You can create a plugin which can send and
//! receive audio data as well as paramaters, given as floating point numbers. That's not much,
//! but it's the foundation of everything else to come.
//!
//!The next libraries I'm going to implement are `URID`, `Atom` and `Midi`, which should cover
//! almost any use cases of LV2. I would consider this re-implementation done when one can write
//! every example in the [LV2 Book](http://lv2plug.in/book/) using this library.

pub extern crate lv2rs_core as core;
pub extern crate lv2rs_urid as urid;
