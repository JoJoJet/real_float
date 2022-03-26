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

mod ops;
pub use ops::*;

mod bits;
pub use bits::ToOrd;

mod check;

mod real;
pub use real::{NanError, Real};

mod finite;
pub use finite::{Finite, InfiniteError};

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
