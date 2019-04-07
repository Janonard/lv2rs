# lv2rs-atom: Rust adaptation of the atom type system for LV2.

The purpose of this crate is to provide safe, idiomatic and easy-to-use means to use the type system introduced by the LV2 atom library. This type system is (relatively) portable and can be used to exchange information of arbitrary type among LV2 plugins.

## What are atoms?
On an abstract level, every atom consist of a header, which contains the URID of the atom type and a size in bytes, and body, a chunk of memory with the specified size. The interpretation of this body is dependent on the atom type and one of the features of this crate. Since this data is supposed to be "plain old data" and therefore must not contain references to other objects, the host does not need to "understand" the atoms; It simply copies the data.

## Getting started

If you want to get started with LV2, you should start with the [root crate](https://crates.io/crates/lv2rs) and check out the
[book](https://janonard.github.io/lv2rs-book/).