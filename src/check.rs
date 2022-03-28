use std::{fmt, marker::PhantomData};

use crate::{unwrap_display, ToOrd};

/// whether or not to panic on NaN.
pub(crate) const STRICT: bool = cfg!(any(debug_assertions, feature = "strict"));

pub(crate) trait Check<F> {
    type Error: fmt::Display;
    fn check(_: F) -> Result<F, Self::Error>;
}

#[repr(transparent)]
pub(crate) struct Checked<F, C: Check<F>>(F, PhantomData<C>);

macro_rules! checked {
    ($val: expr) => {
        Checked($val, PhantomData)
    };
}

impl<F: Copy, C: Check<F>> Checked<F, C> {
    pub fn try_new(val: F) -> Result<Self, C::Error> {
        // This is not a TryFrom implementation due to [this issue](https://github.com/rust-lang/rust/issues/50133).
        let val = C::check(val)?;
        Ok(checked!(val))
    }
    #[track_caller]
    pub fn new(val: F) -> Self {
        if STRICT {
            unwrap_display(Self::try_new(val))
        } else {
            checked!(val)
        }
    }
    pub fn val(self) -> F {
        self.0
    }
}

impl<F: fmt::Debug, C: Check<F>> fmt::Debug for Checked<F, C> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}
impl<F: fmt::Display, C: Check<F>> fmt::Display for Checked<F, C> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl<F: Clone, C: Check<F>> Clone for Checked<F, C> {
    #[inline]
    fn clone(&self) -> Self {
        checked!(self.0.clone())
    }
    #[inline]
    fn clone_from(&mut self, other: &Self) {
        self.0.clone_from(&other.0);
    }
}
impl<F: Copy, C: Check<F>> Copy for Checked<F, C> {}

impl<F: Default, C: Check<F>> Default for Checked<F, C> {
    #[inline]
    fn default() -> Self {
        checked!(F::default())
    }
}

use std::cmp::{Eq, PartialEq};
impl<F: ToOrd, C: Check<F>> PartialEq<F> for Checked<F, C> {
    fn eq(&self, rhs: &F) -> bool {
        // we can ignore the case where `rhs` is NaN since
        // we know that `self` is not NaN.
        self.0.total_eq(*rhs)
    }
}
impl<F: ToOrd, C: Check<F>> PartialEq for Checked<F, C> {
    fn eq(&self, rhs: &Self) -> bool {
        self.0.total_eq(rhs.0)
    }
}
impl<F: ToOrd, C: Check<F>> Eq for Checked<F, C> {}

use std::cmp::{Ord, Ordering, PartialOrd};
impl<F: ToOrd, C: Check<F>> PartialOrd<F> for Checked<F, C> {
    fn partial_cmp(&self, rhs: &F) -> Option<Ordering> {
        let rhs = C::check(*rhs).ok()?.to_ord();
        let lhs = self.0.to_ord();
        Some(lhs.cmp(&rhs))
    }
}
impl<F: ToOrd, C: Check<F>> PartialOrd for Checked<F, C> {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}
impl<F: ToOrd, C: Check<F>> Ord for Checked<F, C> {
    fn cmp(&self, rhs: &Self) -> Ordering {
        let lhs = self.0.to_ord();
        let rhs = rhs.0.to_ord();
        lhs.cmp(&rhs)
    }
}

use crate::ops::Round;
impl<F: Round, C: Check<F>> Checked<F, C> {
    pub fn floor(self) -> Self {
        checked!(self.0.floor())
    }
    pub fn ceil(self) -> Self {
        checked!(self.0.ceil())
    }
    pub fn round(self) -> Self {
        checked!(self.0.round())
    }
    pub fn trunc(self) -> Self {
        checked!(self.0.trunc())
    }
    pub fn fract(self) -> Self {
        checked!(self.0.fract())
    }
}

use crate::ops::Signed;
impl<F: Signed, C: Check<F>> Checked<F, C> {
    pub fn abs(self) -> Self {
        checked!(self.0.abs())
    }
    pub fn signum(self) -> Self {
        checked!(self.0.signum())
    }
    pub fn is_sign_positive(self) -> bool {
        self.0.is_sign_positive()
    }
    pub fn is_sign_negative(self) -> bool {
        self.0.is_sign_negative()
    }
}

impl<F: Copy + ToOrd, C: Check<F>> Checked<F, C> {
    pub fn max(self, other: F) -> Self {
        match self.partial_cmp(&other) {
            Some(std::cmp::Ordering::Less) => checked!(other),
            _ => self,
        }
    }
    pub fn min(self, other: F) -> Self {
        match self.partial_cmp(&other) {
            Some(std::cmp::Ordering::Less) => self,
            _ => checked!(other),
        }
    }
}

use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};
impl<F: Copy, C: Check<F>> Checked<F, C> {
    pub fn try_add(self, rhs: F) -> Result<Self, C::Error>
    where
        F: Add<Output = F>,
    {
        let output = self.0 + rhs;
        Self::try_new(output)
    }
    pub fn try_sub(self, rhs: F) -> Result<Self, C::Error>
    where
        F: Sub<Output = F>,
    {
        let output = self.0 - rhs;
        Self::try_new(output)
    }
    pub fn try_neg(self) -> Result<Self, C::Error>
    where
        F: Neg<Output = F>,
    {
        let output = -self.0;
        Self::try_new(output)
    }
}
impl<F: Add<Output = F> + Copy, C: Check<F>> Add<F> for Checked<F, C> {
    type Output = Self;
    #[track_caller]
    fn add(self, rhs: F) -> Self::Output {
        if STRICT {
            unwrap_display(self.try_add(rhs))
        } else {
            checked!(self.0 + rhs)
        }
    }
}
impl<F: AddAssign + Copy, C: Check<F>> AddAssign<F> for Checked<F, C> {
    #[track_caller]
    fn add_assign(&mut self, rhs: F) {
        self.0 += rhs;
        if STRICT {
            unwrap_display(C::check(self.0));
        }
    }
}
impl<F: Sub<Output = F> + Copy, C: Check<F>> Sub<F> for Checked<F, C> {
    type Output = Self;
    #[track_caller]
    fn sub(self, rhs: F) -> Self::Output {
        if STRICT {
            unwrap_display(self.try_sub(rhs))
        } else {
            checked!(self.0 - rhs)
        }
    }
}
impl<F: SubAssign + Copy, C: Check<F>> SubAssign<F> for Checked<F, C> {
    #[track_caller]
    fn sub_assign(&mut self, rhs: F) {
        self.0 -= rhs;
        if STRICT {
            unwrap_display(C::check(self.0));
        }
    }
}
impl<F: Neg<Output = F> + Copy, C: Check<F>> Neg for Checked<F, C> {
    type Output = Self;
    #[track_caller]
    fn neg(self) -> Self::Output {
        if STRICT {
            unwrap_display(self.try_neg())
        } else {
            checked!(-self.0)
        }
    }
}

use std::ops::{Div, DivAssign, Mul, MulAssign, Rem, RemAssign};
impl<F: Copy, C: Check<F>> Checked<F, C> {
    pub fn try_mul(self, rhs: F) -> Result<Self, C::Error>
    where
        F: Mul<Output = F>,
    {
        let output = self.0 * rhs;
        Self::try_new(output)
    }
    pub fn try_div(self, rhs: F) -> Result<Self, C::Error>
    where
        F: Div<Output = F>,
    {
        let output = self.0 / rhs;
        Self::try_new(output)
    }
    pub fn try_rem(self, rhs: F) -> Result<Self, C::Error>
    where
        F: Rem<Output = F>,
    {
        let output = self.0 % rhs;
        Self::try_new(output)
    }
}
impl<F: Mul<Output = F> + Copy, C: Check<F>> Mul<F> for Checked<F, C> {
    type Output = Self;
    #[track_caller]
    fn mul(self, rhs: F) -> Self::Output {
        if STRICT {
            unwrap_display(self.try_mul(rhs))
        } else {
            checked!(self.0 * rhs)
        }
    }
}
impl<F: MulAssign + Copy, C: Check<F>> MulAssign<F> for Checked<F, C> {
    #[track_caller]
    fn mul_assign(&mut self, rhs: F) {
        self.0 *= rhs;
        if STRICT {
            unwrap_display(C::check(self.0));
        }
    }
}
impl<F: Div<Output = F> + Copy, C: Check<F>> Div<F> for Checked<F, C> {
    type Output = Self;
    #[track_caller]
    fn div(self, rhs: F) -> Self::Output {
        if STRICT {
            unwrap_display(self.try_div(rhs))
        } else {
            checked!(self.0 / rhs)
        }
    }
}
impl<F: DivAssign + Copy, C: Check<F>> DivAssign<F> for Checked<F, C> {
    #[track_caller]
    fn div_assign(&mut self, rhs: F) {
        self.0 /= rhs;
        if STRICT {
            unwrap_display(C::check(self.0));
        }
    }
}
impl<F: Rem<Output = F> + Copy, C: Check<F>> Rem<F> for Checked<F, C> {
    type Output = Self;
    #[track_caller]
    fn rem(self, rhs: F) -> Self::Output {
        if STRICT {
            unwrap_display(self.try_rem(rhs))
        } else {
            checked!(self.0 % rhs)
        }
    }
}
impl<F: RemAssign + Copy, C: Check<F>> RemAssign<F> for Checked<F, C> {
    #[track_caller]
    fn rem_assign(&mut self, rhs: F) {
        self.0 %= rhs;
        if STRICT {
            unwrap_display(C::check(self.0));
        }
    }
}

use crate::ops::Pow;
impl<F: Copy + Pow, C: Check<F>> Checked<F, C> {
    pub fn try_powf(self, n: F) -> Result<Self, C::Error> {
        let output = self.0.powf(n);
        Self::try_new(output)
    }
    pub fn try_powi(self, n: i32) -> Result<Self, C::Error> {
        let output = self.0.powi(n);
        Self::try_new(output)
    }
    pub fn try_sqrt(self) -> Result<Self, C::Error> {
        let output = self.0.sqrt();
        Self::try_new(output)
    }
    pub fn try_cbrt(self) -> Result<Self, C::Error> {
        let output = self.0.cbrt();
        Self::try_new(output)
    }
    pub fn try_hypot(self, other: F) -> Result<Self, C::Error> {
        let output = self.0.hypot(other);
        Self::try_new(output)
    }

    #[track_caller]
    pub fn powf(self, n: F) -> Self {
        if STRICT {
            unwrap_display(self.try_powf(n))
        } else {
            checked!(self.0.powf(n))
        }
    }
    #[track_caller]
    pub fn powi(self, n: i32) -> Self {
        if STRICT {
            unwrap_display(self.try_powi(n))
        } else {
            checked!(self.0.powi(n))
        }
    }
    #[track_caller]
    pub fn sqrt(self) -> Self {
        if STRICT {
            unwrap_display(self.try_sqrt())
        } else {
            checked!(self.0.sqrt())
        }
    }
    #[track_caller]
    pub fn cbrt(self) -> Self {
        if STRICT {
            unwrap_display(self.try_cbrt())
        } else {
            checked!(self.0.cbrt())
        }
    }
    #[track_caller]
    pub fn hypot(self, other: F) -> Self {
        if STRICT {
            unwrap_display(self.try_hypot(other))
        } else {
            checked!(self.0.hypot(other))
        }
    }
}

use crate::ops::Exp;
impl<F: Copy + Exp, C: Check<F>> Checked<F, C> {
    pub fn try_exp(self) -> Result<Self, C::Error> {
        let output = self.0.exp();
        Self::try_new(output)
    }
    pub fn try_exp2(self) -> Result<Self, C::Error> {
        let output = self.0.exp2();
        Self::try_new(output)
    }
    pub fn try_exp_m1(self) -> Result<Self, C::Error> {
        let output = self.0.exp_m1();
        Self::try_new(output)
    }
    pub fn try_log(self, b: F) -> Result<Self, C::Error> {
        let output = self.0.log(b);
        Self::try_new(output)
    }
    pub fn try_ln(self) -> Result<Self, C::Error> {
        let output = self.0.ln();
        Self::try_new(output)
    }
    pub fn try_log2(self) -> Result<Self, C::Error> {
        let output = self.0.log2();
        Self::try_new(output)
    }
    pub fn try_log10(self) -> Result<Self, C::Error> {
        let output = self.0.log10();
        Self::try_new(output)
    }
    pub fn try_ln_1p(self) -> Result<Self, C::Error> {
        let output = self.0.ln_1p();
        Self::try_new(output)
    }

    #[track_caller]
    pub fn exp(self) -> Self {
        if STRICT {
            unwrap_display(self.try_exp())
        } else {
            checked!(self.0.exp())
        }
    }
    #[track_caller]
    pub fn exp2(self) -> Self {
        if STRICT {
            unwrap_display(self.try_exp2())
        } else {
            checked!(self.0.exp2())
        }
    }
    #[track_caller]
    pub fn exp_m1(self) -> Self {
        if STRICT {
            unwrap_display(self.try_exp_m1())
        } else {
            checked!(self.0.exp_m1())
        }
    }
    #[track_caller]
    pub fn log(self, base: F) -> Self {
        if STRICT {
            unwrap_display(self.try_log(base))
        } else {
            checked!(self.0.log(base))
        }
    }
    #[track_caller]
    pub fn ln(self) -> Self {
        if STRICT {
            unwrap_display(self.try_ln())
        } else {
            checked!(self.0.ln())
        }
    }
    #[track_caller]
    pub fn log2(self) -> Self {
        if STRICT {
            unwrap_display(self.try_log2())
        } else {
            checked!(self.0.log2())
        }
    }
    #[track_caller]
    pub fn log10(self) -> Self {
        if STRICT {
            unwrap_display(self.try_log10())
        } else {
            checked!(self.0.log10())
        }
    }
    #[track_caller]
    pub fn ln_1p(self) -> Self {
        if STRICT {
            unwrap_display(self.try_ln_1p())
        } else {
            checked!(self.0.ln_1p())
        }
    }
}

use crate::ops::Trig;
impl<F: Copy + Trig, C: Check<F>> Checked<F, C> {
    pub fn try_sin(self) -> Result<Self, C::Error> {
        let output = self.0.sin();
        Self::try_new(output)
    }
    pub fn try_cos(self) -> Result<Self, C::Error> {
        let output = self.0.cos();
        Self::try_new(output)
    }
    pub fn try_sin_cos(self) -> Result<(Self, Self), C::Error> {
        let (s, c) = self.0.sin_cos();
        let s = Self::try_new(s)?;
        let c = Self::try_new(c)?;
        Ok((s, c))
    }
    pub fn try_tan(self) -> Result<Self, C::Error> {
        let output = self.0.tan();
        Self::try_new(output)
    }
    pub fn try_asin(self) -> Result<Self, C::Error> {
        let output = self.0.asin();
        Self::try_new(output)
    }
    pub fn try_acos(self) -> Result<Self, C::Error> {
        let output = self.0.acos();
        Self::try_new(output)
    }
    pub fn try_atan(self) -> Result<Self, C::Error> {
        let output = self.0.atan();
        Self::try_new(output)
    }
    pub fn try_atan2(self, other: F) -> Result<Self, C::Error> {
        let output = self.0.atan2(other);
        Self::try_new(output)
    }

    #[track_caller]
    pub fn sin(self) -> Self {
        if STRICT {
            unwrap_display(self.try_sin())
        } else {
            checked!(self.0.sin())
        }
    }
    #[track_caller]
    pub fn cos(self) -> Self {
        if STRICT {
            unwrap_display(self.try_cos())
        } else {
            checked!(self.0.cos())
        }
    }
    #[track_caller]
    pub fn sin_cos(self) -> (Self, Self) {
        if STRICT {
            unwrap_display(self.try_sin_cos())
        } else {
            let (s, c) = self.0.sin_cos();
            (checked!(s), checked!(c))
        }
    }
    #[track_caller]
    pub fn tan(self) -> Self {
        if STRICT {
            unwrap_display(self.try_tan())
        } else {
            checked!(self.0.tan())
        }
    }
    #[track_caller]
    pub fn asin(self) -> Self {
        if STRICT {
            unwrap_display(self.try_asin())
        } else {
            checked!(self.0.asin())
        }
    }
    #[track_caller]
    pub fn acos(self) -> Self {
        if STRICT {
            unwrap_display(self.try_acos())
        } else {
            checked!(self.0.acos())
        }
    }
    #[track_caller]
    pub fn atan(self) -> Self {
        if STRICT {
            unwrap_display(self.try_atan())
        } else {
            checked!(self.0.atan())
        }
    }
    #[track_caller]
    pub fn atan2(self, other: F) -> Self {
        if STRICT {
            unwrap_display(self.try_atan2(other))
        } else {
            checked!(self.0.atan2(other))
        }
    }
}
