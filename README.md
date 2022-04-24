# real_float

## Intro
This crate is for dealing checked floating point numbers.
It exports three types: [`Real`], [`Finite`], and [`NonNeg`]. `Real` is checked at runtime
to ensure that it is never `NaN`, while `Finite` adds the additional constraint
that it can never be infinite, and `NonNeg` requires that it be positive.

For the sake of brevity, we will only discuss `Real`, but understand that
everything applies to `Finite` and `NonNeg` as well.

## Checking behavior
A `NaN`-check is inserted in every single operation and method. If a `NaN`
ever surfaces, it will result in a runtime panic.

Note that these checks will normally only be present in debug builds.
This is consistent with Rust's philosphy for integer overflowing: panic
in debug mode, and allow fast-but-likely-incorrect bevhavior in release mode.
If you want these checks to be present no matter what, enable the `strict` feature.

## Fallible API
The types in this crate also support fallible APIs for any operation that would
otherwise panic. These are the `try_*` methods defined on `Real` and `Finite`,
and they will perform `NaN` checks whether or not debug mode is enabled.

## Comparison with similar crates
TODO

License: MIT OR Apache-2.0
