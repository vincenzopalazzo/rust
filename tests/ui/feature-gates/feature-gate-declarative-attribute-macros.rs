// check-pass
// edition:2021
// compile-flags: --test

#![feature(declarative_attribute_macros)]

macro_rules! my_attr {
    () => {};
}

#[my_attr]
fn test_fn() {}

fn main() {
    test_fn();
}
