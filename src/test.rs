macro_rules! test_bignum {
	{
		function: <$primitive: ty $(as $Trait: ident $(<$($gen: ty), *>)?)?> :: $function: ident ($($param: ident : $(ref $re: tt)? $ty: ty), *)
		$(, skip: $skip: expr)?
	} => {
		paste::paste! {
			quickcheck::quickcheck! {
				#[allow(non_snake_case)]
				fn [<quickcheck_ $primitive _ $($Trait _ $($($gen _) *)?)? $function>]($($param : $ty), *) -> quickcheck::TestResult {
					$(if $skip {
						return quickcheck::TestResult::discard();
					})?
	
					let (big, primitive) = crate::test::results!(<$primitive $(as $Trait $(<$($gen), *>)?)?>::$function ($($($re)? Into::into($param)), *));
	
					quickcheck::TestResult::from_bool(big == primitive)
				}
			}
		}
	};
	{
		function: <$primitive: ty $(as $Trait: ty)?> :: $function: ident,
		cases: [
            $(($($arg: expr), *)), *
        ]
	} => {
		paste::paste! {
			#[test]
			fn [<cases_ $primitive _ $function>]() {
				$(
					let (big, primitive) = crate::test::results!(<$primitive> :: $function ($(Into::into($arg)), *));
					assert_eq!(big, primitive);
				)*
			}
		}
	};
	{
		function: <$primitive: ty $(as $Trait: ty)?> :: $function: ident ($($param: ident : $(ref $re: tt)? $ty: ty), *)
		$(, skip: $skip: expr)?
		, cases: [
            $(($($arg: expr), *)), *
        ]
	} => {
		crate::test::test_bignum! {
			function: <$primitive $(as $Trait)?> :: $function,
			cases: [
				$(($($arg), *)), *
			]
		}
		crate::test::test_bignum! {
			function: <$primitive $(as $Trait)?> :: $function ($($param : $(ref $re)? $ty), *)
			$(, skip: $skip)?
		}
	};
}

pub(crate) use test_bignum;

macro_rules! results {
	(<$primitive: ty $(as $Trait: ty)?> :: $function: ident ($($arg: expr), *)) => {
		paste::paste! {
			{
				let big_result = <crate::[<$primitive:upper>] $(as $Trait)?>::$function(
					$($arg), *
				);
				let prim_result = <$primitive $(as $Trait)?>::$function(
					$($arg), *
				);

				use crate::test::TestConvert;
				(TestConvert::into(big_result), TestConvert::into(prim_result))
			}
		}
	};
}

pub(crate) use results;

macro_rules! test_from {
    {
        function: <$primitive: ty as $Trait: ident>:: $name: ident,
        from_types: ($($from_type: ty), *)
    } => {
		$(
			crate::test::test_bignum! {
				function: < $primitive as $Trait<$from_type> >::$name(from: $from_type)
			}
		)*
    }
}

pub(crate) use test_from;

macro_rules! test_into {
    {
        function: <$primitive: ty as $Trait: ident>:: $name: ident,
        into_types: ($($into_type: ty), *)
    } => {
		paste::paste! {
			$(
				crate::test::test_bignum! {
					function: < $primitive as $Trait<$into_type> >::$name(from: $primitive)
				}
			)*
		}
    }
}

pub(crate) use test_into;

macro_rules! quickcheck_from_to_radix {
    ($primitive: ty, $name: ident, $max: expr) => {
        paste::paste! {
            quickcheck::quickcheck! {
                fn [<quickcheck_from_to_ $name>](u: $primitive, radix: u8) -> quickcheck::TestResult {
                    #[allow(unused_comparisons)]
                    if !((2..=$max).contains(&radix)) {
                        return quickcheck::TestResult::discard();
                    }
                    let u = <crate::[<$primitive:upper>]>::from(u);
                    let v = u.[<to_ $name>](radix as u32);
                    let u1 = <crate::[<$primitive:upper>]>::[<from_ $name>](&v, radix as u32).unwrap();
                    quickcheck::TestResult::from_bool(u == u1)
                }
            }
        }
    }
}

pub(crate) use quickcheck_from_to_radix;

macro_rules! test_fmt {
    {
        int: $int: ty,
        name: $name: ident,
        format: $format: expr,
        numbers: {
            $($number: expr), *
        }
    } => {
        paste::paste! {
            #[test]
            fn [<$name _format>]() {
                $(
                    let big = <$int>::from($number);
                    assert_eq!(format!(concat!("{:", $format, "}"), big), format!(concat!("{:", $format, "}"), $number));
                    assert_eq!(format!(concat!("{:#", $format, "}"), big), format!(concat!("{:#", $format, "}"), $number));
                )*
            }
            
            quickcheck::quickcheck! {
                fn [<quickcheck_ $name _format>](i: [<$int:lower>]) -> bool {
                    let big = <$int>::from(i);
                    format!(concat!("{:#", $format, "}"), big) == format!(concat!("{:#", $format, "}"), i)
                }
            }
        }
    }
}

pub(crate) use test_fmt;

macro_rules! debug_skip {
	($skip: expr) => {
		{
			#[cfg(debug_assertions)]
			let skip = $skip;
			#[cfg(not(debug_assertions))]
			let skip = false;

			skip
		}
	};
}

pub(crate) use debug_skip;

#[derive(Clone, Copy)]
pub struct U8ArrayWrapper<const N: usize>([u8; N]);

impl<const N: usize> From<U8ArrayWrapper<N>> for [u8; N] {
    fn from(a: U8ArrayWrapper<N>) -> Self {
        a.0
    }
}

use quickcheck::{Arbitrary, Gen};

impl Arbitrary for U8ArrayWrapper<16> {
    fn arbitrary(g: &mut Gen) -> Self {
        Self(u128::arbitrary(g).to_be_bytes())
    }
}

impl Arbitrary for U8ArrayWrapper<8> {
    fn arbitrary(g: &mut Gen) -> Self {
        Self(u64::arbitrary(g).to_be_bytes())
    }
}

use core::fmt::{Formatter, self, Debug};

impl<const N: usize> Debug for U8ArrayWrapper<N> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

use crate::types::{U128, I128, U64, I64/*, F64*/};

pub trait TestConvert {
    type Output;

    fn into(self) -> Self::Output;
}

impl TestConvert for u128 {
    type Output = u128;

    #[inline]
    fn into(self) -> Self::Output {
        self.to_le()
    }
}

impl TestConvert for U128 {
    type Output = u128;

    #[inline]
    fn into(self) -> Self::Output {
        unsafe {
            core::mem::transmute(self)
        }
    }
}

impl TestConvert for u64 {
    type Output = u64;

    #[inline]
    fn into(self) -> Self::Output {
        self.to_le()
    }
}

impl TestConvert for U64 {
    type Output = u64;

    #[inline]
    fn into(self) -> Self::Output {
        unsafe {
            core::mem::transmute(self)
        }
    }
}

impl TestConvert for I64 {
    type Output = i64;

    #[inline]
    fn into(self) -> Self::Output {
        unsafe {
            core::mem::transmute(self)
        }
    }
}

impl TestConvert for i128 {
    type Output = i128;

    #[inline]
    fn into(self) -> Self::Output {
        self.to_le()
    }
}

impl TestConvert for I128 {
    type Output = i128;

    #[inline]
    fn into(self) -> Self::Output {
        unsafe {
            core::mem::transmute(self)
        }
    }
}

impl<T: TestConvert> TestConvert for Option<T> {
    type Output = Option<<T as TestConvert>::Output>;

    #[inline]
    fn into(self) -> Self::Output {
        self.map(TestConvert::into)
    }
}

impl TestConvert for f64 {
    type Output = u64;

    #[inline]
    fn into(self) -> Self::Output {
        self.to_bits().to_le()
    }
}

impl TestConvert for f32 {
    type Output = u32;

    #[inline]
    fn into(self) -> Self::Output {
        self.to_bits().to_le()
    }
}

/*impl TestConvert for F64 {
    type Output = u64;

    #[inline]
    fn into(self) -> Self::Output {
        unsafe {
            core::mem::transmute(self.to_bits())
        }
    }
}*/

impl<T: TestConvert, U: TestConvert> TestConvert for (T, U) {
    type Output = (<T as TestConvert>::Output, <U as TestConvert>::Output);

    #[inline]
    fn into(self) -> Self::Output {
        (TestConvert::into(self.0), TestConvert::into(self.1))
    }
}

impl<T, const N: usize> TestConvert for [T; N] {
    type Output = Self;
    
    #[inline]
    fn into(self) -> Self::Output {
        self
    }
}

impl TestConvert for u32 {
    type Output = u32;
    
    fn into(self) -> Self::Output {
        self
    }
}

impl TestConvert for crate::ParseIntError {
    type Output = core::num::IntErrorKind;

    #[inline]
    fn into(self) -> Self::Output {
        self.kind().clone()
    }
}

impl TestConvert for core::num::ParseIntError {
    type Output = core::num::IntErrorKind;

    #[inline]
    fn into(self) -> Self::Output {
        self.kind().clone()
    }
}

impl<T: TestConvert, E: TestConvert> TestConvert for Result<T, E> {
    type Output = Result<<T as TestConvert>::Output, <E as TestConvert>::Output>;

    #[inline]
    fn into(self) -> Self::Output {
        match self {
            Ok(val) => Ok(TestConvert::into(val)),
            Err(err) => Err(TestConvert::into(err)),
        }
    }
}

impl TestConvert for core::num::TryFromIntError {
	type Output = ();

	#[inline]
	fn into(self) -> Self::Output {
		()
	}
}

impl TestConvert for crate::error::TryFromIntError {
	type Output = ();

	#[inline]
	fn into(self) -> Self::Output {
		()
	}
}

impl TestConvert for core::convert::Infallible {
	type Output = ();

	#[inline]
	fn into(self) -> Self::Output {
		()
	}
}

macro_rules! test_convert_to_self {
    ($($ty: ty), *) => {
        $(
            impl TestConvert for $ty {
                type Output = Self;
                
                #[inline]
                fn into(self) -> Self::Output {
                    self
                }
            }
        )*
    };
}

test_convert_to_self!(core::num::FpCategory, bool, core::cmp::Ordering, u8, u16, usize, i8, i16, i32, i64, isize);