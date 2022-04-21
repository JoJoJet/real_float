use crate::IntoInner;

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
pub struct Finite<F: IsFinite>(F);

impl<F: IsFinite> Finite<F> {
    /// Attempts to create a new `Finite` float.
    /// # Errors
    /// If the value is non-finite.
    pub fn try_new(val: F) -> Result<Self, InfiniteError> {
        if val.is_finite() {
            Ok(Self(val))
        } else {
            Err(InfiniteError)
        }
    }
    /// Gets the inner value of this number.
    #[inline]
    pub fn val(self) -> F {
        self.0
    }
}

ctor_impls!(Finite<F: IsFinite>, "If the number is non-finite.");

impl<F: IsFinite> IntoInner<F> for Finite<F> {
    #[inline]
    fn into_inner(self) -> F {
        self.val()
    }
}

eq_impls!(Finite<F: IsFinite>);
ord_impls!(Finite<F: IsFinite>);
round_impls!(Finite<F: IsFinite>);
signed_impls!(Finite<F: IsFinite>);
sum_impls!(
    Finite<F: IsFinite>,
    InfiniteError,
    "If the result is non-finite."
);
neg_impls!(
    Finite<F: IsFinite>,
    InfiniteError,
    "If the result is non-finite."
);
product_impls!(
    Finite<F: IsFinite>,
    InfiniteError,
    "If the result is non-finite."
);
impl<F: IsFinite + crate::ops::Pow> Finite<F> {
    pown_methods!(F, InfiniteError, "If the result is non-finite.");
    recip_methods!(F, InfiniteError, "If the result is non-finite.");
    root_methods!(F, InfiniteError, "If the result is non-finite.");
}
exp_impls!(
    Finite<F: IsFinite>,
    InfiniteError,
    "If the result is non-finite."
);
impl<F: IsFinite + crate::ops::Trig> Finite<F> {
    sin_cos_methods!(F); // sin and cos always succeed for finite values.
    tan_methods!(F, InfiniteError, "If the result is non-finite.");
    asin_acos_methods!(
        F,
        InfiniteError,
        "If the result is non-finite (caused if the magnitude of the input exceeds 1)."
    );
    atan_methods!(F); // atan always succeeds for finite values.
    atan2_methods!(F, InfiniteError, "If the result is non-finite.");
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
        assert_eq!(finite!(2.0f32).recip(), finite!(0.5));
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
