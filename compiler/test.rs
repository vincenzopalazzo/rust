//! This is a test file do not merge!

macro_rules! m {
    arbitrary_ident => { rule };
    // ($($(attr($($attr:tt)*))? ($($lhs:tt)*) => $rhs:tt);*$(;)?) => { $( println!("Rule: attr: {}, lhs: {}, rhs: {}", stringify!(($($($attr)*)*)), stringify!(($($lhs)*)), stringify!($rhs)); )* }
    ($($($clause:ident ($($clause_args:tt)*))? ($($lhs:tt)*) => $rhs:tt);*$(;)?) => {
        $(
            println!(
                "Rule: clause: {}, lhs: {}, rhs: {}",
                stringify!($($clause $($clause_args)*)?),
                stringify!($($lhs)*),
                stringify!($rhs)
            );
        )*
    };
    ($($($clause:ident ($($clause_args:tt)*))? {$($lhs:tt)*} => $rhs:tt);*$(;)?) => {
        $(
            println!(
                "Rule: clause: {}, lhs: {}, rhs: {}",
                stringify!($($clause $($clause_args)*)?),
                stringify!($($lhs)*),
                stringify!($rhs)
            );
        )*
    };
}

fn main() {
    m! {
        {$y:ident} => ($y);
        {42} => (0x42);
        attr($x:ident) {$y:ident} => ($x $y);
    }
}
