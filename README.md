# lv2rs: Rust library for the creation of LV2 plugins.

[![Build Status](https://travis-ci.com/Janonard/lv2rs.svg?branch=master)](https://travis-ci.com/Janonard/lv2rs)

This is a idiomatic library empowering you to create LV2-compatible plugins for audio applications with ease.

## What works, what doesn't?

Currently 4 out of 22 [official and stable LV2 specifications](http://lv2plug.in/ns/) are
supported, with more being in the works. These are:

* Atom
* LV2
* MIDI
* URID

As you can see, this library is far from complete. The current development goal is to be able to write all examples of the [LV2 book](http://lv2plug.in/book/) in Rust. Some examples are already implemented and are hosted together with a translation of the said book on [GitHub](https://janonard.github.io/lv2rs-book/). After that goal is achieved, this library is considered more or less complete, although further development may continue afterwards.

However, deprecated specifications will never be supported and some only affect the declarative part of the standard. Therefore, some specifications will never be supported. Also, I haven't looked at non-standard specifications yet.

## Getting started

If you want to get started with LV2 and Rust, you should check out the book, hosted on [GitHub](https://github.com/Janonard/lv2rs-book)

## License

lv2rs is published under the [ISC](https://opensource.org/licenses/ISC) license, just like LV2 itself!