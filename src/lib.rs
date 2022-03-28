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

pub mod ops;

mod bits;
pub use bits::ToOrd;

mod check;

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
