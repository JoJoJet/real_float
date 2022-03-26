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

use std::ops::{Add, Neg, Sub};
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

use std::ops::{Div, Mul, Rem};
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

use crate::Pow;
impl<F: Copy + Pow, C: Check<F>> Checked<F, C> {
    pub fn try_powf(self, n: F) -> Result<Self, C::Error> {
        let output = self.0.powf(n);
        Self::try_new(output)
    }
    pub fn try_powi(self, n: i32) -> Result<Self, C::Error> {
        let output = self.0.powi(n);
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
}

use crate::Root;
impl<F: Copy + Root, C: Check<F>> Checked<F, C> {
    pub fn try_sqrt(self) -> Result<Self, C::Error> {
        let output = self.0.sqrt();
        Self::try_new(output)
    }
    pub fn try_cbrt(self) -> Result<Self, C::Error> {
        let output = self.0.cbrt();
        Self::try_new(output)
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
}

use crate::Log;
impl<F: Copy + Log, C: Check<F>> Checked<F, C> {
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
}
