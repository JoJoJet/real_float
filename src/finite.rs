use crate::{check::Checked, IntoInner, IsFinite};

/// The error produced when infinity or NaN is encountered.
#[derive(Debug, Clone, Copy)]
pub struct InfiniteError;
impl std::fmt::Display for InfiniteError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "encountered infinity or NaN unexpectedly")
    }
}

struct FiniteCheck;
impl<F: IsFinite> crate::check::Check<F> for FiniteCheck {
    type Error = InfiniteError;
    fn check(val: F) -> Result<F, InfiniteError> {
        if val.is_finite() {
            Ok(val)
        } else {
            Err(InfiniteError)
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(transparent)]
pub struct Finite<F: IsFinite>(Checked<F, FiniteCheck>);

impl<F: IsFinite> Finite<F> {
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
    /// If the result is non-finite.
    pub fn try_new(val: F) -> Result<Self, InfiniteError> {
        // This is not a TryFrom implementation due to [this issue](https://github.com/rust-lang/rust/issues/50133).
        Checked::try_new(val).map(Self)
    }
    /// Gets the inner value of this number.
    #[inline]
    pub fn val(self) -> F {
        self.0.val()
    }
}

impl<F: IsFinite> IntoInner<F> for Finite<F> {
    #[inline]
    fn into_inner(self) -> F {
        self.val()
    }
}

use crate::ToOrd;
use std::cmp::{Eq, PartialEq};
impl<F: IsFinite + ToOrd, Rhs: IntoInner<F> + Copy> PartialEq<Rhs> for Finite<F> {
    fn eq(&self, rhs: &Rhs) -> bool {
        self.0 == (*rhs).into_inner()
    }
}
impl<F: IsFinite + ToOrd> Eq for Finite<F> {}

use std::cmp::{Ord, Ordering, PartialOrd};
impl<F: IsFinite + ToOrd> PartialOrd<F> for Finite<F> {
    fn partial_cmp(&self, rhs: &F) -> Option<Ordering> {
        self.0.partial_cmp(rhs)
    }
}
impl<F: IsFinite + ToOrd> PartialOrd for Finite<F> {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&rhs.0)
    }
}
impl<F: IsFinite + ToOrd> Ord for Finite<F> {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.0.cmp(&rhs.0)
    }
}

use std::ops::{Add, Neg, Sub};
impl<F: IsFinite> Finite<F> {
    /// Attempts to add two numbers.
    /// # Errors
    /// If the result is non-finite.
    pub fn try_add(self, rhs: impl IntoInner<F>) -> Result<Self, InfiniteError>
    where
        F: Add<Output = F>,
    {
        self.0.try_add(rhs.into_inner()).map(Self)
    }
    /// Attempts to subtract two numbers.
    /// # Errors
    /// If the result is non-finite.
    pub fn try_sub(self, rhs: impl IntoInner<F>) -> Result<Self, InfiniteError>
    where
        F: Sub<Output = F>,
    {
        self.0.try_sub(rhs.into_inner()).map(Self)
    }
}
impl<F: Add<Output = F> + IsFinite, Rhs: IntoInner<F>> Add<Rhs> for Finite<F> {
    type Output = Self;
    #[track_caller]
    fn add(self, rhs: Rhs) -> Self::Output {
        Self(self.0 + rhs.into_inner())
    }
}
impl<F: Sub<Output = F> + IsFinite, Rhs: IntoInner<F>> Sub<Rhs> for Finite<F> {
    type Output = Self;
    #[track_caller]
    fn sub(self, rhs: Rhs) -> Self::Output {
        Self(self.0 - rhs.into_inner())
    }
}
impl<F: Neg<Output = F> + IsFinite> Neg for Finite<F> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

use std::ops::{Div, Mul, Rem};
impl<F: IsFinite> Finite<F> {
    /// Attempts to multiply two numbers.
    /// # Errors
    /// If the result is non-finite.
    pub fn try_mul(self, rhs: impl IntoInner<F>) -> Result<Self, InfiniteError>
    where
        F: Mul<Output = F>,
    {
        self.0.try_mul(rhs.into_inner()).map(Self)
    }
    /// Attempts to divide self by another number.
    /// # Errors
    /// If the result is non-finite.
    pub fn try_div(self, rhs: impl IntoInner<F>) -> Result<Self, InfiniteError>
    where
        F: Div<Output = F>,
    {
        self.0.try_div(rhs.into_inner()).map(Self)
    }
    /// Attempts to find the remainder of `self / rhs`.
    /// # Errors
    /// If the result is non-finite.
    pub fn try_rem(self, rhs: impl IntoInner<F>) -> Result<Self, InfiniteError>
    where
        F: Rem<Output = F>,
    {
        self.0.try_rem(rhs.into_inner()).map(Self)
    }
}
impl<F: Mul<Output = F> + IsFinite, Rhs: IntoInner<F>> Mul<Rhs> for Finite<F> {
    type Output = Self;
    #[track_caller]
    fn mul(self, rhs: Rhs) -> Self::Output {
        Self(self.0 * rhs.into_inner())
    }
}
impl<F: Div<Output = F> + IsFinite, Rhs: IntoInner<F>> Div<Rhs> for Finite<F> {
    type Output = Self;
    #[track_caller]
    fn div(self, rhs: Rhs) -> Self::Output {
        Self(self.0 / rhs.into_inner())
    }
}
impl<F: Rem<Output = F> + IsFinite, Rhs: IntoInner<F>> Rem<Rhs> for Finite<F> {
    type Output = Self;
    #[track_caller]
    fn rem(self, rhs: Rhs) -> Self::Output {
        Self(self.0 % rhs.into_inner())
    }
}

use crate::Pow;
impl<F: IsFinite + Pow> Finite<F> {
    /// Attempts to raise `self` to the power `n`.
    /// # Errors
    /// If the result is non-finite.
    pub fn try_powf(self, n: impl IntoInner<F>) -> Result<Self, InfiniteError> {
        self.0.try_powf(n.into_inner()).map(Self)
    }
    /// Attempts to raise `self` to the power `n`.
    /// # Errors
    /// If the result is non-finite.
    pub fn try_powi(self, n: i32) -> Result<Self, InfiniteError> {
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
impl<F: IsFinite + Root> Finite<F> {
    /// Attempts to find the square root of a number.
    /// # Errors
    /// If the result is non-finite.
    pub fn try_sqrt(self) -> Result<Self, InfiniteError> {
        self.0.try_sqrt().map(Self)
    }
    /// Attempts to find the cube root of a number.
    /// # Errors
    /// If the result is non-finite.
    pub fn try_cbrt(self) -> Result<Self, InfiniteError> {
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
impl<F: IsFinite + Log> Finite<F> {
    /// Attempts to find the log base `b` of self.
    /// # Errors
    /// If the result is non-finite.
    pub fn try_log(self, b: impl IntoInner<F>) -> Result<Self, InfiniteError> {
        self.0.try_log(b.into_inner()).map(Self)
    }
    /// Attempts to find the natural log (base e) of self.
    /// # Errors
    /// If the result is non-finite.
    pub fn try_ln(self) -> Result<Self, InfiniteError> {
        self.0.try_ln().map(Self)
    }
    /// Attempts to find the log base 2 of self.
    /// # Errors
    /// If the result is non-finite.
    pub fn try_log2(self) -> Result<Self, InfiniteError> {
        self.0.try_log2().map(Self)
    }
    /// Attempts to find the log base 10 of self.
    /// # Errors
    /// If the result is non-finite.
    pub fn try_log10(self) -> Result<Self, InfiniteError> {
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
            assert!(($e).is_err(), "{:?}", $e)
        };
    }

    macro_rules! finite {
        ($f: expr) => {
            Finite::new($f)
        };
    }

    #[test]
    #[should_panic]
    fn assert_new_nan() {
        finite!(f32::NAN);
    }
    #[test]
    #[should_panic]
    fn assert_new_nan2() {
        finite!(-f32::NAN);
    }
    #[test]
    #[should_panic]
    fn assert_new_inf() {
        finite!(f32::INFINITY);
    }
    #[test]
    #[should_panic]
    fn assert_new_inf2() {
        finite!(f32::NEG_INFINITY);
    }

    #[test]
    fn assert_nan() {
        assert_err!(finite!(f32::MAX).try_add(f32::MAX));
        assert_err!(finite!(f32::MIN).try_sub(f32::MAX));
        assert_err!(finite!(f32::MAX).try_mul(f32::MAX));
        assert_err!(finite!(1.0f32).try_div(0.0));
        assert_err!(finite!(1.0f32).try_div(-0.0));
        // TODO: figure out how to get infinity out of `Rem`, if possible
        assert_err!(finite!(1.0f32).try_rem(0.0));

        assert_err!(finite!(1000.0f32).try_powf(1000.0)); // overflows to infinity

        assert_err!(finite!(-1.0f32).try_sqrt());

        assert_err!(finite!(-1.0f32).try_log(3.0));
        assert_err!(finite!(-1.0f32).try_ln());
        assert_err!(finite!(-1.0f32).try_log2());
        assert_err!(finite!(-1.0f32).try_log10());
    }

    #[test]
    fn assert_ops() {
        assert_eq!(finite!(2.0f32) + 1.0, finite!(3.0));
        assert_eq!(finite!(2.0f32) - 1.0, finite!(1.0));
        assert_eq!(finite!(5.0f32) * 2.0, finite!(10.0));
        assert_eq!(finite!(8.0f32) / 2.0, finite!(4.0));
        assert_eq!(-finite!(1.0f32), finite!(-1.0));

        assert_eq!(finite!(4.0f32).powf(3.5), finite!(128.0));
        assert_eq!(finite!(2.0f32).powi(8), finite!(256.0));
        assert_eq!(finite!(4.0f32).sqrt(), finite!(2.0));
        assert_eq!(finite!(27.0f32).cbrt(), finite!(3.0));
        assert_eq!(finite!(16.0f32).log(4.0), finite!(2.0));
        assert_eq!(finite!(1.0f32).ln(), finite!(0.0));
        assert_eq!(finite!(8.0f32).log2(), finite!(3.0));
        assert_eq!(finite!(1000.0f32).log10(), finite!(3.0));
    }

    #[test]
    #[allow(clippy::bool_assert_comparison)]
    #[allow(clippy::cmp_nan)]
    fn assert_cmp_weird() {
        assert!(finite!(-1.0f32) < finite!(0.0));

        assert_eq!(finite!(0.0f32), finite!(0.0));
        assert_eq!(finite!(0.0f32), finite!(-0.0));
        assert_eq!(finite!(-0.0f32), finite!(0.0));
        assert_eq!(finite!(-0.0f32), finite!(-0.0));

        assert!(finite!(0.0) < finite!(1.0));

        assert_eq!(finite!(1.0) < f32::NAN, false);
        assert_eq!(finite!(1.0) >= f32::NAN, false);
    }
}
