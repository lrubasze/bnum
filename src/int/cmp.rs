#[cfg(test)]
macro_rules! tests {
    ($int: ty) => {
        use crate::test::{test_bignum, types::*};

        test_bignum! {
            function: <$int>::eq(a: ref &$int, b: ref &$int)
        }
        test_bignum! {
            function: <$int>::partial_cmp(a: ref &$int, b: ref &$int)
        }

        test_bignum! {
            function: <$int>::cmp(a: ref &$int, b: ref &$int)
        }
        test_bignum! {
            function: <$int>::max(a: $int, b: $int)
        }
        test_bignum! {
            function: <$int>::min(a: $int, b: $int)
        }
        test_bignum! {
            function: <$int>::clamp(a: $int, min: $int, max: $int),
            skip: min > max
        }
    };
}

#[cfg(test)]
pub(crate) use tests;
