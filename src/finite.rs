use crate::{check::Checked, IntoInner};

/// The error produced when infinity or NaN is encountered.
#[derive(Debug, Clone, Copy)]
pub struct InfiniteError;
impl std::fmt::Display for InfiniteError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "encountered infinity or NaN unexpectedly")
    }
}

#[allow(clippy::module_name_repetitions)]
pub trait IsFinite: Sized + Copy {
    fn is_finite(self) -> bool;
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

/// Constructor for [`Finite`] that never checks the value, and can be used in a const context.
/// # Safety
/// Ensure that the value can never be `NaN` or infinite.
#[macro_export]
macro_rules! finite_unchecked {
    ($f: expr) => {{
        union Transmute<F: $crate::IsFinite> {
            inner: F,
            finite: $crate::Finite<F>,
        }

        // SAFETY: `Finite` is `repr(transparent)`.
        let val = Transmute { inner: $f };
        val.finite
    }};
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

    #[cfg_attr(track_caller, debug_assertions)]
    fn new_unchecked(val: F) -> Self {
        Self(Checked::new_unchecked(val))
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

use crate::ops::Round;
impl<F: IsFinite + Round> Finite<F> {
    #[must_use]
    pub fn floor(self) -> Self {
        Self(self.0.floor())
    }
    #[must_use]
    pub fn ceil(self) -> Self {
        Self(self.0.ceil())
    }
    #[must_use]
    pub fn round(self) -> Self {
        Self(self.0.round())
    }
    #[must_use]
    pub fn trunc(self) -> Self {
        Self(self.0.trunc())
    }
    #[must_use]
    pub fn fract(self) -> Self {
        Self(self.0.fract())
    }
}

use crate::ops::Signed;
impl<F: IsFinite + Signed> Finite<F> {
    /// Computes the absolute value of self.
    #[must_use]
    pub fn abs(self) -> Self {
        Self(self.0.abs())
    }
    /// Returns a number that represents the sign of self.
    /// * `1.0` if the number is positive or `+0.0`.
    /// * `-1.0` if the number is negative or `-0.0`.
    #[must_use]
    pub fn signum(self) -> Self {
        Self(self.0.signum())
    }
    /// Returns true if self has a negative sign, including -0.0 and negative infinity.
    #[must_use]
    pub fn is_sign_negative(self) -> bool {
        self.0.is_sign_negative()
    }
    /// Returns true if self has a positive sign, including +0.0 and positive infinity.
    #[must_use]
    pub fn is_sign_positive(self) -> bool {
        self.0.is_sign_positive()
    }
}

impl<F: IsFinite + ToOrd> Finite<F> {
    #[must_use]
    pub fn max(self, other: impl IntoInner<F>) -> Self {
        Self(self.0.max(other.into_inner()))
    }
    #[must_use]
    pub fn min(self, other: impl IntoInner<F>) -> Self {
        Self(self.0.min(other.into_inner()))
    }
}

use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};
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
impl<F: AddAssign + IsFinite, Rhs: IntoInner<F>> AddAssign<Rhs> for Finite<F> {
    #[track_caller]
    fn add_assign(&mut self, rhs: Rhs) {
        self.0 += rhs.into_inner();
    }
}
impl<F: Sub<Output = F> + IsFinite, Rhs: IntoInner<F>> Sub<Rhs> for Finite<F> {
    type Output = Self;
    #[track_caller]
    fn sub(self, rhs: Rhs) -> Self::Output {
        Self(self.0 - rhs.into_inner())
    }
}
impl<F: SubAssign + IsFinite, Rhs: IntoInner<F>> SubAssign<Rhs> for Finite<F> {
    #[track_caller]
    fn sub_assign(&mut self, rhs: Rhs) {
        self.0 -= rhs.into_inner();
    }
}
impl<F: Neg<Output = F> + IsFinite> Neg for Finite<F> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

use std::ops::{Div, DivAssign, Mul, MulAssign, Rem, RemAssign};
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
impl<F: MulAssign + IsFinite, Rhs: IntoInner<F>> MulAssign<Rhs> for Finite<F> {
    #[track_caller]
    fn mul_assign(&mut self, rhs: Rhs) {
        self.0 *= rhs.into_inner();
    }
}
impl<F: Div<Output = F> + IsFinite, Rhs: IntoInner<F>> Div<Rhs> for Finite<F> {
    type Output = Self;
    #[track_caller]
    fn div(self, rhs: Rhs) -> Self::Output {
        Self(self.0 / rhs.into_inner())
    }
}
impl<F: DivAssign + IsFinite, Rhs: IntoInner<F>> DivAssign<Rhs> for Finite<F> {
    #[track_caller]
    fn div_assign(&mut self, rhs: Rhs) {
        self.0 /= rhs.into_inner();
    }
}
impl<F: Rem<Output = F> + IsFinite, Rhs: IntoInner<F>> Rem<Rhs> for Finite<F> {
    type Output = Self;
    #[track_caller]
    fn rem(self, rhs: Rhs) -> Self::Output {
        Self(self.0 % rhs.into_inner())
    }
}
impl<F: RemAssign + IsFinite, Rhs: IntoInner<F>> RemAssign<Rhs> for Finite<F> {
    #[track_caller]
    fn rem_assign(&mut self, rhs: Rhs) {
        self.0 %= rhs.into_inner();
    }
}

use crate::ops::Pow;
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
    /// Attempts to compute the reciprocal (`1/x`) of `self`.
    /// # Errors
    /// If the result is non-finite.
    pub fn try_recip(self) -> Result<Self, InfiniteError> {
        self.0.try_recip().map(Self)
    }
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
    /// Attempts to calculate the length of the hypotenuse of a
    /// right-angle triangle given legs of length `x` and `y`.
    /// # Errors
    /// If the output is non-finite.
    pub fn try_hypot(self, other: impl IntoInner<F>) -> Result<Self, InfiniteError> {
        self.0.try_hypot(other.into_inner()).map(Self)
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
    #[track_caller]
    #[must_use]
    pub fn recip(self) -> Self {
        Self(self.0.recip())
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
    #[track_caller]
    #[must_use]
    pub fn hypot(self, other: impl IntoInner<F>) -> Self {
        Self(self.0.hypot(other.into_inner()))
    }
}

use crate::ops::Exp;
impl<F: IsFinite + Exp> Finite<F> {
    /// Attempts to find `e^(self)`, the exponential function.
    /// # Errors
    /// If the output is non-finite.
    pub fn try_exp(self) -> Result<Self, InfiniteError> {
        self.0.try_exp().map(Self)
    }
    /// Attempts to find `2^(self)`.
    /// # Errors
    /// If the output is non-finite.
    pub fn try_exp2(self) -> Result<Self, InfiniteError> {
        self.0.try_exp().map(Self)
    }
    /// Attempts to find `e^(self) - 1` in a way that is accurate even if the number is close to zero.
    /// # Errors
    /// If the output is non-finite.
    pub fn try_exp_m1(self) -> Result<Self, InfiniteError> {
        self.0.try_exp_m1().map(Self)
    }
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
    /// If the output is non-finite.
    pub fn try_log10(self) -> Result<Self, InfiniteError> {
        self.0.try_log10().map(Self)
    }
    /// Attempts to find `ln(1+n)` (natural logarithm) more accurately than if the operations were performed separately.
    /// # Errors
    /// If the output is non-finite.
    pub fn try_ln_1p(self) -> Result<Self, InfiniteError> {
        self.0.try_ln_1p().map(Self)
    }

    #[track_caller]
    #[must_use]
    pub fn exp(self) -> Self {
        Self(self.0.exp())
    }
    #[track_caller]
    #[must_use]
    pub fn exp2(self) -> Self {
        Self(self.0.exp2())
    }
    #[track_caller]
    #[must_use]
    pub fn exp_m1(self) -> Self {
        Self(self.0.exp_m1())
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
    #[track_caller]
    #[must_use]
    pub fn ln_1p(self) -> Self {
        Self(self.0.ln_1p())
    }
}

use crate::ops::Trig;
impl<F: IsFinite + Trig> Finite<F> {
    /// Attempts to compute the tangent of a number (in radians).
    /// # Errors
    /// If the output is non-finite.
    pub fn try_tan(self) -> Result<Self, InfiniteError> {
        self.0.try_tan().map(Self)
    }
    /// Attempts to compute the arcsine of a number (in radians).
    /// # Errors
    /// If the output is NaN (caused if the magnitude of the input exceeds 1).
    pub fn try_asin(self) -> Result<Self, InfiniteError> {
        self.0.try_asin().map(Self)
    }
    /// Attempts to compute the arccosine of a number (in radians).
    /// # Errors
    /// If the output is NaN (caused if the magnitude of the input exceeds 1).
    pub fn try_acos(self) -> Result<Self, InfiniteError> {
        self.0.try_acos().map(Self)
    }
    /// Attempts to ompute the four quadrant arctangent of self (y) and other (x) in radians.
    /// # Errors
    /// If the output is NaN or non-finite.
    pub fn try_atan2(self, other: impl IntoInner<F>) -> Result<Self, InfiniteError> {
        self.0.try_atan2(other.into_inner()).map(Self)
    }

    #[cfg_attr(track_caller, debug_assertions)]
    #[must_use]
    pub fn sin(self) -> Self {
        // Sine always succeeds for any finite value
        Self::new_unchecked(self.val().sin())
    }
    #[cfg_attr(track_caller, debug_assertions)]
    #[must_use]
    pub fn cos(self) -> Self {
        // Cosine always succeeds for any finite value
        Self::new_unchecked(self.val().cos())
    }
    #[cfg_attr(track_caller, debug_assertions)]
    #[must_use]
    pub fn sin_cos(self) -> (Self, Self) {
        let (s, c) = self.val().sin_cos();
        (Self::new_unchecked(s), Self::new_unchecked(c))
    }
    #[track_caller]
    #[must_use]
    pub fn tan(self) -> Self {
        // tan might return infinity if you input PI/2
        Self(self.0.tan())
    }
    #[track_caller]
    #[must_use]
    pub fn asin(self) -> Self {
        Self(self.0.asin())
    }
    #[track_caller]
    #[must_use]
    pub fn acos(self) -> Self {
        Self(self.0.acos())
    }
    #[track_caller]
    #[must_use]
    pub fn atan(self) -> Self {
        Self(self.0.atan())
    }
    #[track_caller]
    #[must_use]
    pub fn atan2(self, other: impl IntoInner<F>) -> Self {
        Self(self.0.atan2(other.into_inner()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn unchecked() {
        let finite = unsafe { finite_unchecked!(f32::INFINITY) };
        assert!(finite.val().is_infinite());
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

    #[test]
    fn assert_pow() {
        assert_eq!(finite!(4.0f32).powf(3.5), finite!(128.0));
        assert_eq!(finite!(2.0f32).powi(8), finite!(256.0));
        assert_eq!(finite!(4.0f32).sqrt(), finite!(2.0));
        assert_eq!(finite!(27.0f32).cbrt(), finite!(3.0));
    }

    #[test]
    fn assert_exp() {
        assert_epsilon!(finite!(2.0f32).exp(), finite!(7.389_056));
        assert_epsilon!(finite!(3.0f32).exp2(), finite!(8.0));
        assert_epsilon!(finite!(5.0f32).exp_m1(), finite!(147.413_16));
        assert_epsilon!(finite!(16.0f32).log(4.0), finite!(2.0));
        assert_epsilon!(finite!(1.0f32).ln(), finite!(0.0));
        assert_epsilon!(finite!(8.0f32).log2(), finite!(3.0));
        assert_epsilon!(finite!(1000.0f32).log10(), finite!(3.0));
        assert_epsilon!(finite!(147.413_16f32).ln_1p(), finite!(5.0));
    }

    #[test]
    fn assert_trig() {
        use std::f32::consts::{FRAC_1_SQRT_2, PI};

        assert_epsilon!(finite!(0.0f32).sin(), finite!(0.0));
        assert_epsilon!(finite!(PI / 4.0).sin(), finite!(FRAC_1_SQRT_2));
        assert_epsilon!(finite!(PI / 2.0).sin(), finite!(1.0));

        assert_epsilon!(finite!(0.0f32).cos(), finite!(1.0));
        assert_epsilon!(finite!(PI / 4.0).cos(), finite!(FRAC_1_SQRT_2));
        assert_epsilon!(finite!(PI / 2.0).cos(), 0.0);

        assert_epsilon!(finite!(0.0f32).tan(), finite!(0.0));
        assert_epsilon!(finite!(PI / 4.0).tan(), finite!(1.0));
        // PI / 2 overflows to infinity

        assert_epsilon!(finite!(0.0f32).asin(), finite!(0.0));
        assert_epsilon!(finite!(FRAC_1_SQRT_2).asin(), finite!(PI / 4.0));
        assert_epsilon!(finite!(1.0f32).asin(), finite!(PI / 2.0));

        assert_epsilon!(finite!(0.0f32).acos(), finite!(PI / 2.0));
        assert_epsilon!(finite!(FRAC_1_SQRT_2).acos(), finite!(PI / 4.0));
        assert_epsilon!(finite!(1.0f32).acos(), finite!(0.0));

        assert_epsilon!(finite!(0.0f32).atan(), finite!(0.0));
        assert_epsilon!(finite!(1.0f32).atan(), finite!(PI / 4.0));
        // inf.atan() = infinity, can't show here
    }
}
