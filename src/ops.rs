/// Trait for a floating point number that can be checked for NaN (not-a-number).
pub trait IsNan: Sized + Copy {
    fn is_nan(self) -> bool;
}

/// Trait for raising a floating point number to a power.
pub trait Pow: Sized + Copy {
    #[must_use]
    fn powf(self, n: Self) -> Self;
    #[must_use]
    fn powi(self, n: i32) -> Self;
}

/// Trait for taking roots of a floating point number.
pub trait Root: Sized + Copy {
    #[must_use]
    fn sqrt(self) -> Self;
    #[must_use]
    fn cbrt(self) -> Self;
}

/// Trait for finding logarithms of a floating point number.
pub trait Log: Sized + Copy {
    #[must_use]
    fn log(self, base: Self) -> Self;
    #[must_use]
    fn ln(self) -> Self;
    #[must_use]
    fn log2(self) -> Self;
    #[must_use]
    fn log10(self) -> Self;
}

macro_rules! impl_ops {
    ($f: ty) => {
        impl IsNan for $f {
            #[inline]
            fn is_nan(self) -> bool {
                <$f>::is_nan(self)
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
        }
        impl Root for $f {
            #[inline]
            fn sqrt(self) -> $f {
                <$f>::sqrt(self)
            }
            #[inline]
            fn cbrt(self) -> $f {
                <$f>::cbrt(self)
            }
        }
        impl Log for $f {
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
        }
    };
    ($($f: ty),*) => {
        $(impl_ops!($f);)*
    }
}
impl_ops!(f32, f64);
