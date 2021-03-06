// Copyright 2014 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![crate_name = "string_cache"]
#![crate_type = "rlib"]

#![cfg_attr(test, deny(warnings))]
#![cfg_attr(all(test, feature = "unstable"), feature(test, filling_drop))]
#![cfg_attr(feature = "unstable", feature(unsafe_no_drop_flag, plugin))]
#![cfg_attr(feature = "heap_size", feature(plugin, custom_derive))]
#![cfg_attr(feature = "unstable", plugin(string_cache_plugin))]
#![cfg_attr(feature = "heap_size", plugin(heapsize_plugin))]

#[cfg(all(test, feature = "unstable"))]
extern crate test;

#[macro_use]
extern crate lazy_static;

#[cfg(test)]
extern crate rand;

#[cfg(feature = "log-events")]
extern crate rustc_serialize;

#[cfg(feature = "heap_size")]
extern crate heapsize;

extern crate serde;

extern crate string_cache_shared;

pub use atom::Atom;
pub use namespace::{Namespace, QualName};

#[macro_export]
macro_rules! qualname (($ns:tt, $local:tt) => (
    ::string_cache::namespace::QualName {
        ns: ns!($ns),
        local: atom!($local),
    }
));

#[cfg(not(feature = "unstable"))]
include!(concat!(env!("OUT_DIR"), "/ns_atom_macros_without_plugin.rs"));

#[cfg(feature = "log-events")]
#[macro_use]
pub mod event;

pub mod atom;
pub mod namespace;

// A private module so that macro-expanded idents like
// `::string_cache::atom::Atom` will also work in this crate.
//
// `libstd` uses the same trick.
#[doc(hidden)]
mod string_cache {
    pub use atom;
    pub use namespace;
}
