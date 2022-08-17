/// Code that cause the rust issue #95698
macro_rules! from_cow_impls {
    ($( $from: ty ),+ $(,)? ) => {
        // recursion call
        from_cow_impls!(
            $( $from, Cow::from ),+
        );
    };

    ($( $from: ty, $normalizer: expr ),+ $(,)? ) => {
        $( impl<'a> From<$from> for LhsValue<'a> {
            fn from(val: $from) -> Self {
                LhsValue::Bytes($normalizer(val))
            }
        } )+
    };
}

from_cow_impls!(&'a [u8], Vec<u8>,);
from_cow_impls!(
    (&'a str, |x: &'a str| Cow::from(x.as_bytes())),
    (String, |x: String| Cow::from(x.into_bytes())),
);

fn main() {}
