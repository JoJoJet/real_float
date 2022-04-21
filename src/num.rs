use std::fmt;

use num_traits::{Float, Num, NumCast, One, ToPrimitive, Zero};

use crate::{
    ops::{Exp, Pow, Round, Signed, Trig},
    Finite, InfiniteError, IsFinite, IsNan, NanError, Real, ToOrd,
};

#[derive(Debug)]
pub enum FromStrError<P, C> {
    Parse(P),
    Check(C),
}
impl<P: fmt::Display, C: fmt::Display> fmt::Display for FromStrError<P, C> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Parse(e) => write!(f, "error parsing float: {e}"),
            Self::Check(e) => write!(f, "error checking float: {e}"),
        }
    }
}

macro_rules! impl_float {
    ($ty: ident, $($(+)? $bound: path)*, $error: ty) => {
        impl<F: Float $(+ $bound)*> One for $ty<F> {
            fn one() -> Self {
                Self::new(F::one())
            }
            fn is_one(&self) -> bool {
                self.val().is_one()
            }
        }
        impl<F: Float $(+ $bound)*> Zero for $ty<F> {
            fn zero() -> Self {
                Self::new(F::zero())
            }
            fn is_zero(&self) -> bool {
                self.val().is_zero()
            }
        }

        impl<F: Float $(+ $bound)*> Num for $ty<F> {
            type FromStrRadixErr = FromStrError<F::FromStrRadixErr, $error>;

            fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
                let val = F::from_str_radix(str, radix).map_err(FromStrError::Parse)?;
                let val = Self::try_new(val).map_err(FromStrError::Check)?;
                Ok(val)
            }
        }

        impl<F: Float $(+ $bound)*> ToPrimitive for $ty<F> {
            fn to_i64(&self) -> Option<i64> {
                self.val().to_i64()
            }

            fn to_u64(&self) -> Option<u64> {
                self.val().to_u64()
            }
        }
        impl<F: Float $(+ $bound)*> NumCast for $ty<F> {
            fn from<T: num_traits::ToPrimitive>(n: T) -> Option<Self> {
                <F as NumCast>::from(n).and_then(|n| Self::try_new(n).ok())
            }
        }

        impl<F: Float $(+ $bound)* + Round + Signed + Pow + Exp + Trig> Float for $ty<F> {
            fn nan() -> Self {
                unimplemented!()
            }
            fn infinity() -> Self {
                Self::new(F::infinity()) // TODO: add inherent method
            }
            fn neg_infinity() -> Self {
                Self::new(F::neg_infinity()) // TODO: add inherent method
            }
            fn neg_zero() -> Self {
                Self::new(F::neg_zero()) // TODO: add inherent method
            }
            fn min_value() -> Self {
                Self::new(F::min_value()) // TODO: add inherent method
            }
            fn min_positive_value() -> Self {
                Self::new(F::min_positive_value()) // TODO: add inherent method
            }
            fn max_value() -> Self {
                Self::new(F::max_value()) // TODO: add inherent method
            }

            fn is_nan(self) -> bool {
                if crate::check::STRICT {
                    false
                } else {
                    <F as Float>::is_nan(self.val())
                }
            }
            fn is_infinite(self) -> bool {
                <F as Float>::is_infinite(self.val()) // TODO: add inherent method
            }
            fn is_finite(self) -> bool {
                <F as Float>::is_finite(self.val()) // TODO: add inherent method
            }
            fn is_normal(self) -> bool {
                self.val().is_normal() // TODO: add inherent method
            }

            fn classify(self) -> std::num::FpCategory {
                self.val().classify() // TODO: add inherent method
            }

            fn floor(self) -> Self {
                self.floor()
            }
            fn ceil(self) -> Self {
                self.ceil()
            }
            fn round(self) -> Self {
                self.round()
            }
            fn trunc(self) -> Self {
                self.trunc()
            }
            fn fract(self) -> Self {
                self.fract()
            }

            fn abs(self) -> Self {
                self.abs()
            }
            fn abs_sub(self, other: Self) -> Self {
                // we're not giving this an inherent method bc its bad
                Self::new(self.val().abs_sub(other.val()))
            }
            fn signum(self) -> Self {
                self.signum()
            }
            fn is_sign_positive(self) -> bool {
                self.is_sign_positive()
            }
            fn is_sign_negative(self) -> bool {
                self.is_sign_negative()
            }

            fn mul_add(self, a: Self, b: Self) -> Self {
                Self::new(self.val().mul_add(a.val(), b.val())) // TODO: add inherent method
            }
            fn recip(self) -> Self {
                self.recip()
            }
            fn powi(self, n: i32) -> Self {
                self.powi(n)
            }
            fn powf(self, n: Self) -> Self {
                self.powf(n)
            }
            fn sqrt(self) -> Self {
                self.sqrt()
            }
            fn cbrt(self) -> Self {
                self.cbrt()
            }
            fn hypot(self, other: Self) -> Self {
                self.hypot(other)
            }

            fn exp(self) -> Self {
                self.exp()
            }
            fn exp2(self) -> Self {
                self.exp2()
            }
            fn exp_m1(self) -> Self {
                self.exp_m1()
            }
            fn ln(self) -> Self {
                self.ln()
            }
            fn log(self, base: Self) -> Self {
                self.log(base)
            }
            fn log2(self) -> Self {
                self.log2()
            }
            fn log10(self) -> Self {
                self.log10()
            }
            fn ln_1p(self) -> Self {
                self.ln_1p()
            }

            fn max(self, other: Self) -> Self {
                self.max(other)
            }
            fn min(self, other: Self) -> Self {
                self.min(other)
            }

            fn sin(self) -> Self {
                self.sin()
            }
            fn cos(self) -> Self {
                self.cos()
            }
            fn sin_cos(self) -> (Self, Self) {
                self.sin_cos()
            }
            fn tan(self) -> Self {
                self.tan()
            }
            fn asin(self) -> Self {
                self.asin()
            }
            fn acos(self) -> Self {
                self.acos()
            }
            fn atan(self) -> Self {
                self.atan()
            }
            fn atan2(self, other: Self) -> Self {
                self.atan2(other)
            }

            fn sinh(self) -> Self {
                Self::new(self.val().sinh()) // TODO: add inherent method
            }
            fn cosh(self) -> Self {
                Self::new(self.val().cosh()) // TODO: add inherent method
            }
            fn tanh(self) -> Self {
                Self::new(self.val().tanh()) // TODO: add inherent method
            }
            fn asinh(self) -> Self {
                Self::new(self.val().asinh()) // TODO: add inherent method
            }
            fn acosh(self) -> Self {
                Self::new(self.val().acosh()) // TODO: add inherent method
            }
            fn atanh(self) -> Self {
                Self::new(self.val().atanh()) // TODO: add inherent method
            }

            fn integer_decode(self) -> (u64, i16, i8) {
                self.val().integer_decode()
            }
        }
    };
}

impl_float!(Real, IsNan + ToOrd, NanError);
impl_float!(Finite, IsFinite + ToOrd, InfiniteError);
