//! MIDI message integration for [`lv2rs-atom`](https://docs.rs/lv2rs-atom/).
//!
//! This crate introduces two new atom types for the use with `lv2rs-atom`:
//! [`RawMidiMessage`](atom/struct.RawMidiMessage.html) and
//! [`SystemExclusiveMessage`](atom/struct.SystemExclusiveMessage.html), as well as means to use
//! them. Using these structs, one can read from external controlling devices or keyboards or
//! controll synthesizers or even create a light show!
//! 
//! This is a frozen prototype and therefore, development of this crate will not continue here. Further
//! development continues as [rust-lv2](https://github.com/rust-dsp/rust-lv2).
//!
//! This crate depends on the non-standard integer types introduced by the `ux` crate, but you don't
//! need to depend on it too. The required types are exported too.
extern crate lv2rs_atom;
extern crate lv2rs_urid;
extern crate ux;

mod atom;
mod message;
pub mod status_bytes;
pub mod uris;

/// Re-export module intended for wildcard use
///
/// Simply import it like that:
///
///     extern crate lv2rs_midi as midi;
///     use midi::prelude::*;
pub mod prelude {
    pub use ux::{u14, u3, u4, u7};
}

pub use atom::*;
pub use message::*;
