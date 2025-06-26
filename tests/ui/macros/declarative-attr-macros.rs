//@ run-pass
//! Test for declarative attribute macros (RFC #1594)
//!
//! This test verifies that we can match on attribute macros using the new `:attr` matcher
//! in declarative macros and use the `#[declarative_attribute_macros]` feature gate.
#![allow(incomplete_features)]
#![feature(declarative_attribute_macros)]

#[allow(unused_macros)] // FIXME: temporary
macro_rules! main {
    attr ($func:item) => { println!("No threads"); };
    // attr(threads = $threads:literal) ($func:item) => { println!("Threads: {}", $threads); };
}

// #[main]
fn main() { }

//#[main(threads = 42)]
//fn main() {  }

// Scratch
//
// You can kind of think of declarative macros as being implemented in declarative macros:
//
// ```
// macro_rules! macro_rules {
//     ($($lhs:tt => $rhs:tt)* $(;)*) => compile_declarative_macro!($($lhs)*, $($rhs)*);
// }
// ```
//
// It looks like we'll have to rewrite macro_rules as:
//
// ```
// macro_rules! macro_rules {
//     (($($lhs:tt)*) => $rhs:tt) => { compile_declarative_macro!(($($lhs)*), $rhs) };
//     ([$($lhs:tt)*] => $rhs:tt) => { compile_declarative_macro!(($($lhs)*), $rhs) };
//     ({$($lhs:tt)*} => $rhs:tt) => { compile_declarative_macro!(($($lhs)*), $rhs) };
// }
// ```
//
// That's close to what we want, but it wouldn't allow you to mix rules with different delimiters.
