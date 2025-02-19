use crate::nightly::impl_const;
use core::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};

macro_rules! cmp {
    ($BUint: ident, $BInt: ident, $Digit: ident) => {
        impl_const! {
            impl<const N: usize> const PartialEq for $BUint<N> {
                #[inline]
                fn eq(&self, other: &Self) -> bool {
                    let mut i = 0;
                    while i < N {
                        if self.digits[i] != other.digits[i] {
                            return false;
                        }
                        i += 1;
                    }
                    true
                }
            }
        }

        impl_const! {
            impl<const N: usize> const Eq for $BUint<N> {}
        }

        impl_const! {
            impl<const N: usize> const PartialOrd for $BUint<N> {
                #[inline]
                fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                    Some(self.cmp(other))
                }
            }
        }

        impl_const! {
            impl<const N: usize> const Ord for $BUint<N> {
                #[inline]
                fn cmp(&self, other: &Self) -> Ordering {
                    let mut i = N;
                    while i > 0 {
                        i -= 1;
                        let a = self.digits[i];
                        let b = other.digits[i];

                        // Clippy: don't use match here as `cmp` is not yet const for primitive integers
                        #[allow(clippy::comparison_chain)]
                        if a > b {
                            return Ordering::Greater;
                        } else if a < b {
                            return Ordering::Less;
                        }
                    }
                    Ordering::Equal
                }

                #[inline]
                fn max(self, other: Self) -> Self {
                    match self.cmp(&other) {
                        Ordering::Less | Ordering::Equal => other,
                        _ => self,
                    }
                }

                #[inline]
                fn min(self, other: Self) -> Self {
                    match self.cmp(&other) {
                        Ordering::Less | Ordering::Equal => self,
                        _ => other,
                    }
                }

                #[inline]
                fn clamp(self, min: Self, max: Self) -> Self {
                    assert!(min <= max);
                    if let Ordering::Less = self.cmp(&min) {
                        min
                    } else if let Ordering::Greater = self.cmp(&max) {
                        max
                    } else {
                        self
                    }
                }
            }
        }
        #[cfg(test)]
        paste::paste! {
            mod [<$Digit _digit_tests>] {
                use crate::test::types::big_types::$Digit::*;
        		crate::int::cmp::tests!(utest);
            }
        }
    };
}

crate::macro_impl!(cmp);
