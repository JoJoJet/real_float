use std::cmp;

/// A trait that converts a floating point number into something implementing total ordering.
///
/// The behavior of this trait is unspecified for NaN, and may result in a logic error.
pub trait ToOrd: Sized + Copy {
    /// A type implementing total ordering, which a float can be trivially converted to.  
    ///
    /// The exact value of this associated type should be considered opaque, and is subject
    /// to change in minor releases.
    type Ord: cmp::Ord;
    /// Trivially converts this floating point number into one that implements total ordering.
    /// When implementing this, ensure that `-0.0 == 0.0`.
    ///
    /// Behavior is usnpecified for NaN and may result in a logic error.
    fn to_ord(self) -> Self::Ord;
    /// Checks two floating point numbers for total equality, ensuring that `-0.0 == 0.0`.
    ///
    /// This has a default implementation based on [`to_ord`](ToOrd::to_ord).
    /// Only override this if you can provide a more performant implementation (benchmark!).
    #[inline]
    fn total_eq(self, rhs: Self) -> bool {
        self.to_ord() == rhs.to_ord()
    }
}

macro_rules! impl_to_ord {
    ($f: ty, $u: ty) => {
        impl ToOrd for $f {
            type Ord = $u;
            #[inline]
            fn to_ord(self) -> Self::Ord {
                const MSB: $u = 1 << (std::mem::size_of::<$f>() * 8 - 1);
                let bits = self.to_bits();

                if bits & MSB == 0 {
                    // if it's positive, flip the most significant bit.
                    bits | MSB
                } else {
                    // Special case: if it's negative zero, pretend that it's postive zero.
                    // This ensures that -0.0 == +0.0
                    if bits << 1 == 0 {
                        // Bencharking shows that marking as cold provides a slight performance boost,
                        // probably because it aids branch prediction.
                        // This would be even better we if we had an `unlikely` intrinsic, as the special
                        // case could remain inline.
                        #[cold]
                        fn zero() -> $u {
                            MSB // this is the result of flipping the most significant bit of +0.0
                        }

                        zero()
                    }
                    // If it's any other negative number, flip every bit.
                    else {
                        !bits
                    }
                }
            }
            #[inline]
            fn total_eq(self, rhs: Self) -> bool {
                let a = self.to_bits();
                let b = rhs.to_bits();

                // Disregard the sign bit when comparing zeros.
                if a << 1 == 0 {
                    #[cold]
                    fn zero(b: $u) -> bool {
                        b << 1 == 0
                    }

                    zero(b)
                }
                // Compare any other numbers directly.
                else {
                    a == b
                }
            }
        }
    };
}
impl_to_ord!(f32, u32);
impl_to_ord!(f64, u64);
