// From https://github.com/rust-lang/rust/issues/132558
//#![recursion_limit = "10"]

trait S {
    type Child: S;
}

impl<X: S> S for (X, X) {
    // Recursing "inwards" here using `X::Child` seems to be
    // important. E.g. if I instead define
    //
    //type Child = ((X, X), (X, X));
    //
    // which is just as exponential, but not recursive, then the compiler again
    // gives the correct "recursion depth exceeded" error with the default and
    // small recursion limits.
    type Child = (X::Child, X::Child);
}

impl<X: S> S for Box<X> {
    type Child = X;
}

type Data = Box<(A, A)>;
struct A {}
impl S for A {
    // Manually expanding the `Data::Child` here fixes the bug both for a small
    // recursion limit, and for the default recursion limit, i.e. the compiler
    // gives a "recursion depth exceeded" error message in both cases. This is
    // in contrast to the version above in my report, where expanding the
    // `Data::Child` type fixes the bug for the small recursion limit, but the
    // compiler still hangs for the default recursion limit.
    //
    //type Child = (A, A);
    type Child = <Data as S>::Child;
}

fn uhoh<T: S>() {
    uhoh::<T::Child>();
}

fn main() {
    uhoh::<A>();
}
