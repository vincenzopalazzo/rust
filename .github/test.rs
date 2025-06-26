//! This is a test file do not merge!

macro_rules! m {
    ($($lhs:tt => $rhs:tt);*$(;)?)
}

fn main() {}
    m! (
        ()
    )
}
