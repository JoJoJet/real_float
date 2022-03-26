/// The error produced when NaN is encountered.
#[derive(Debug, Clone, Copy)]
pub struct NanError;
impl std::fmt::Display for NanError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "encountered NaN unexpectedly")
    }
}

/// whether or not to panic on NaN.
const STRICT: bool = cfg!(any(debug_assertions, feature = "strict"));

#[derive(Debug, Clone, Copy, Default, PartialEq)]
#[repr(transparent)]
pub struct Real<F: IsNan>(F);

impl<F: IsNan> Real<F> {
    #[track_caller]
    pub fn new(val: F) -> Self {
        if STRICT {
            unwrap_display(Self::try_new(val))
        } else {
            Self(val)
        }
    }
    pub fn try_new(val: F) -> Result<Self, NanError> {
        // This is not a TryFrom implementation due to [this issue](https://github.com/rust-lang/rust/issues/50133).
        if val.is_nan() {
            Err(NanError)
        } else {
            Ok(Self(val))
        }
    }
}

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
