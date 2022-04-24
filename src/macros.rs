macro_rules! ctor_impls {
    ($ty:ident <F: $bound:ident>, $msg:literal) => {
        impl<F: $bound> $ty<F> {
            /// Creates a new checked float.
            /// # Panics
            #[doc = $msg]
            /// Note that this fn will *not* panic in release mode, unless the `strict` feature flag is set.
            #[track_caller]
            pub fn new(val: F) -> Self {
                if $crate::STRICT {
                    $crate::unwrap_display(Self::try_new(val))
                } else {
                    Self::new_unchecked(val)
                }
            }
            #[cfg_attr(track_caller, debug_assertions)]
            fn new_unchecked(val: F) -> Self {
                // panic anyway in debug mode b/c why not.
                #[cfg(debug_assertions)]
                let _ = crate::unwrap_display(Self::try_new(val));
                Self(val)
            }
        }
    };
}

macro_rules! eq_impls {
    ($ty:ident <F: $bound:ident>) => {
        impl<F: $bound + $crate::ToOrd, Rhs: $crate::IntoInner<F> + Copy>
            ::core::cmp::PartialEq<Rhs> for $ty<F>
        {
            fn eq(&self, rhs: &Rhs) -> bool {
                // we can ignore the case where `rhs` is NaN since
                // we know that `self` is not NaN.
                let rhs = (*rhs).into_inner();
                self.val().total_eq(rhs)
            }
        }
        impl<F: $bound + $crate::ToOrd> ::core::cmp::Eq for $ty<F> {}
    };
}

macro_rules! ord_impls {
    ($ty:ident <F : $bound:ident>) => {
        impl<F: $bound + $crate::ToOrd, Rhs: $crate::IntoInner<F> + Copy>
            ::core::cmp::PartialOrd<Rhs> for $ty<F>
        {
            fn partial_cmp(&self, rhs: &Rhs) -> Option<::core::cmp::Ordering> {
                let rhs = (*rhs).into_inner();
                let rhs = Self::try_new(rhs).ok()?.val().to_ord();
                let lhs = self.val().to_ord();
                Some(lhs.cmp(&rhs))
            }
        }
        impl<F: $bound + $crate::ToOrd> ::core::cmp::Ord for $ty<F> {
            fn cmp(&self, rhs: &Self) -> ::core::cmp::Ordering {
                let lhs = self.val().to_ord();
                let rhs = rhs.val().to_ord();
                lhs.cmp(&rhs)
            }
        }

        impl<F: $bound + $crate::ToOrd> $ty<F> {
            /// Returns the larger of two floating point values.
            #[must_use]
            pub fn max(self, other: impl IntoInner<F>) -> Self {
                let other = other.into_inner();
                match self.partial_cmp(&other) {
                    Some(::core::cmp::Ordering::Greater) => Self::new_unchecked(other),
                    _ => self,
                }
            }
            /// Returns the smaller of two floating point values.
            #[must_use]
            pub fn min(self, other: impl IntoInner<F>) -> Self {
                let other = other.into_inner();
                match self.partial_cmp(&other) {
                    Some(::core::cmp::Ordering::Less) => Self::new_unchecked(other),
                    _ => self,
                }
            }
        }
    };
}

macro_rules! round_impls {
    ($ty: ident <F : $bound: ident>) => {
        impl<F: $bound + $crate::ops::Round> $ty<F> {
            /// Rounds this floating point number to the previous whole number.
            #[must_use]
            pub fn floor(self) -> Self {
                Self::new_unchecked(self.val().floor())
            }
            /// Rounds this floating point number to the next whole number.
            #[must_use]
            pub fn ceil(self) -> Self {
                Self::new_unchecked(self.val().ceil())
            }
            /// Rounds this floating point number to the nearest whole number.
            #[must_use]
            pub fn round(self) -> Self {
                Self::new_unchecked(self.val().round())
            }
            /// Drops the fractional part of this floating point number.
            #[must_use]
            pub fn trunc(self) -> Self {
                Self::new_unchecked(self.val().trunc())
            }
            /// Returns the fractional part of this floating point number.
            #[must_use]
            pub fn fract(self) -> Self {
                Self::new_unchecked(self.val().fract())
            }
        }
    };
}

macro_rules! signed_impls {
    ($ty: ident <F : $bound: ident>) => {
        impl<F: $bound + $crate::ops::Signed> $ty<F> {
            /// Computes the absolute value of self.
            #[must_use]
            pub fn abs(self) -> Self {
                Self::new_unchecked(self.val().abs())
            }
            /// Returns a number that represents the sign of self.
            /// * `1.0` if the number is positive, `+0.0` or `INFINITY`
            /// * `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
            #[must_use]
            pub fn signum(self) -> Self {
                Self::new_unchecked(self.val().signum())
            }
            /// Returns true if self has a negative sign, including -0.0 and negative infinity.
            #[must_use]
            pub fn is_sign_negative(self) -> bool {
                self.val().is_sign_negative()
            }
            /// Returns true if self has a positive sign, including +0.0 and positive infinity.
            #[must_use]
            pub fn is_sign_positive(self) -> bool {
                self.val().is_sign_positive()
            }
        }
    };
}

macro_rules! sum_impls {
    ($ty: ident <F : $bound: ident>, $err: ty, $msg: literal) => {
        impl<F: $bound> $ty<F> {
            /// Attempts to add two numbers.
            /// # Errors
            #[doc = $msg]
            pub fn try_add(self, rhs: impl $crate::IntoInner<F>) -> Result<Self, $err>
            where
                F: ::core::ops::Add<Output = F>,
            {
                let val = self.val() + rhs.into_inner();
                Self::try_new(val)
            }
            /// Attempts to subtract two numbers.
            /// # Errors
            #[doc = $msg]
            pub fn try_sub(self, rhs: impl $crate::IntoInner<F>) -> Result<Self, $err>
            where
                F: ::core::ops::Sub<Output = F>,
            {
                let val = self.val() - rhs.into_inner();
                Self::try_new(val)
            }
        }

        impl<F: $bound + ::core::ops::Add<Output = F>, Rhs: $crate::IntoInner<F>>
            ::core::ops::Add<Rhs> for $ty<F>
        {
            type Output = Self;
            #[track_caller]
            fn add(self, rhs: Rhs) -> Self {
                let val = self.val() + rhs.into_inner();
                Self::new(val)
            }
        }
        impl<F: $bound + ::core::ops::Sub<Output = F>, Rhs: $crate::IntoInner<F>>
            ::core::ops::Sub<Rhs> for $ty<F>
        {
            type Output = Self;
            #[track_caller]
            fn sub(self, rhs: Rhs) -> Self {
                let val = self.val() - rhs.into_inner();
                Self::new(val)
            }
        }

        impl<F: $bound + ::core::ops::Add<Output = F>, Rhs: $crate::IntoInner<F>>
            ::core::ops::AddAssign<Rhs> for $ty<F>
        {
            #[track_caller]
            fn add_assign(&mut self, rhs: Rhs) {
                *self = *self + rhs.into_inner();
            }
        }
        impl<F: $bound + ::core::ops::Sub<Output = F>, Rhs: $crate::IntoInner<F>>
            ::core::ops::SubAssign<Rhs> for $ty<F>
        {
            #[track_caller]
            fn sub_assign(&mut self, rhs: Rhs) {
                *self = *self - rhs.into_inner();
            }
        }
    };
}
macro_rules! neg_impls {
    ($ty: ident <F : $bound: ident>, $err: ty, $msg: literal) => {
        impl<F: $bound> $ty<F> {
            /// Attempts to negate a number.
            /// # Errors
            #[doc = $msg]
            pub fn try_neg(self) -> Result<Self, $err>
            where
                F: ::core::ops::Neg<Output = F>,
            {
                let val = -self.val();
                Self::try_new(val)
            }
        }
        impl<F: $bound + ::core::ops::Neg<Output = F>> ::core::ops::Neg for $ty<F> {
            type Output = Self;
            #[track_caller]
            fn neg(self) -> Self {
                let val = -self.val();
                Self::new(val)
            }
        }
    };
}

macro_rules! product_impls {
    ($ty: ident <F : $bound: ident>, $err: ty, $msg: literal) => {
        impl<F: $bound> $ty<F> {
            /// Attempts to multiply two numbers.
            /// # Errors
            #[doc = $msg]
            pub fn try_mul(self, rhs: impl $crate::IntoInner<F>) -> Result<Self, $err>
            where
                F: ::core::ops::Mul<Output = F>,
            {
                let val = self.val() * rhs.into_inner();
                Self::try_new(val)
            }
            /// Attempts to divide two numbers.
            /// # Errors
            #[doc = $msg]
            pub fn try_div(self, rhs: impl $crate::IntoInner<F>) -> Result<Self, $err>
            where
                F: ::core::ops::Div<Output = F>,
            {
                let val = self.val() / rhs.into_inner();
                Self::try_new(val)
            }
            /// Attempts to find the remainder of two numbers.
            /// # Errors
            #[doc = $msg]
            pub fn try_rem(self, rhs: impl $crate::IntoInner<F>) -> Result<Self, $err>
            where
                F: ::core::ops::Rem<Output = F>,
            {
                let val = self.val() % rhs.into_inner();
                Self::try_new(val)
            }
        }

        impl<F: $bound + ::core::ops::Mul<Output = F>, Rhs: $crate::IntoInner<F>>
            ::core::ops::Mul<Rhs> for $ty<F>
        {
            type Output = Self;
            #[track_caller]
            fn mul(self, rhs: Rhs) -> Self {
                let val = self.val() * rhs.into_inner();
                Self::new(val)
            }
        }
        impl<F: $bound + ::core::ops::Div<Output = F>, Rhs: $crate::IntoInner<F>>
            ::core::ops::Div<Rhs> for $ty<F>
        {
            type Output = Self;
            #[track_caller]
            fn div(self, rhs: Rhs) -> Self {
                let val = self.val() / rhs.into_inner();
                Self::new(val)
            }
        }
        impl<F: $bound + ::core::ops::Rem<Output = F>, Rhs: $crate::IntoInner<F>>
            ::core::ops::Rem<Rhs> for $ty<F>
        {
            type Output = Self;
            #[track_caller]
            fn rem(self, rhs: Rhs) -> Self {
                let val = self.val() % rhs.into_inner();
                Self::new(val)
            }
        }

        impl<F: $bound + ::core::ops::Mul<Output = F>, Rhs: $crate::IntoInner<F>>
            ::core::ops::MulAssign<Rhs> for $ty<F>
        {
            #[track_caller]
            fn mul_assign(&mut self, rhs: Rhs) {
                *self = *self * rhs.into_inner()
            }
        }
        impl<F: $bound + ::core::ops::Div<Output = F>, Rhs: $crate::IntoInner<F>>
            ::core::ops::DivAssign<Rhs> for $ty<F>
        {
            #[track_caller]
            fn div_assign(&mut self, rhs: Rhs) {
                *self = *self / rhs.into_inner()
            }
        }
        impl<F: $bound + ::core::ops::Rem<Output = F>, Rhs: $crate::IntoInner<F>>
            ::core::ops::RemAssign<Rhs> for $ty<F>
        {
            #[track_caller]
            fn rem_assign(&mut self, rhs: Rhs) {
                *self = *self % rhs.into_inner()
            }
        }
    };
}

macro_rules! pow_methods {
    ($f: ident, $err: ty, $msg: literal) => {
        /// Attempts to raise `self` to the power `n`.
        /// # Errors
        #[doc = $msg]
        pub fn try_powf(self, n: impl $crate::IntoInner<F>) -> Result<Self, $err> {
            let val = self.val().powf(n.into_inner());
            Self::try_new(val)
        }
        /// Attempts to raise `self` to the power `n`.
        /// # Errors
        #[doc = $msg]
        pub fn try_powi(self, n: i32) -> Result<Self, $err> {
            let val = self.val().powi(n);
            Self::try_new(val)
        }

        /// Raises `self` to the power `n`.
        /// # Panics
        #[doc = $msg]
        #[track_caller]
        #[must_use]
        pub fn powf(self, n: impl $crate::IntoInner<F>) -> Self {
            let val = self.val().powf(n.into_inner());
            Self::new(val)
        }
        /// Raises `self` to the power `n`.
        /// # Panics
        #[doc = $msg]
        #[track_caller]
        #[must_use]
        pub fn powi(self, n: i32) -> Self {
            let val = self.val().powi(n);
            Self::new(val)
        }
    };
}
macro_rules! recip_methods {
    ($f: ident, $err: ty, $msg: literal) => {
        /// Attempts to compute the reciprocal (`1/x`) of `self`.
        /// # Errors
        #[doc = $msg]
        pub fn try_recip(self) -> Result<Self, $err> {
            let val = self.val().recip();
            Self::try_new(val)
        }
        /// Computes the reciprocal (`1/x`) of `self`.
        /// # Panics
        #[doc = $msg]
        #[track_caller]
        #[must_use]
        pub fn recip(self) -> Self {
            let val = self.val().recip();
            Self::new(val)
        }
    };
    ($f:ident) => {
        /// Computes the reciprocal (`1/x`) of `self`.
        #[must_use]
        pub fn recip(self) -> Self {
            // this macro arm assumes that `recip` always succeeds.
            Self::new_unchecked(self.val().recip())
        }
    };
}
macro_rules! sqrt_methods {
    ($f:ident, $err:ty, $msg:literal) => {
        /// Attempts to find the square root of a number.
        /// # Errors
        #[doc = $msg]
        pub fn try_sqrt(self) -> Result<Self, $err> {
            let val = self.val().sqrt();
            Self::try_new(val)
        }
        /// Computes the square root of a number.
        /// # Panics
        #[doc = $msg]
        #[track_caller]
        #[must_use]
        pub fn sqrt(self) -> Self {
            let val = self.val().sqrt();
            Self::new(val)
        }
    };
    ($f:ident) => {
        /// Computes the square root of a number.
        #[must_use]
        pub fn sqrt(self) -> Self {
            let val = self.val().sqrt();
            Self::new_unchecked(val)
        }
    };
}
macro_rules! cbrt_methods {
    ($f:ident) => {
        /// Computes the cube root of a number.
        #[must_use]
        pub fn cbrt(self) -> Self {
            // cube root is defined for any real value
            let val = self.val().cbrt();
            Self::new_unchecked(val)
        }
    };
}
macro_rules! hypot_methods {
    ($f:ident, $err: ty, $msg: literal) => {
        /// Attempts to calculate the length of the hypotenuse of a right-angle triangle given legs of length `x` and `y`.
        ///
        /// Equivalent to `sqrt(x^2 + y^2)`.
        /// # Errors
        #[doc = $msg]
        pub fn try_hypot(self, other: impl $crate::IntoInner<F>) -> Result<Self, $err> {
            let val = self.val().hypot(other.into_inner());
            Self::try_new(val)
        }
        /// Calculates the length of the hypotenuse of a right-angle triangle given legs of length `x` and `y`.
        /// # Panics
        #[doc = $msg]
        #[track_caller]
        #[must_use]
        pub fn hypot(self, other: impl $crate::IntoInner<F>) -> Self {
            let val = self.val().hypot(other.into_inner());
            Self::new(val)
        }
    };
}

macro_rules! exp_methodss {
    ($f:ident, $err:ty, $msg:literal) => {
        /// Attempts to find `e^(self)`, the exponential function.
        /// # Errors
        #[doc = $msg]
        pub fn try_exp(self) -> Result<Self, $err> {
            let val = self.val().exp();
            Self::try_new(val)
        }
        /// Attempts to find `2^(self)`.
        /// # Errors
        #[doc = $msg]
        pub fn try_exp2(self) -> Result<Self, $err> {
            let val = self.val().exp2();
            Self::try_new(val)
        }
        /// Attempts to find `e^(self) - 1` in a way that is accurate even if the number is close to zero.
        /// # Errors
        #[doc = $msg]
        pub fn try_exp_m1(self) -> Result<Self, $err> {
            let val = self.val().exp_m1();
            Self::try_new(val)
        }

        /// Computes `e^(self)`, the exponential function.
        /// # Panics
        #[doc = $msg]
        #[track_caller]
        #[must_use]
        pub fn exp(self) -> Self {
            let val = self.val().exp();
            Self::new(val)
        }
        /// Computes `2^(self)`.
        /// # Panics
        #[doc = $msg]
        #[track_caller]
        #[must_use]
        pub fn exp2(self) -> Self {
            let val = self.val().exp2();
            Self::new(val)
        }
        /// Computes `e^(self) - 1` more accurately than performing the operations separately.
        /// # Panics
        #[doc = $msg]
        #[track_caller]
        #[must_use]
        pub fn exp_m1(self) -> Self {
            let val = self.val().exp_m1();
            Self::new(val)
        }
    };
}
macro_rules! log_methods {
    ($f:ident, $err:ty, $msg:literal) => {
        /// Attempts to find the log base `b` of `self`.
        /// # Errors
        #[doc = $msg]
        pub fn try_log(self, b: impl IntoInner<F>) -> Result<Self, $err> {
            let val = self.val().log(b.into_inner());
            Self::try_new(val)
        }
        /// Attempts to find the natural log (base e) of `self`.
        /// # Errors
        #[doc = $msg]
        pub fn try_ln(self) -> Result<Self, $err> {
            let val = self.val().ln();
            Self::try_new(val)
        }
        /// Attempts to find the log base 2 of `self`.
        /// # Errors
        #[doc = $msg]
        pub fn try_log2(self) -> Result<Self, $err> {
            let val = self.val().log2();
            Self::try_new(val)
        }
        /// Attempts to find the log base 10 of `self`.
        /// # Errors
        #[doc = $msg]
        pub fn try_log10(self) -> Result<Self, $err> {
            let val = self.val().log10();
            Self::try_new(val)
        }
        /// Attempts to find `ln(1+n)` (natural logarithm) more accurately than if the operations were performed separately.
        /// # Errors
        #[doc = $msg]
        pub fn try_ln_1p(self) -> Result<Self, $err> {
            let val = self.val().ln_1p();
            Self::try_new(val)
        }

        /// Computes the log base `b` of `self`.
        /// # Panics
        #[doc = $msg]
        #[track_caller]
        #[must_use]
        pub fn log(self, b: impl IntoInner<F>) -> Self {
            let val = self.val().log(b.into_inner());
            Self::new(val)
        }
        /// Computes the natural log (base e) of `self`.
        /// # Panics
        #[doc = $msg]
        #[track_caller]
        #[must_use]
        pub fn ln(self) -> Self {
            let val = self.val().ln();
            Self::new(val)
        }
        /// Computes the log base 2 of `self`.
        /// # Panics
        #[doc = $msg]
        #[track_caller]
        #[must_use]
        pub fn log2(self) -> Self {
            let val = self.val().log2();
            Self::new(val)
        }
        /// Computes the log base 10 of `self`.
        /// # Panics
        #[doc = $msg]
        #[track_caller]
        #[must_use]
        pub fn log10(self) -> Self {
            let val = self.val().log10();
            Self::new(val)
        }
        /// Computes `ln(1+n)` (natural logarithm) more accurately than if the operations were performed separately.
        /// # Panics
        #[doc = $msg]
        #[track_caller]
        #[must_use]
        pub fn ln_1p(self) -> Self {
            let val = self.val().ln_1p();
            Self::new(val)
        }
    };
}
macro_rules! exp_impls {
    ($ty:ident <F : $bound:ident>, $err:ty, $msg:literal) => {
        impl<F: $bound + $crate::ops::Exp> $ty<F> {
            exp_methodss!(F, $err, $msg);
            log_methods!(F, $err, $msg);
        }
    };
}

macro_rules! sin_cos_methods {
    ($f:ident, $err:ty, $msg:literal) => {
        /// Attempts to compute the sine of a number (in radians).
        /// # Errors
        #[doc = $msg]
        pub fn try_sin(self) -> Result<Self, $err> {
            let val = self.val().sin();
            Self::try_new(val)
        }
        /// Attempts to compute the cosine of a number (in radians).
        /// # Errors
        #[doc = $msg]
        pub fn try_cos(self) -> Result<Self, $err> {
            let val = self.val().cos();
            Self::try_new(val)
        }
        /// Attempts to compute both the sine and cosine of a number simultaneously (in radians).
        /// # Errors
        #[doc = $msg]
        pub fn try_sin_cos(self) -> Result<(Self, Self), $err> {
            let (s, c) = self.val().sin_cos();
            Ok((Self::try_new(s)?, Self::try_new(c)?))
        }

        /// Computes the sine of a number.
        /// # Panics
        #[doc = $msg]
        #[track_caller]
        #[must_use]
        pub fn sin(self) -> Self {
            let val = self.val().sin();
            Self::new(val)
        }
        /// Computes the cosine of a number.
        /// # Panics
        #[doc = $msg]
        #[track_caller]
        #[must_use]
        pub fn cos(self) -> Self {
            let val = self.val().cos();
            Self::new(val)
        }
        /// Computes the sine and cosine of a number simultaneously.
        /// # Panics
        #[doc = $msg]
        #[track_caller]
        #[must_use]
        pub fn sin_cos(self) -> (Self, Self) {
            let (s, c) = self.val().sin_cos();
            (Self::new(s), Self::new(c))
        }
    };
    ($f:ident) => {
        /// Computes the sine of a number.
        #[must_use]
        pub fn sin(self) -> Self {
            // this macro arm assumes that sin/cos always succeed
            Self::new_unchecked(self.val().sin())
        }
        /// Computes the cosine of a number.
        #[must_use]
        pub fn cos(self) -> Self {
            Self::new_unchecked(self.val().cos())
        }
        /// Computes the sine and cosine of a number simultaneously.
        #[must_use]
        pub fn sin_cos(self) -> (Self, Self) {
            let (s, c) = self.val().sin_cos();
            (Self::new_unchecked(s), Self::new_unchecked(c))
        }
    };
}
macro_rules! tan_methods {
    ($f:ident, $err:ty, $msg:literal) => {
        /// Attempts to compute the tangent of a number (in radians).
        /// # Errors
        #[doc = $msg]
        pub fn try_tan(self) -> Result<Self, $err> {
            let val = self.val().tan();
            Self::try_new(val)
        }
        /// Computes the tangent of a number.
        /// # Panics
        #[doc = $msg]
        #[track_caller]
        #[must_use]
        pub fn tan(self) -> Self {
            let val = self.val().tan();
            Self::new(val)
        }
    };
}
macro_rules! asin_acos_methods {
    ($f:ident, $err:ty, $msg:literal) => {
        /// Attempts to compute the arcsine of a number (in radians).
        /// # Errors
        #[doc = $msg]
        pub fn try_asin(self) -> Result<Self, $err> {
            let val = self.val().asin();
            Self::try_new(val)
        }
        /// Attempts to compute the arccosine of a number (in radians).
        /// # Errors
        #[doc = $msg]
        pub fn try_acos(self) -> Result<Self, $err> {
            let val = self.val().acos();
            Self::try_new(val)
        }

        /// Computes the arcsine of a number.
        /// # Panics
        #[doc = $msg]
        #[track_caller]
        #[must_use]
        pub fn asin(self) -> Self {
            let val = self.val().asin();
            Self::new(val)
        }
        /// Computes the arccosine of a number.
        /// # Panics
        #[doc = $msg]
        #[track_caller]
        #[must_use]
        pub fn acos(self) -> Self {
            let val = self.val().acos();
            Self::new(val)
        }
    };
}
macro_rules! atan_methods {
    ($f:ident, $err:ty, $msg:literal) => {
        /// Attempts to compute the arccosine of a number (in radians).
        /// # Errors
        #[doc = $msg]
        pub fn try_atan(self) -> Result<Self, $err> {
            let val = self.val().atan();
            Self::try_new(val)
        }
        /// Computes the arctangent of a number.
        /// # Panics
        #[doc = $msg]
        #[track_caller]
        #[must_use]
        pub fn atan(self) -> Self {
            let val = self.val().atan();
            Self::new(val)
        }
    };
    ($f:ident) => {
        /// Computes the arctangent of a number.
        #[must_use]
        pub fn atan(self) -> Self {
            // this macro arm assuems tangent always succeeds
            Self::new_unchecked(self.val().atan())
        }
    };
}
macro_rules! atan2_methods {
    ($f:ident, $err:ty, $msg:literal) => {
        /// Attempts to compute the four quadrant arctangent of self (`y`) and other (`x`) in radians.
        /// # Errors
        #[doc = $msg]
        pub fn try_atan2(self, other: impl IntoInner<F>) -> Result<Self, $err> {
            let val = self.val().atan2(other.into_inner());
            Self::try_new(val)
        }
        /// Computes the four quadrant arctangent of self (`y`) and other (`x`) in radians.
        /// # Panics
        #[doc = $msg]
        #[track_caller]
        #[must_use]
        pub fn atan2(self, other: impl IntoInner<F>) -> Self {
            let val = self.val().atan2(other.into_inner());
            Self::new(val)
        }
    };
}
