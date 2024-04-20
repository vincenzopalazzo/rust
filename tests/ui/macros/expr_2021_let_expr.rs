//@ revisions: edi2021 edi2024
//@[edi2024]compile-flags: --edition=2024 -Z unstable-options
//@[edi2021]compile-flags: --edition=2021

// This test ensures that the inline const match only on edition 2024
#![feature(let_chains)]
#![feature(expr_fragment_specifier_2024)]
#![allow(incomplete_features)]

macro_rules! m2021 {
    ($e:expr_2021) => {
        $e
    };
}

macro_rules! m2024 {
    ($e:expr) => {
        $e
    };
}

enum Flower {
    Rose,
    Violet,
}

fn main() {
    // FIXME: what is the let syntax here>
    let first_flower = Flower::Rose;
    let second_flower = Flower::Violet;
    m2021!(if let Flower::Rose = first_flower
           && let Flower::Violet = second_flower
           {}); //~ ERROR: no rules expected the token `let`
    m2024!(if let Flower::Rose = first_flower
           && let Flower::Violet = second_flower
           {}); //[edi2021]~ERROR: no rules expected the token `let`
}

