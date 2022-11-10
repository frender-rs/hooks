#![deny(clippy::undocumented_unsafe_blocks)]

mod ext;
mod hook;

pub use ext::*;
pub use hook::*;

pub mod fn_hook;
pub mod hook_pair;

pub(crate) mod utils;