// wasm32 proptest cannot be compiled at the same time as non-wasm32 proptest, so disable tests that
// use proptest completely for wasm32
//
// See https://github.com/rust-lang/cargo/issues/4866
#[cfg(all(not(target_arch = "wasm32"), test))]
mod test;

use liblumen_alloc::erts::term::prelude::Term;

use lumen_runtime_macros::native_implemented_function;

/// `>=/2` infix operator.  Floats and integers are converted.
#[native_implemented_function(>=/2)]
pub fn native(left: Term, right: Term) -> Term {
    left.ge(&right).into()
}
