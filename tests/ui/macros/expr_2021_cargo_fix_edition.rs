//@  compile-flags: --edition=2021
// This test ensures that expr_2021 is not allowed on pre-2021 editions
#![deny(edition_2024_expr_fragment_specifier)]

macro_rules! m {
    ($e:expr) => { //~ ERROR: the `expr` fragment specifier will accept more expressions in the 2024 edition.
       //~^ WARN: this changes meaning in Rust 2024
        $e
    };
}

fn main() {
    m!(());
}

