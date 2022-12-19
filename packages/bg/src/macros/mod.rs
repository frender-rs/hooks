#[cfg(all(test, debug_assertions))]
mod build_macro;
#[cfg(all(test, debug_assertions))]
pub use build_macro::*;

#[cfg(not(all(test, debug_assertions)))]
mod build_macro_simple;
#[cfg(not(all(test, debug_assertions)))]
pub use build_macro_simple::*;

mod props_macro;
pub use props_macro::*;
