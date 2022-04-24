use crate::{ops::Signed, IntoInner};

/// The error produced when a negative or NaN value is encountered.
#[derive(Debug, Clone, Copy)]
pub struct NegativeError;
impl std::fmt::Display for NegativeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "encountered a negative or NaN unexpectedly")
    }
}

#[doc(hidden)]
pub trait IsNegative: crate::ops::Signed + crate::IsNan {}
impl<T: Signed + crate::IsNan> IsNegative for T {}

/// Constructor for [`NonNeg`]ative that never checks the value, and can be used in a const context.
/// # Safety
/// Ensure that the value can never be `NaN` or negative.
#[macro_export]
macro_rules! non_neg_unchecked {
    ($f: expr) => {{
        union Transmute<F: $crate::IsNegative> {
            inner: F,
            nn: $crate::NonNeg<F>,
        }

        // SAFETY: `NonNeg` is `repr(transparent)`.
        let val = Transmute { inner: $f };
        val.nn
    }};
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(transparent)]
pub struct NonNeg<F: IsNegative>(F);

impl<F: IsNegative> NonNeg<F> {
    /// Attempts to create a new `NonNeg`ative float.
    /// # Errors
    /// If the value is negative or NaN.
    pub fn try_new(val: F) -> Result<Self, NegativeError> {
        if val.is_sign_negative() || val.is_nan() {
            Err(NegativeError)
        } else {
            Ok(Self(val))
        }
    }
    /// Gets the inner value of this number.
    #[inline]
    pub fn val(self) -> F {
        self.0
    }
}

ctor_impls!(NonNeg<F: IsNegative>, "If the number is negative or NaN.");

impl<F: IsNegative> IntoInner<F> for NonNeg<F> {
    #[inline]
    fn into_inner(self) -> F {
        self.val()
    }
}

eq_impls!(NonNeg<F: IsNegative>);
ord_impls!(NonNeg<F: IsNegative>);
round_impls!(NonNeg<F: IsNegative>);
signed_impls!(NonNeg<F: IsNegative>);
sum_impls!(
    NonNeg<F: IsNegative>,
    NegativeError,
    "If the result is negative or NaN."
);
// neg is not defined
product_impls!(
    NonNeg<F: IsNegative>,
    NegativeError,
    "If the result is negative or NaN."
);
impl<F: IsNegative + crate::ops::Pow> NonNeg<F> {
    pow_methods!(F, NegativeError, "If the result is negative or NaN.");
    recip_methods!(F, NegativeError, "If the result is negative or NaN.");
    sqrt_methods!(F);
    cbrt_methods!(F);
    hypot_methods!(F, NegativeError, "If the result is negative or NaN.");
}
exp_impls!(
    NonNeg<F: IsNegative>,
    NegativeError,
    "If the result is negative or NaN."
);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! nn {
        ($f: expr) => {
            NonNeg::new($f)
        };
    }

    #[test]
    #[should_panic]
    fn assert_new_nan() {
        nn!(f32::NAN);
    }
    #[test]
    #[should_panic]
    fn assert_new_nan2() {
        nn!(-f32::NAN);
    }
    #[test]
    #[should_panic]
    fn assert_new_inf2() {
        nn!(f32::NEG_INFINITY);
    }
    #[test]
    #[should_panic]
    fn assert_new_neg() {
        nn!(-1.0f32);
    }

    #[test]
    fn unchecked() {
        let finite = unsafe { non_neg_unchecked!(f32::INFINITY) };
        assert!(finite.val().is_infinite());
    }

    #[test]
    fn assert_nan() {
        assert_err!(nn!(f32::INFINITY).try_add(f32::NEG_INFINITY));
        assert_err!(nn!(f32::INFINITY).try_sub(f32::INFINITY));
        assert_err!(nn!(1.0f32).try_add(-2.0));
        assert_err!(nn!(1.0f32).try_sub(2.0));
        assert_err!(nn!(1.0f32).try_mul(-1.0));
        assert_err!(nn!(1.0f32).try_rem(0.0));

        assert_err!(nn!(0.5f32).try_log(3.0));
        assert_err!(nn!(0.5f32).try_ln());
        assert_err!(nn!(0.5f32).try_log2());
        assert_err!(nn!(0.5f32).try_log10());
    }

    #[test]
    fn assert_ops() {
        assert_eq!(nn!(2.0f32) + 1.0, nn!(3.0));
        assert_eq!(nn!(2.0f32) - 1.0, nn!(1.0));
        assert_eq!(nn!(5.0f32) * 2.0, nn!(10.0));
        assert_eq!(nn!(8.0f32) / 2.0, nn!(4.0));
    }

    #[test]
    #[allow(clippy::bool_assert_comparison)]
    #[allow(clippy::cmp_nan)]
    fn assert_cmp_weird() {
        assert_eq!(nn!(0.0f32), nn!(0.0));
        assert_eq!(nn!(0.0f32), crate::Finite::new(-0.0));

        assert!(nn!(0.0) < nn!(1.0));

        assert_eq!(nn!(1.0) < f32::NAN, false);
        assert_eq!(nn!(1.0) >= f32::NAN, false);
    }

    #[test]
    fn assert_pow() {
        assert_eq!(nn!(4.0f32).powf(3.5), nn!(128.0));
        assert_eq!(nn!(2.0f32).powi(8), nn!(256.0));
        assert_eq!(nn!(2.0f32).recip(), nn!(0.5));
        assert_eq!(nn!(4.0f32).sqrt(), nn!(2.0));
        assert_eq!(nn!(27.0f32).cbrt(), nn!(3.0));
    }

    #[test]
    fn assert_exp() {
        assert_epsilon!(nn!(2.0f32).exp(), nn!(7.389_056));
        assert_epsilon!(nn!(3.0f32).exp2(), nn!(8.0));
        assert_epsilon!(nn!(5.0f32).exp_m1(), nn!(147.413_16));
        assert_epsilon!(nn!(16.0f32).log(4.0), nn!(2.0));
        assert_epsilon!(nn!(1.0f32).ln(), nn!(0.0));
        assert_epsilon!(nn!(8.0f32).log2(), nn!(3.0));
        assert_epsilon!(nn!(1000.0f32).log10(), nn!(3.0));
        assert_epsilon!(nn!(147.413_16f32).ln_1p(), nn!(5.0));
    }
}
