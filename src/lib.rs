//! # Intro
//! This crate is for dealing checked floating point numbers.
//! It exports three types: [`Real`], [`Finite`], and [`NonNeg`]. `Real` is checked at runtime
//! to ensure that it is never `NaN`, while `Finite` adds the additional constraint
//! that it can never be infinite, and `NonNeg` requires that it have a postive sign.  
//!
//! For the sake of brevity, we will only discuss `Real`, but understand that
//! everything applies to `Finite` and `NonNeg` as well.  
//!
//! # Checking behavior
//! A `NaN`-check is inserted in every single operation and method. If a `NaN`
//! ever surfaces, it will result in a runtime panic.  
//!
//! Note that these checks will normally only be present in debug builds.
//! This is consistent with Rust's philosphy for integer overflowing: panic
//! in debug mode, and allow fast-but-likely-incorrect bevhavior in release mode.  
//! If you want these checks to be present no matter what, enable the `strict` feature.
//!
//! # Fallible API
//! The types in this crate also support fallible APIs for any operation that would
//! otherwise panic. These are the `try_*` methods defined on `Real`, `Finite`, etc.,
//! and they will perform `NaN` checks whether or not debug mode is enabled.
//!
//! # Comparison with similar crates
//! TODO

#![warn(clippy::pedantic)]

/// Unwraps the inner value of a checked floating point number.
///
/// This trait is required instead of using std traits because of coherence rules.
#[doc(hidden)]
pub trait IntoInner<F>: Sized {
    fn into_inner(self) -> F;
}
impl<F> IntoInner<F> for F {
    #[inline]
    fn into_inner(self) -> F {
        self
    }
}

/// Module containinig traits that define required operations for floating point numbers.
/// If the optional `num-traits` feature is enabled, these will automatically be implemented for
/// any type implementing `num_traits::Float`.
pub mod ops;

mod bits;
pub use bits::ToOrd;

/// whether or not to panic on an invalid value.
#[doc(hidden)]
pub const STRICT: bool = cfg!(any(debug_assertions, feature = "strict"));

#[macro_use]
mod macros;

#[cfg(test)]
macro_rules! assert_epsilon {
    ($l: expr, $r: expr, $ep: expr) => {{
        let l = $l;
        let r = $r;
        if (l - r).abs() <= $ep {
            // OK
        } else {
            panic!("assertion failed: `left ≈ right`:\nleft: `{l:?}`\nright:`{r:?}`");
        }
    }};
    ($l: expr, $r: expr) => {
        assert_epsilon!($l, $r, f32::EPSILON);
    };
}
#[cfg(test)]
macro_rules! assert_err {
    ($e: expr) => {
        assert!(($e).is_err())
    };
}

mod real;
pub use real::{IsNan, NanError, Real};

mod finite;
pub use finite::{Finite, InfiniteError, IsFinite};

mod non_neg;
pub use non_neg::{IsNegative, NegativeError, NonNeg};

#[cfg(feature = "num-traits")]
pub mod num;

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
