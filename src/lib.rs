/// The error produced when NaN is encountered.
#[derive(Debug, Clone, Copy)]
pub struct NanError;
impl std::fmt::Display for NanError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "encountered NaN unexpectedly")
    }
}

/// whether or not to panic on NaN.
const STRICT: bool = cfg!(any(debug_assertions, feature = "strict"));

#[derive(Debug, Clone, Copy, Default)]
#[repr(transparent)]
pub struct Real<F: IsNan>(F);

mod ops;
pub use ops::*;

impl<F: IsNan> Real<F> {
    #[track_caller]
    pub fn new(val: F) -> Self {
        if STRICT {
            unwrap_display(Self::try_new(val))
        } else {
            Self(val)
        }
    }
    pub fn try_new(val: F) -> Result<Self, NanError> {
        // This is not a TryFrom implementation due to [this issue](https://github.com/rust-lang/rust/issues/50133).
        if val.is_nan() {
            Err(NanError)
        } else {
            Ok(Self(val))
        }
    }
}

/// Unwraps the inner value of a [`Real`].  
///
/// This trait is required instead of using std traits because of coherence rules.
#[doc(hidden)]
pub trait IntoInner<F>: Sized {
    fn into_inner(self) -> F;
}

impl<F: IsNan> IntoInner<F> for Real<F> {
    #[inline]
    fn into_inner(self) -> F {
        self.0
    }
}
impl<F: IsNan> IntoInner<F> for F {
    #[inline]
    fn into_inner(self) -> F {
        self
    }
}

mod bits;
pub use bits::*;

use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
impl<F: IsNan + ToOrd, Rhs: IntoInner<F> + Copy> PartialEq<Rhs> for Real<F> {
    fn eq(&self, rhs: &Rhs) -> bool {
        self.0.total_eq(rhs.into_inner())
    }
}
impl<F: IsNan + ToOrd> Eq for Real<F> {}

impl<F: IsNan + ToOrd> PartialOrd<F> for Real<F> {
    fn partial_cmp(&self, rhs: &F) -> Option<Ordering> {
        let lhs = self.0.to_ord();
        let rhs = Self::try_new(*rhs).ok()?.0.to_ord();
        Some(lhs.cmp(&rhs))
    }
}
impl<F: IsNan + ToOrd> PartialOrd for Real<F> {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}
impl<F: IsNan + ToOrd> Ord for Real<F> {
    fn cmp(&self, rhs: &Self) -> Ordering {
        let lhs = self.0.to_ord();
        let rhs = rhs.0.to_ord();
        lhs.cmp(&rhs)
    }
}

use std::ops::{Add, Div, Mul, Neg, Rem, Sub};
impl<F: IsNan> Real<F> {
    pub fn try_add(self, rhs: impl IntoInner<F>) -> Result<Self, NanError>
    where
        F: Add<Output = F>,
    {
        let output = self.0 + rhs.into_inner();
        Self::try_new(output)
    }
    pub fn try_sub(self, rhs: impl IntoInner<F>) -> Result<Self, NanError>
    where
        F: Sub<Output = F>,
    {
        let output = self.0 - rhs.into_inner();
        Self::try_new(output)
    }
    pub fn try_mul(self, rhs: impl IntoInner<F>) -> Result<Self, NanError>
    where
        F: Mul<Output = F>,
    {
        let output = self.0 * rhs.into_inner();
        Self::try_new(output)
    }
    pub fn try_div(self, rhs: impl IntoInner<F>) -> Result<Self, NanError>
    where
        F: Div<Output = F>,
    {
        let output = self.0 / rhs.into_inner();
        Self::try_new(output)
    }
    pub fn try_rem(self, rhs: impl IntoInner<F>) -> Result<Self, NanError>
    where
        F: Rem<Output = F>,
    {
        let output = self.0 % rhs.into_inner();
        Self::try_new(output)
    }
}

impl<F: Add<Output = F> + IsNan, Rhs: IntoInner<F>> Add<Rhs> for Real<F> {
    type Output = Self;
    #[track_caller]
    fn add(self, rhs: Rhs) -> Self::Output {
        let rhs = rhs.into_inner();
        if STRICT {
            unwrap_display(self.try_add(rhs))
        } else {
            Self(self.0 + rhs)
        }
    }
}
impl<F: Sub<Output = F> + IsNan, Rhs: IntoInner<F>> Sub<Rhs> for Real<F> {
    type Output = Self;
    #[track_caller]
    fn sub(self, rhs: Rhs) -> Self::Output {
        let rhs = rhs.into_inner();
        if STRICT {
            unwrap_display(self.try_sub(rhs))
        } else {
            Self(self.0 - rhs)
        }
    }
}
impl<F: Neg<Output = F> + IsNan> Neg for Real<F> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}
impl<F: Mul<Output = F> + IsNan, Rhs: IntoInner<F>> Mul<Rhs> for Real<F> {
    type Output = Self;
    #[track_caller]
    fn mul(self, rhs: Rhs) -> Self::Output {
        let rhs = rhs.into_inner();
        if STRICT {
            unwrap_display(self.try_mul(rhs))
        } else {
            Self(self.0 * rhs)
        }
    }
}
impl<F: Div<Output = F> + IsNan, Rhs: IntoInner<F>> Div<Rhs> for Real<F> {
    type Output = Self;
    #[track_caller]
    fn div(self, rhs: Rhs) -> Self::Output {
        let rhs = rhs.into_inner();
        if STRICT {
            unwrap_display(self.try_div(rhs))
        } else {
            Self(self.0 / rhs)
        }
    }
}
impl<F: Rem<Output = F> + IsNan, Rhs: IntoInner<F>> Rem<Rhs> for Real<F> {
    type Output = Self;
    #[track_caller]
    fn rem(self, rhs: Rhs) -> Self::Output {
        let rhs = rhs.into_inner();
        if STRICT {
            unwrap_display(self.try_rem(rhs))
        } else {
            Self(self.0 % rhs)
        }
    }
}

impl<F: IsNan + Pow> Real<F> {
    pub fn try_powf(self, n: impl IntoInner<F>) -> Result<Self, NanError> {
        let output = self.0.powf(n.into_inner());
        Self::try_new(output)
    }
    pub fn try_powi(self, n: i32) -> Result<Self, NanError> {
        let output = self.0.powi(n);
        Self::try_new(output)
    }
    #[track_caller]
    pub fn powf(self, n: impl IntoInner<F>) -> Self {
        let n = n.into_inner();
        if STRICT {
            unwrap_display(self.try_powf(n))
        } else {
            Self(self.0.powf(n))
        }
    }
    #[track_caller]
    pub fn powi(self, n: i32) -> Self {
        if STRICT {
            unwrap_display(self.try_powi(n))
        } else {
            Self(self.0.powi(n))
        }
    }
}

impl<F: IsNan + Root> Real<F> {
    pub fn try_sqrt(self) -> Result<Self, NanError> {
        let output = self.0.sqrt();
        Self::try_new(output)
    }
    pub fn try_cbrt(self) -> Result<Self, NanError> {
        let output = self.0.cbrt();
        Self::try_new(output)
    }
    #[track_caller]
    pub fn sqrt(self) -> Self {
        if STRICT {
            unwrap_display(self.try_sqrt())
        } else {
            Self(self.0.sqrt())
        }
    }
    #[track_caller]
    pub fn cbrt(self) -> Self {
        if STRICT {
            unwrap_display(self.try_cbrt())
        } else {
            Self(self.0.cbrt())
        }
    }
}

impl<F: IsNan + Log> Real<F> {
    pub fn try_log(self, rhs: impl IntoInner<F>) -> Result<Self, NanError> {
        let output = self.0.log(rhs.into_inner());
        Self::try_new(output)
    }
    pub fn try_ln(self) -> Result<Self, NanError> {
        let output = self.0.ln();
        Self::try_new(output)
    }
    pub fn try_log2(self) -> Result<Self, NanError> {
        let output = self.0.log2();
        Self::try_new(output)
    }
    pub fn try_log10(self) -> Result<Self, NanError> {
        let output = self.0.log10();
        Self::try_new(output)
    }

    #[track_caller]
    pub fn log(self, base: impl IntoInner<F>) -> Self {
        let base = base.into_inner();
        if STRICT {
            unwrap_display(self.try_log(base))
        } else {
            Self(self.0.log(base))
        }
    }
    #[track_caller]
    pub fn ln(self) -> Self {
        if STRICT {
            unwrap_display(self.try_ln())
        } else {
            Self(self.0.ln())
        }
    }
    #[track_caller]
    pub fn log2(self) -> Self {
        if STRICT {
            unwrap_display(self.try_log2())
        } else {
            Self(self.0.log2())
        }
    }
    #[track_caller]
    pub fn log10(self) -> Self {
        if STRICT {
            unwrap_display(self.try_log10())
        } else {
            Self(self.0.log10())
        }
    }
}

#[track_caller]
fn unwrap_display<T, E: std::fmt::Display>(res: Result<T, E>) -> T {
    match res {
        Ok(val) => val,
        Err(e) => panic_display(&e),
    }
}
#[inline(never)]
#[cold]
#[track_caller]
fn panic_display(error: &dyn std::fmt::Display) -> ! {
    panic!("{}", error)
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
