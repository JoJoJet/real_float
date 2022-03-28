use crate::{check::Checked, IntoInner};

/// The error produced when NaN is encountered.
#[derive(Debug, Clone, Copy)]
pub struct NanError;
impl std::fmt::Display for NanError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "encountered NaN unexpectedly")
    }
}

/// Trait for a floating point number that can be checked for NaN (not-a-number).
pub trait IsNan: Sized + Copy {
    fn is_nan(self) -> bool;
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

use crate::ops::Round;
impl<F: IsNan + Round> Real<F> {
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
impl<F: IsNan + Signed> Real<F> {
    /// Computes the absolute value of self.
    #[must_use]
    pub fn abs(self) -> Self {
        Self(self.0.abs())
    }
    /// Returns a number that represents the sign of self.
    /// * `1.0` if the number is positive, `+0.0` or `INFINITY`
    /// * `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
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

impl<F: IsNan + ToOrd> Real<F> {
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
impl<F: AddAssign + IsNan, Rhs: IntoInner<F>> AddAssign<Rhs> for Real<F> {
    #[track_caller]
    fn add_assign(&mut self, rhs: Rhs) {
        self.0 += rhs.into_inner();
    }
}
impl<F: Sub<Output = F> + IsNan, Rhs: IntoInner<F>> Sub<Rhs> for Real<F> {
    type Output = Self;
    #[track_caller]
    fn sub(self, rhs: Rhs) -> Self::Output {
        Self(self.0 - rhs.into_inner())
    }
}
impl<F: SubAssign + IsNan, Rhs: IntoInner<F>> SubAssign<Rhs> for Real<F> {
    #[track_caller]
    fn sub_assign(&mut self, rhs: Rhs) {
        self.0 -= rhs.into_inner();
    }
}
impl<F: Neg<Output = F> + IsNan> Neg for Real<F> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

use std::ops::{Div, DivAssign, Mul, MulAssign, Rem, RemAssign};
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
impl<F: MulAssign + IsNan, Rhs: IntoInner<F>> MulAssign<Rhs> for Real<F> {
    #[track_caller]
    fn mul_assign(&mut self, rhs: Rhs) {
        self.0 *= rhs.into_inner();
    }
}
impl<F: Div<Output = F> + IsNan, Rhs: IntoInner<F>> Div<Rhs> for Real<F> {
    type Output = Self;
    #[track_caller]
    fn div(self, rhs: Rhs) -> Self::Output {
        Self(self.0 / rhs.into_inner())
    }
}
impl<F: DivAssign + IsNan, Rhs: IntoInner<F>> DivAssign<Rhs> for Real<F> {
    #[track_caller]
    fn div_assign(&mut self, rhs: Rhs) {
        self.0 /= rhs.into_inner();
    }
}
impl<F: Rem<Output = F> + IsNan, Rhs: IntoInner<F>> Rem<Rhs> for Real<F> {
    type Output = Self;
    #[track_caller]
    fn rem(self, rhs: Rhs) -> Self::Output {
        Self(self.0 % rhs.into_inner())
    }
}
impl<F: RemAssign + IsNan, Rhs: IntoInner<F>> RemAssign<Rhs> for Real<F> {
    #[track_caller]
    fn rem_assign(&mut self, rhs: Rhs) {
        self.0 %= rhs.into_inner();
    }
}

use crate::ops::Pow;
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
    /// Attempts to calculate the length of the hypotenuse of a
    /// right-angle triangle given legs of length `x` and `y`.
    /// # Errors
    /// If the result is NaN.
    pub fn try_hypot(self, other: impl IntoInner<F>) -> Result<Self, NanError> {
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
impl<F: IsNan + Exp> Real<F> {
    /// Attempts to find `e^(self)`, the exponential function.
    /// # Errors
    /// If the result is NaN.
    pub fn try_exp(self) -> Result<Self, NanError> {
        self.0.try_exp().map(Self)
    }
    /// Attempts to find `2^(self)`.
    /// # Errors
    /// If the result is NaN.
    pub fn try_exp2(self) -> Result<Self, NanError> {
        self.0.try_exp().map(Self)
    }
    /// Attempts to find `e^(self) - 1` in a way that is accurate even if the number is close to zero.
    /// # Errors
    /// If the result is NaN.
    pub fn try_exp_m1(self) -> Result<Self, NanError> {
        self.0.try_exp_m1().map(Self)
    }
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
    /// Attempts to find `ln(1+n)` (natural logarithm) more accurately than if the operations were performed separately.
    /// # Errors
    /// If the result is NaN.
    pub fn try_ln_1p(self) -> Result<Self, NanError> {
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
impl<F: IsNan + Trig> Real<F> {
    /// Attempts to compute the sine of a number (in radians).
    /// # Errors
    /// If the output is NaN (caused if the input is `±infinity`).
    pub fn try_sin(self) -> Result<Self, NanError> {
        self.0.try_sin().map(Self)
    }
    /// Attempts to compute the cosine of a number (in radians).
    /// # Errors
    /// If the output is NaN (caused if the input is `±infinity`).
    pub fn try_cos(self) -> Result<Self, NanError> {
        self.0.try_cos().map(Self)
    }
    /// Attempts to compute both the sine and cosine of a number simultaneously (in radians).
    /// # Errors
    /// If the output is NaN (caused if the input is `±infinity`).
    pub fn try_sin_cos(self) -> Result<(Self, Self), NanError> {
        let (s, c) = self.0.try_sin_cos()?;
        Ok((Self(s), Self(c)))
    }
    /// Attempts to compute the tangent of a number (in radians).
    /// # Errors
    /// If the output is NaN (caused if the input is `±infinity`).
    pub fn try_tan(self) -> Result<Self, NanError> {
        self.0.try_tan().map(Self)
    }
    /// Attempts to compute the arcsine of a number (in radians).
    /// # Errors
    /// If the output is NaN (caused if the magnitude of the input exceeds 1).
    pub fn try_asin(self) -> Result<Self, NanError> {
        self.0.try_asin().map(Self)
    }
    /// Attempts to compute the arccosine of a number (in radians).
    /// # Errors
    /// If the output is NaN (caused if the magnitude of the input exceeds 1).
    pub fn try_acos(self) -> Result<Self, NanError> {
        self.0.try_acos().map(Self)
    }
    /// Attempts to ompute the four quadrant arctangent of self (y) and other (x) in radians.
    /// # Errors
    /// If the output is NaN.
    pub fn try_atan2(self, other: impl IntoInner<F>) -> Result<Self, NanError> {
        self.0.try_atan2(other.into_inner()).map(Self)
    }

    #[track_caller]
    #[must_use]
    pub fn sin(self) -> Self {
        Self(self.0.sin())
    }
    #[track_caller]
    #[must_use]
    pub fn cos(self) -> Self {
        Self(self.0.cos())
    }
    #[track_caller]
    #[must_use]
    pub fn sin_cos(self) -> (Self, Self) {
        let (s, c) = self.0.sin_cos();
        (Self(s), Self(c))
    }
    #[track_caller]
    #[must_use]
    pub fn tan(self) -> Self {
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
