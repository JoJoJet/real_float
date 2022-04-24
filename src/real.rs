use crate::IntoInner;

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

/// Constructor for [`Real`] that never checks the value, and can be used in a const context.
/// # Safety
/// Ensure that the value can never be `NaN`.
#[macro_export]
macro_rules! real_unchecked {
    ($f: expr) => {{
        union Transmute<F: $crate::IsNan> {
            inner: F,
            real: $crate::Real<F>,
        }

        // SAFETY: `Real` is `repr(transparent)`.
        let val = Transmute { inner: $f };
        val.real
    }};
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(transparent)]
pub struct Real<F: IsNan>(F);

impl<F: IsNan> Real<F> {
    /// Attempts to create a new `Real` float.
    /// # Errors
    /// If the value is NaN.
    pub fn try_new(val: F) -> Result<Self, NanError> {
        if val.is_nan() {
            Err(NanError)
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

ctor_impls!(Real<F: IsNan>, "If the number is NaN.");

impl<F: IsNan> IntoInner<F> for Real<F> {
    #[inline]
    fn into_inner(self) -> F {
        self.val()
    }
}

eq_impls!(Real<F: IsNan>);
ord_impls!(Real<F: IsNan>);
round_impls!(Real<F: IsNan>);
signed_impls!(Real<F: IsNan>);
sum_impls!(Real<F: IsNan>, NanError, "If the result is NaN.");
neg_impls!(Real<F: IsNan>, NanError, "If the result is NaN.");
product_impls!(Real<F: IsNan>, NanError, "If the result is NaN.");
impl<F: IsNan + crate::ops::Pow> Real<F> {
    pow_methods!(F, NanError, "If the result is NaN.");
    recip_methods!(F); // recip is infallible for real numbers
    sqrt_methods!(F, NanError, "If the result is NaN.");
    cbrt_methods!(F);
    hypot_methods!(F, NanError, "If the result is NaN.");
}
exp_impls!(Real<F: IsNan>, NanError, "If the result is NaN.");
impl<F: IsNan + crate::ops::Trig> Real<F> {
    sin_cos_methods!(
        F,
        NanError,
        "If the output is NaN (caused if the input is `±infinity`)."
    );
    tan_methods!(
        F,
        NanError,
        "If the result is NaN (caused if the input is `±infinity`)."
    );
    asin_acos_methods!(
        F,
        NanError,
        "If the output is NaN (caused if the magnitude of the input exceeds 1)."
    );
    atan_methods!(F); // atan always succeeds for real inputs.
    atan2_methods!(F, NanError, "If the output is NaN.");
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn unchecked() {
        let real = unsafe { real_unchecked!(f32::NAN) };
        assert!(real.val().is_nan());
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

        assert_err!(real!(f32::INFINITY).try_sin());
        assert_err!(real!(f32::INFINITY).try_cos());
        assert_err!(real!(f32::INFINITY).try_tan());
    }

    #[test]
    fn assert_ops() {
        assert_eq!(real!(2.0f32) + 1.0, real!(3.0));
        assert_eq!(real!(2.0f32) - 1.0, real!(1.0));
        assert_eq!(real!(5.0f32) * 2.0, real!(10.0));
        assert_eq!(real!(8.0f32) / 2.0, real!(4.0));
        assert_eq!(-real!(1.0f32), real!(-1.0));
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

    #[test]
    fn assert_pow() {
        assert_eq!(real!(1000.0f32).powf(1000.0), real!(f32::INFINITY));
        assert_eq!(real!(4.0f32).powf(3.5), real!(128.0));
        assert_eq!(real!(2.0f32).powi(8), real!(256.0));
        assert_eq!(real!(2.0f32).recip(), real!(0.5));
        assert_eq!(real!(4.0f32).sqrt(), real!(2.0));
        assert_eq!(real!(27.0f32).cbrt(), real!(3.0));
    }

    #[test]
    fn assert_exp() {
        assert_epsilon!(real!(2.0f32).exp(), real!(7.389_056));
        assert_epsilon!(real!(3.0f32).exp2(), real!(8.0));
        assert_epsilon!(real!(5.0f32).exp_m1(), real!(147.413_16));
        assert_epsilon!(real!(16.0f32).log(4.0), real!(2.0));
        assert_epsilon!(real!(1.0f32).ln(), real!(0.0));
        assert_epsilon!(real!(8.0f32).log2(), real!(3.0));
        assert_epsilon!(real!(1000.0f32).log10(), real!(3.0));
        assert_epsilon!(real!(147.413_16f32).ln_1p(), real!(5.0));
    }

    #[test]
    fn assert_trig() {
        use std::f32::consts::{FRAC_1_SQRT_2, PI};

        assert_epsilon!(real!(0.0f32).sin(), real!(0.0));
        assert_epsilon!(real!(PI / 4.0).sin(), real!(FRAC_1_SQRT_2));
        assert_epsilon!(real!(PI / 2.0).sin(), real!(1.0));

        assert_epsilon!(real!(0.0f32).cos(), real!(1.0));
        assert_epsilon!(real!(PI / 4.0).cos(), real!(FRAC_1_SQRT_2));
        assert_epsilon!(real!(PI / 2.0).cos(), 0.0);

        assert_epsilon!(real!(0.0f32).tan(), real!(0.0));
        assert_epsilon!(real!(PI / 4.0).tan(), real!(1.0));
        assert!(real!(PI / 2.0 - f32::EPSILON).tan() > real!(2_000_000.0)); // its big

        assert_epsilon!(real!(0.0f32).asin(), real!(0.0));
        assert_epsilon!(real!(FRAC_1_SQRT_2).asin(), real!(PI / 4.0));
        assert_epsilon!(real!(1.0f32).asin(), real!(PI / 2.0));

        assert_epsilon!(real!(0.0f32).acos(), real!(PI / 2.0));
        assert_epsilon!(real!(FRAC_1_SQRT_2).acos(), real!(PI / 4.0));
        assert_epsilon!(real!(1.0f32).acos(), real!(0.0));

        assert_epsilon!(real!(0.0f32).atan(), real!(0.0));
        assert_epsilon!(real!(1.0f32).atan(), real!(PI / 4.0));
        assert_epsilon!(real!(f32::INFINITY).atan(), real!(PI / 2.0));
    }
}
