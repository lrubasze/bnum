use super::Float;
use crate::digit::Digit;
use core::num::FpCategory;
use crate::BUint;

struct Masks<const W: usize, const MB: usize>;

impl<const W: usize, const MB: usize> Masks<W, MB> {
    const Q_NAN_MASK: BUint<W> = Float::<W, MB>::NAN.to_bits();
    const FINITE_MASK: BUint<W> = Float::<W, MB>::INFINITY.to_bits();
}

impl<const W: usize, const MB: usize> Float<W, MB> {
    #[inline]
    pub const fn is_sign_positive(self) -> bool {
        !self.is_sign_negative()
    }

    #[inline]
    pub const fn is_sign_negative(self) -> bool {
        self.to_int().is_negative()
    }

    #[inline]
    pub const fn is_finite(self) -> bool {
        self.to_bits() & Masks::<W, MB>::FINITE_MASK != Masks::<W, MB>::FINITE_MASK
    }

    #[inline]
    pub const fn is_infinite(self) -> bool {
        self.abs().to_bits() == Masks::<W, MB>::FINITE_MASK
        /*let bits = self.abs().to_bits();
        bits.trailing_zeros() == Self::MB && bits.count_ones() == Self::EXPONENT_BITS as ExpType*/
    }

    #[inline]
    pub const fn is_nan(self) -> bool {
        //!(self.mantissa().is_zero() || self.is_finite())
        !self.is_finite() && self.to_bits().trailing_zeros() < Self::MB
    }

    #[inline]
    pub const fn is_quiet_nan(self) -> bool {
        self.to_bits() & Masks::<W, MB>::Q_NAN_MASK == Masks::<W, MB>::Q_NAN_MASK
    }

    #[inline]
    pub const fn is_quiet_indefinite_nan(self) -> bool {
        self == Self::NAN
    }

    #[inline] 
    pub const fn is_signalling_nan(self) -> bool {
        self.to_bits() & Masks::<W, MB>::Q_NAN_MASK == Self::INFINITY.to_bits()
    }

    #[inline]
    pub const fn is_subnormal(self) -> bool {
        /*!self.is_zero() && self.exponent().is_zero()*/
        let lz = self.abs().to_bits().leading_zeros();
        lz < Self::BITS && lz > Self::EXPONENT_BITS
    }

    #[inline]
    pub const fn is_normal(self) -> bool {
        matches!(self.classify(), FpCategory::Normal)
    }

    #[inline]
    pub const fn is_zero(&self) -> bool {
        let mut i = 0;
        while i < W - 1 {
            if self.words()[i] != 0 {
                return false;
            }
            i += 1;
        }
        let last = self.words()[W - 1];
        last.trailing_zeros() >= Digit::BITS - 1
    }

    #[inline]
    pub const fn classify(self) -> FpCategory {
        let u = self.abs().to_bits();
        if u.is_zero() {
            FpCategory::Zero
        } else if u == Self::INFINITY.to_bits() {
            FpCategory::Infinite
        } else {
            let u = u & Masks::<W, MB>::FINITE_MASK;
            if u.is_zero() {
                FpCategory::Subnormal
            } else if u == Masks::<W, MB>::FINITE_MASK {
                FpCategory::Nan
            } else {
                FpCategory::Normal
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::F64;
	use crate::test::test_bignum;

    test_bignum! {
        function: <f64>::is_sign_positive(a: f64)
    }
    test_bignum! {
        function: <f64>::is_sign_negative(a: f64)
    }
    test_bignum! {
        function: <f64>::is_finite(a: f64)
    }
    test_bignum! {
        function: <f64>::is_infinite(a: f64)
    }
    test_bignum! {
        function: <f64>::is_nan(a: f64)
    }
    test_bignum! {
        function: <f64>::is_subnormal(a: f64)
    }
    test_bignum! {
        function: <f64>::is_normal(a: f64)
    }
    test_bignum! {
        function: <f64>::classify(a: f64)
    }
    #[test]
    fn is_zero() {
        let z1 = F64::ZERO;
        let z2 = F64::NEG_ZERO;
        assert!(z1.is_zero());
        assert!(z2.is_zero());
        assert!(!F64::ONE.is_zero());
    }
}