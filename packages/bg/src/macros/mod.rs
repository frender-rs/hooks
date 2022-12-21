#[macro_export]
macro_rules! __impl_base_expr {
    ([][$($path:tt)*]) => {
        $($path)* ()
    };
    ([$base:expr][$($path:tt)*]) => {
        $($path)* ::Building($base)
    };
}

// #[cfg(all(test, debug_assertions))]
// mod build_macro;
// #[cfg(all(test, debug_assertions))]
// pub use build_macro::*;

// #[cfg(not(all(test, debug_assertions)))]
mod build_macro_simple;
// #[cfg(not(all(test, debug_assertions)))]
pub use build_macro_simple::*;

mod builder_macro;
pub use builder_macro::*;
