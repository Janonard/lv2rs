//! This is a prototype of an idiomatic library empowering you to create LV2-compatible plugins for audio applications with ease.
//!
//! ## How to use it?
//!
//! If you want to get started with LV2, you should checkout the
//! [tutorial](https://janonard.github.io/lv2rs-book/) first. It is a "translation" of the original
//! [LV2 Book](http://lv2plug.in/book/) by David Robillard, one of the creators of LV2, from C to
//! Rust.
//!
//! The core of the library is formed by the [`core`](https://docs.rs/lv2rs-core) crate, which
//! contains a trait and a macro that makes the creation of plugins easy. Then, there are the
//! [`atom`](https://docs.rs/lv2rs-atom) and the [`midi`](https://docs.rs/lv2rs-midi) crates, which
//! provide general data exchange and MIDI messages.
//!
//! ## What is supported, what isn't?
//!
//! Currently 4 out of 22 [official and stable LV2 specifications](http://lv2plug.in/ns/) are
//! supported. These are:
//! 
//! * Atom
//! * LV2
//! * MIDI
//! * URID
//! 
//! This is a frozen prototype and therefore, development of this crate will not continue here. Further
//! development continues as [rust-lv2](https://github.com/rust-dsp/rust-lv2).

pub extern crate lv2rs_atom as atom;
pub extern crate lv2rs_core as core;
pub extern crate lv2rs_midi as midi;
pub extern crate lv2rs_urid as urid;

/// Re-export module intended for wildcard use
///
/// Simply import it like that:
///
///     extern crate lv2rs as lv2;
///     use lv2::prelude::*;
pub mod prelude {
    pub use atom::prelude::*;
    pub use midi::prelude::*;
}
