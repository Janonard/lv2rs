# lv2rs: Rust re-implementation of the LV2 libraries.

This is a safe and idiomatic re-implementation of some LV2 libraries, with the goal of being compatible to existing hosts written in C and providing an easy-to-use interface for plugin implementors.

## What works, what doesn't?

Currently, the following libraries are implemented:
* Core
* URID

The next libraries I'm going to implement are `Atom` and `Midi`, which should cover almost all use cases of LV2. I would consider this re-implementation done when one can write every example in the [LV2 Book](http://lv2plug.in/book/) using this library.

## Getting started

Creating a plugin binary is fairly simple:
 * Create a new library crate
 * Add the following entry to your `Cargo.toml` in order to build a dynamic library:
```
[lib]
crate-type = ["dylib"]
```
 * Add lv2rs-core as a dependency both in your `Cargo.toml`:
```
[dependencies]
lv2rs = "0.2.0"
```
 * Create a struct, implement `Plugin` for it, and use the `lv2_main!` macro to export the required symbols. For example:
```
extern crate lv2rs as lv2;
use std::ffi::CStr;

struct MyPlugin {}

impl lv2::core::Plugin for MyPlugin {
    fn instantiate(
        _descriptor: &lv2::core::Descriptor,
        _rate: f64,
        _bundle_path: &CStr,
        _features: Option<&[*mut lv2::core::Feature]>
    ) -> Option<Self> {
        Some(Self {})
    }

    unsafe fn connect_port(&mut self, port: u32, data: *mut ()) {}

    fn run(&mut self, _n_samples: u32) {}
}

use lv2::core;
core::lv2_main!(core, MyPlugin, b"http://example.org/Dummy\0");
```

That's it! Although, if you really want to get started with LV2, you should check out the
[tutorial](http://lv2plug.in/book/) at LV2's website.

## Why "re-implemention", not "wrapping" or "binding"?

In most cases, adaptations of C libraries for Rust are split into two crates: One provides a raw interface to the library and one adapts them for idiomatic use. The reasoning behind this is that some may want to directly interact with library for extra performance and therefore, translation and adaptation should be split.

Except, in the case of LV2, this does not really make sense: The LV2 library only consists of type defintions and the only provided functions are inlined utility functions, which make little to no sense to use in Rust. I therefore translated the struct and function definitions and built Rust-style utilities around them. This get's closer to a re-implementation of the library than to a binding and therefore, I named this crate "re-implementation".

## License

Although LV2 itself is published under it's own license, this crate is published under the terms of the LGPL v3; You may copy, modify and redistribute this software and your support is very welcome!