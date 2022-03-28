pub trait Round: Sized + Copy {
    #[must_use]
    fn floor(self) -> Self;
    #[must_use]
    fn ceil(self) -> Self;
    #[must_use]
    fn round(self) -> Self;
    #[must_use]
    fn trunc(self) -> Self;
    #[must_use]
    fn fract(self) -> Self;
}

pub trait Signed: Sized + Copy {
    #[must_use]
    fn abs(self) -> Self;
    #[must_use]
    fn signum(self) -> Self;
    #[must_use]
    fn is_sign_positive(self) -> bool;
    #[must_use]
    fn is_sign_negative(self) -> bool;
}

/// Trait for raising a floating point number to a power, and inverse operations.
pub trait Pow: Sized + Copy {
    #[must_use]
    fn powf(self, n: Self) -> Self;
    #[must_use]
    fn powi(self, n: i32) -> Self;
    #[must_use]
    fn recip(self) -> Self;

    #[must_use]
    fn sqrt(self) -> Self;
    #[must_use]
    fn cbrt(self) -> Self;
    #[must_use]
    fn hypot(self, other: Self) -> Self;
}

/// Trait for perfoming exponential operations and their inverses with floating point numbers.
pub trait Exp: Sized + Copy {
    #[must_use]
    fn exp(self) -> Self;
    #[must_use]
    fn exp2(self) -> Self;
    #[must_use]
    fn exp_m1(self) -> Self;

    #[must_use]
    fn log(self, base: Self) -> Self;
    #[must_use]
    fn ln(self) -> Self;
    #[must_use]
    fn log2(self) -> Self;
    #[must_use]
    fn log10(self) -> Self;
    #[must_use]
    fn ln_1p(self) -> Self;
}

/// Trait for computing trigonometric functions and their inverses.
pub trait Trig: Sized + Copy {
    #[must_use]
    fn sin(self) -> Self;
    #[must_use]
    fn cos(self) -> Self;
    #[must_use]
    fn sin_cos(self) -> (Self, Self);
    #[must_use]
    fn tan(self) -> Self;

    #[must_use]
    fn asin(self) -> Self;
    #[must_use]
    fn acos(self) -> Self;
    #[must_use]
    fn atan(self) -> Self;
    #[must_use]
    fn atan2(self, _: Self) -> Self;
}

macro_rules! impl_ops {
    ($f: ty) => {
        impl crate::IsNan for $f {
            #[inline]
            fn is_nan(self) -> bool {
                <$f>::is_nan(self)
            }
        }
        impl crate::IsFinite for $f {
            #[inline]
            fn is_finite(self) -> bool {
                <$f>::is_finite(self)
            }
        }
        impl Round for $f {
            #[inline]
            fn floor(self) -> $f {
                <$f>::floor(self)
            }
            #[inline]
            fn ceil(self) -> $f {
                <$f>::ceil(self)
            }
            #[inline]
            fn round(self) -> $f {
                <$f>::round(self)
            }
            #[inline]
            fn trunc(self) -> $f {
                <$f>::trunc(self)
            }
            #[inline]
            fn fract(self) -> $f {
                <$f>::fract(self)
            }
        }
        impl Signed for $f {
            #[inline]
            fn abs(self) -> $f {
                <$f>::abs(self)
            }
            #[inline]
            fn signum(self) -> $f {
                <$f>::signum(self)
            }
            #[inline]
            fn is_sign_positive(self) -> bool {
                <$f>::is_sign_positive(self)
            }
            #[inline]
            fn is_sign_negative(self) -> bool {
                <$f>::is_sign_negative(self)
            }
        }
        impl Pow for $f {
            #[inline]
            fn powf(self, n: $f) -> $f {
                <$f>::powf(self, n)
            }
            #[inline]
            fn powi(self, n: i32) -> $f {
                <$f>::powi(self, n)
            }
            #[inline]
            fn recip(self) -> $f {
                <$f>::recip(self)
            }

            #[inline]
            fn sqrt(self) -> $f {
                <$f>::sqrt(self)
            }
            #[inline]
            fn cbrt(self) -> $f {
                <$f>::cbrt(self)
            }
            #[inline]
            fn hypot(self, other: Self) -> $f {
                <$f>::hypot(self, other)
            }
        }
        impl Exp for $f {
            #[inline]
            fn exp(self) -> $f {
                <$f>::exp(self)
            }
            #[inline]
            fn exp2(self) -> $f {
                <$f>::exp2(self)
            }
            #[inline]
            fn exp_m1(self) -> $f {
                <$f>::exp_m1(self)
            }

            #[inline]
            fn log(self, b: $f) -> $f {
                <$f>::log(self, b)
            }
            #[inline]
            fn ln(self) -> $f {
                <$f>::ln(self)
            }
            #[inline]
            fn log2(self) -> $f {
                <$f>::log2(self)
            }
            #[inline]
            fn log10(self) -> $f {
                <$f>::log10(self)
            }
            #[inline]
            fn ln_1p(self) -> $f {
                <$f>::ln_1p(self)
            }
        }
        impl Trig for $f {
            #[inline]
            fn sin(self) -> $f {
                <$f>::sin(self)
            }
            #[inline]
            fn cos(self) -> $f {
                <$f>::cos(self)
            }
            #[inline]
            fn sin_cos(self) -> ($f, $f) {
                <$f>::sin_cos(self)
            }
            #[inline]
            fn tan(self) -> $f {
                <$f>::tan(self)
            }

            #[inline]
            fn asin(self) -> $f {
                <$f>::asin(self)
            }
            #[inline]
            fn acos(self) -> $f {
                <$f>::acos(self)
            }
            #[inline]
            fn atan(self) -> $f {
                <$f>::atan(self)
            }
            #[inline]
            fn atan2(self, x: Self) -> $f {
                <$f>::atan2(self, x)
            }
        }
    };
    ($($f: ty),*) => {
        $(impl_ops!($f);)*
    }
}

#[cfg(not(feature = "num_traits"))]
impl_ops!(f32, f64);

#[cfg(feature = "num_traits")]
mod impl_num_traits {
    use super::*;
    use num_traits::Float;

    impl<F: Float> crate::IsNan for F {
        #[inline]
        fn is_nan(self) -> bool {
            <F as Float>::is_nan(self)
        }
    }
    impl<F: Float> crate::IsFinite for F {
        #[inline]
        fn is_finite(self) -> bool {
            <F as Float>::is_finite(self)
        }
    }
    impl<F: Float> Round for F {
        #[inline]
        fn floor(self) -> Self {
            <F as Float>::floor(self);
        }
        #[inline]
        fn ceil(self) -> Self {
            <F as Float>::ceil(self);
        }
        #[inline]
        fn round(self) -> Self {
            <F as Float>::round(self);
        }
        #[inline]
        fn trunc(self) -> Self {
            <F as Float>::trunc(self);
        }
        #[inline]
        fn fract(self) -> Self {
            <F as Float>::fract(self);
        }
    }
    impl Signed for F {
        #[inline]
        fn abs(self) -> Self {
            <F as Float>::abs(self)
        }
        #[inline]
        fn signum(self) -> Self {
            <F as Float>::signum(self)
        }
        #[inline]
        fn is_sign_positive(self) -> bool {
            <F as Float>::is_sign_positive(self)
        }
        #[inline]
        fn is_sign_negative(self) -> bool {
            <F as Float>::is_sign_negative(self)
        }
    }
    impl<F: Float> Pow for F {
        #[inline]
        fn powf(self, n: Self) -> Self {
            <F as Float>::powf(self, n)
        }
        #[inline]
        fn powi(self, n: i32) -> Self {
            <F as Float>::powi(self, n)
        }
        #[inline]
        fn recip(self) -> Self {
            <F as Float>::recip(self)
        }

        #[inline]
        fn sqrt(self) -> Self {
            <F as Float>::sqrt(self)
        }
        #[inline]
        fn cbrt(self) -> Self {
            <F as Float>::cbrt(self)
        }
        #[inline]
        fn hypot(self, other: Self) {
            <F as Float>::hypot(self, other)
        }
    }
    impl<F: Float> Exp for F {
        #[inline]
        fn exp(self) -> Self {
            <F as Float>::exp(self)
        }
        #[inline]
        fn exp2(self) -> Self {
            <F as Float>::exp2(self)
        }
        #[inline]
        fn exp_m1(self) -> Self {
            <F as Float>::exmp_m1(self)
        }

        #[inline]
        fn log(self, b: Self) -> Self {
            <F as Float>::log(self, b)
        }
        #[inline]
        fn ln(self) -> Self {
            <F as Float>::ln(self)
        }
        #[inline]
        fn log2(self) -> Self {
            <F as Float>::log2(self)
        }
        #[inline]
        fn log10(self) -> Self {
            <F as Float>::log10(self)
        }
        #[inline]
        fn ln_1p(self) -> Self {
            <F as Float>::ln_1p(self)
        }
    }
    impl<F: Float> Trig for F {
        #[inline]
        fn sin(self) -> F {
            <F as Float>::sin(self)
        }
        #[inline]
        fn cos(self) -> F {
            <F as Float>::cos(self)
        }
        #[inline]
        fn sin_cos(self) -> (F, F) {
            <F as Float>::sin_cos(self)
        }
        #[inline]
        fn tan(self) -> F {
            <F as Float>::tan(self)
        }

        #[inline]
        fn asin(self) -> F {
            <F as Float>::asin(self)
        }
        #[inline]
        fn acos(self) -> F {
            <F as Float>::acos(self)
        }
        #[inline]
        fn atan(self) -> F {
            <F as Float>::atan(self)
        }
        #[inline]
        fn atan2(self, x: Self) -> F {
            <F as Float>::atan2(self, x)
        }
    }
}
