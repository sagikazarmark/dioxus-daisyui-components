// `dx components add` writes `src/components/mod.rs` and appends a `pub mod`
// line per Component, but leaves referencing that module from the crate root to
// the consumer: the one manual step it prints on the way out. The module does
// not exist until an install has run, so this crate only compiles inside Seam 1.
//
// Nothing here calls the Components it installs, so every one of them is dead
// code; the allow keeps that from burying a real warning.
#![allow(dead_code, unused_imports)]

mod components;

fn main() {}
