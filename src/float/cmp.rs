use super::Float;
use crate::{Bint, BUint};
use core::cmp::{PartialOrd, PartialEq, Ordering};

impl<const W: usize, const MB: usize> Float<W, MB> {
    #[inline]
    pub const fn max(self, other: Self) -> Self {
        handle_nan!(other; self);
        handle_nan!(self; other);
        if self < other {
            other
        } else {
            self
        }
    }

    #[inline]
    pub const fn min(self, other: Self) -> Self {
        handle_nan!(other; self);
        handle_nan!(self; other);
        if self > other {
            other
        } else {
            self
        }
    }

    #[inline]
    pub const fn maximum(self, other: Self) -> Self {
        handle_nan!(self; self);
        handle_nan!(other; other);
        if let Ordering::Less = self.total_cmp(&other) {
            other
        } else {
            self
        }
    }

    #[inline]
    pub const fn minimum(self, other: Self) -> Self {
        handle_nan!(self; self);
        handle_nan!(other; other);
        if let Ordering::Greater = self.total_cmp(&other) {
            other
        } else {
            self
        }
    }

    #[inline]
    pub const fn clamp(self, min: Self, max: Self) -> Self {
        assert!(min <= max);
        let mut x = self;
        if x < min {
            x = min;
        }
        if x > max {
            x = max;
        }
        x
    }

    #[inline]
    pub const fn total_cmp(&self, other: &Self) -> Ordering {
        let left = self.to_int();
        let right = other.to_int();
        if left.is_negative() && right.is_negative() {
            Bint::cmp(&left, &right).reverse()
        } else {
            Bint::cmp(&left, &right)
        }
    }
}

impl<const W: usize, const MB: usize> const PartialEq for Float<W, MB> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        handle_nan!(false; self, other);
        (self.is_zero() && other.is_zero()) || BUint::eq(&self.to_bits(), &other.to_bits())
    }
}

impl<const W: usize, const MB: usize> const PartialOrd for Float<W, MB> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        handle_nan!(None; self, other);
        if self.is_zero() && other.is_zero() {
            return Some(Ordering::Equal);
        }
        Some(self.total_cmp(other))
    }
}

#[cfg(test)]
mod tests {
	use crate::test::test_bignum;

    test_bignum! {
        function: <f64>::max(a: f64, b: f64)
    }
    test_bignum! {
        function: <f64>::min(a: f64, b: f64)
    }
    test_bignum! {
        function: <f64>::maximum(a: f64, b: f64)
    }
    test_bignum! {
        function: <f64>::minimum(a: f64, b: f64)
    }
    test_bignum! {
        function: <f64>::clamp(a: f64, b: f64, c: f64),
        skip: !(b <= c)
    }

    #[test]
    fn maximum() {
        let f1 = 0f64;
        let f2 = -0f64;
        //println!("{:064b}", ((-0.0f64).div_euclid(f2)).to_bits());
        let a = (crate::F64::from(f1).min(crate::F64::from(f2))).to_bits();
        let b = (f1.min(f2)).to_bits();
        /*println!("{:064b}", a);
        println!("{:064b}", b);*/
        assert!(a == b.into());
    }

    test_bignum! {
        function: <f64>::total_cmp(a: ref &f64, b: ref &f64)
    }
    test_bignum! {
        function: <f64>::partial_cmp(a: ref &f64, b: ref &f64)
    }
    test_bignum! {
        function: <f64>::eq(a: ref &f64, b: ref &f64)
    }
}