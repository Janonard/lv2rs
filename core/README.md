# lv2rs-core: Rust re-implementation of the LV2 core library.

This is a safe and idiomatic re-implementation of the LV2 core library. It goals are to provide an interface that is compatible with hosts written in C and uses an idiomatic API for plugin implementors.

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
lv2rs-core = "*"
```
 * Create a struct, implement `Plugin` for it, and use the `lv2_main!` macro to export the required symbols. For example:
```
extern crate lv2rs_core as core;
use std::ffi::CStr;

struct MyPlugin {}

impl core::Plugin for MyPlugin {
    fn instantiate(
        _descriptor: &core::Descriptor,
        _rate: f64,
        _bundle_path: &CStr,
        _features: Option<&[*mut core::Feature]>
    ) -> Self {
        Self {}
    }

    unsafe fn connect_port(&mut self, port: u32, data: *mut ()) {}

    fn run(&mut self, _n_samples: u32) {}
}

core::lv2_main!(core, MyPlugin, b"http://example.org/Dummy\0");
```

That's it! Although, if you really want to get started with LV2, you should check out the
[tutorial](http://lv2plug.in/book/) at LV2's website.