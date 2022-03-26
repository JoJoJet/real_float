use crate::{check::Checked, IntoInner, IsNan};

/// The error produced when NaN is encountered.
#[derive(Debug, Clone, Copy)]
pub struct NanError;
impl std::fmt::Display for NanError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "encountered NaN unexpectedly")
    }
}

struct NanCheck;
impl<F: IsNan> crate::check::Check<F> for NanCheck {
    type Error = NanError;
    fn check(val: F) -> Result<F, NanError> {
        if val.is_nan() {
            Err(NanError)
        } else {
            Ok(val)
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(transparent)]
pub struct Real<F: IsNan>(Checked<F, NanCheck>);

impl<F: IsNan> Real<F> {
    /// Creates a new [`Real`] float, panicking if it's NaN.  
    ///
    /// Note that this function will *not* panic in release mode,
    /// unless the `strict` feature flag is set.
    #[track_caller]
    pub fn new(val: F) -> Self {
        Self(Checked::new(val))
    }
    /// Attempts to create a new [`Real`] float.
    /// # Errors
    /// If the number is NaN.
    pub fn try_new(val: F) -> Result<Self, NanError> {
        // This is not a TryFrom implementation due to [this issue](https://github.com/rust-lang/rust/issues/50133).
        Checked::try_new(val).map(Self)
    }
    /// Gets the inner value of this number.
    #[inline]
    pub fn val(self) -> F {
        self.0.val()
    }
}

impl<F: IsNan> IntoInner<F> for Real<F> {
    #[inline]
    fn into_inner(self) -> F {
        self.val()
    }
}

use crate::ToOrd;
use std::cmp::{Eq, PartialEq};
impl<F: IsNan + ToOrd, Rhs: IntoInner<F> + Copy> PartialEq<Rhs> for Real<F> {
    fn eq(&self, rhs: &Rhs) -> bool {
        self.0 == (*rhs).into_inner()
    }
}
impl<F: IsNan + ToOrd> Eq for Real<F> {}

use std::cmp::{Ord, Ordering, PartialOrd};
impl<F: IsNan + ToOrd> PartialOrd<F> for Real<F> {
    fn partial_cmp(&self, rhs: &F) -> Option<Ordering> {
        self.0.partial_cmp(rhs)
    }
}
impl<F: IsNan + ToOrd> PartialOrd for Real<F> {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&rhs.0)
    }
}
impl<F: IsNan + ToOrd> Ord for Real<F> {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.0.cmp(&rhs.0)
    }
}

use std::ops::{Add, Neg, Sub};
impl<F: IsNan> Real<F> {
    /// Attempts to add two numbers.
    /// # Errors
    /// If the result is NaN.
    pub fn try_add(self, rhs: impl IntoInner<F>) -> Result<Self, NanError>
    where
        F: Add<Output = F>,
    {
        self.0.try_add(rhs.into_inner()).map(Self)
    }
    /// Attempts to subtract two numbers.
    /// # Errors
    /// If the result is NaN.
    pub fn try_sub(self, rhs: impl IntoInner<F>) -> Result<Self, NanError>
    where
        F: Sub<Output = F>,
    {
        self.0.try_sub(rhs.into_inner()).map(Self)
    }
}
impl<F: Add<Output = F> + IsNan, Rhs: IntoInner<F>> Add<Rhs> for Real<F> {
    type Output = Self;
    #[track_caller]
    fn add(self, rhs: Rhs) -> Self::Output {
        Self(self.0 + rhs.into_inner())
    }
}
impl<F: Sub<Output = F> + IsNan, Rhs: IntoInner<F>> Sub<Rhs> for Real<F> {
    type Output = Self;
    #[track_caller]
    fn sub(self, rhs: Rhs) -> Self::Output {
        Self(self.0 - rhs.into_inner())
    }
}
impl<F: Neg<Output = F> + IsNan> Neg for Real<F> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

use std::ops::{Div, Mul, Rem};
impl<F: IsNan> Real<F> {
    /// Attempts to multiply two numbers.
    /// # Errors
    /// If the result is NaN.
    pub fn try_mul(self, rhs: impl IntoInner<F>) -> Result<Self, NanError>
    where
        F: Mul<Output = F>,
    {
        self.0.try_mul(rhs.into_inner()).map(Self)
    }
    /// Attempts to divide self by another number.
    /// # Errors
    /// If the result is NaN.
    pub fn try_div(self, rhs: impl IntoInner<F>) -> Result<Self, NanError>
    where
        F: Div<Output = F>,
    {
        self.0.try_div(rhs.into_inner()).map(Self)
    }
    /// Attempts to find the remainder of `self / rhs`.
    /// # Errors
    /// If the result is NaN.
    pub fn try_rem(self, rhs: impl IntoInner<F>) -> Result<Self, NanError>
    where
        F: Rem<Output = F>,
    {
        self.0.try_rem(rhs.into_inner()).map(Self)
    }
}
impl<F: Mul<Output = F> + IsNan, Rhs: IntoInner<F>> Mul<Rhs> for Real<F> {
    type Output = Self;
    #[track_caller]
    fn mul(self, rhs: Rhs) -> Self::Output {
        Self(self.0 * rhs.into_inner())
    }
}
impl<F: Div<Output = F> + IsNan, Rhs: IntoInner<F>> Div<Rhs> for Real<F> {
    type Output = Self;
    #[track_caller]
    fn div(self, rhs: Rhs) -> Self::Output {
        Self(self.0 / rhs.into_inner())
    }
}
impl<F: Rem<Output = F> + IsNan, Rhs: IntoInner<F>> Rem<Rhs> for Real<F> {
    type Output = Self;
    #[track_caller]
    fn rem(self, rhs: Rhs) -> Self::Output {
        Self(self.0 % rhs.into_inner())
    }
}

use crate::Pow;
impl<F: IsNan + Pow> Real<F> {
    /// Attempts to raise `self` to the power `n`.
    /// # Errors
    /// If the result is NaN.
    pub fn try_powf(self, n: impl IntoInner<F>) -> Result<Self, NanError> {
        self.0.try_powf(n.into_inner()).map(Self)
    }
    /// Attempts to raise `self` to the power `n`.
    /// # Errors
    /// If the result is NaN.
    pub fn try_powi(self, n: i32) -> Result<Self, NanError> {
        self.0.try_powi(n).map(Self)
    }
    #[track_caller]
    #[must_use]
    pub fn powf(self, n: impl IntoInner<F>) -> Self {
        Self(self.0.powf(n.into_inner()))
    }
    #[track_caller]
    #[must_use]
    pub fn powi(self, n: i32) -> Self {
        Self(self.0.powi(n))
    }
}

use crate::Root;
impl<F: IsNan + Root> Real<F> {
    /// Attempts to find the square root of a number.
    /// # Errors
    /// If the result is NaN.
    pub fn try_sqrt(self) -> Result<Self, NanError> {
        self.0.try_sqrt().map(Self)
    }
    /// Attempts to find the cube root of a number.
    /// # Errors
    /// If the result is NaN.
    pub fn try_cbrt(self) -> Result<Self, NanError> {
        self.0.try_cbrt().map(Self)
    }
    #[track_caller]
    #[must_use]
    pub fn sqrt(self) -> Self {
        Self(self.0.sqrt())
    }
    #[track_caller]
    #[must_use]
    pub fn cbrt(self) -> Self {
        Self(self.0.cbrt())
    }
}

use crate::Log;
impl<F: IsNan + Log> Real<F> {
    /// Attempts to find the log base `b` of self.
    /// # Errors
    /// If the result is NaN.
    pub fn try_log(self, b: impl IntoInner<F>) -> Result<Self, NanError> {
        self.0.try_log(b.into_inner()).map(Self)
    }
    /// Attempts to find the natural log (base e) of self.
    /// # Errors
    /// If the result is NaN.
    pub fn try_ln(self) -> Result<Self, NanError> {
        self.0.try_ln().map(Self)
    }
    /// Attempts to find the log base 2 of self.
    /// # Errors
    /// If the result is NaN.
    pub fn try_log2(self) -> Result<Self, NanError> {
        self.0.try_log2().map(Self)
    }
    /// Attempts to find the log base 10 of self.
    /// # Errors
    /// If the result is NaN.
    pub fn try_log10(self) -> Result<Self, NanError> {
        self.0.try_log10().map(Self)
    }

    #[track_caller]
    #[must_use]
    pub fn log(self, base: impl IntoInner<F>) -> Self {
        Self(self.0.log(base.into_inner()))
    }
    #[track_caller]
    #[must_use]
    pub fn ln(self) -> Self {
        Self(self.0.ln())
    }
    #[track_caller]
    #[must_use]
    pub fn log2(self) -> Self {
        Self(self.0.log2())
    }
    #[track_caller]
    #[must_use]
    pub fn log10(self) -> Self {
        Self(self.0.log10())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_err {
        ($e: expr) => {
            assert!(($e).is_err())
        };
    }

    macro_rules! real {
        ($f: expr) => {
            Real::new($f)
        };
    }

    #[test]
    #[should_panic]
    fn assert_new_nan() {
        real!(f32::NAN);
    }
    #[test]
    #[should_panic]
    fn assert_new_nan2() {
        real!(-f32::NAN);
    }

    #[test]
    fn assert_nan() {
        assert_err!(real!(f32::INFINITY).try_add(f32::NEG_INFINITY));
        assert_err!(real!(f32::INFINITY).try_sub(f32::INFINITY));
        assert_err!(real!(0.0f32).try_mul(f32::INFINITY));
        assert_err!(real!(0.0f32).try_div(0.0));
        assert_err!(real!(f32::INFINITY).try_rem(1.0));
        assert_err!(real!(1.0f32).try_rem(0.0));

        assert_err!(real!(-1.0f32).try_sqrt());

        assert_err!(real!(-1.0f32).try_log(3.0));
        assert_err!(real!(-1.0f32).try_ln());
        assert_err!(real!(-1.0f32).try_log2());
        assert_err!(real!(-1.0f32).try_log10());
    }

    #[test]
    fn assert_ops() {
        assert_eq!(real!(2.0f32) + 1.0, real!(3.0));
        assert_eq!(real!(2.0f32) - 1.0, real!(1.0));
        assert_eq!(real!(5.0f32) * 2.0, real!(10.0));
        assert_eq!(real!(8.0f32) / 2.0, real!(4.0));
        assert_eq!(-real!(1.0f32), real!(-1.0));

        assert_eq!(real!(1000.0f32).powf(1000.0), real!(f32::INFINITY));
        assert_eq!(real!(4.0f32).powf(3.5), real!(128.0));
        assert_eq!(real!(2.0f32).powi(8), real!(256.0));
        assert_eq!(real!(4.0f32).sqrt(), real!(2.0));
        assert_eq!(real!(27.0f32).cbrt(), real!(3.0));
        assert_eq!(real!(16.0f32).log(4.0), real!(2.0));
        assert_eq!(real!(1.0f32).ln(), real!(0.0));
        assert_eq!(real!(8.0f32).log2(), real!(3.0));
        assert_eq!(real!(1000.0f32).log10(), real!(3.0));
    }

    #[test]
    #[allow(clippy::bool_assert_comparison)]
    #[allow(clippy::cmp_nan)]
    fn assert_cmp_weird() {
        assert!(real!(f32::NEG_INFINITY) < real!(-1.0));
        assert!(real!(-1.0f32) < real!(0.0));

        assert_eq!(real!(0.0f32), real!(0.0));
        assert_eq!(real!(0.0f32), real!(-0.0));
        assert_eq!(real!(-0.0f32), real!(0.0));
        assert_eq!(real!(-0.0f32), real!(-0.0));

        assert!(real!(0.0) < real!(1.0));
        assert!(real!(1.0) < real!(f32::INFINITY));

        assert_eq!(real!(1.0) < f32::NAN, false);
        assert_eq!(real!(1.0) >= f32::NAN, false);
    }
}
