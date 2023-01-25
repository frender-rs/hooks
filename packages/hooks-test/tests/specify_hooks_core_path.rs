pub use hooks::core as hc;
use hooks::hook;

#[hook]
pub fn use_default() {}

#[hook(hooks_core_path = "hc")]
pub fn use_ident() {}

#[hook(hooks_core_path = "self::hc")]
pub fn use_path() {}

#[hook(hooks_core_path(hc))]
pub fn use_paren_ident() {}

#[hook(hooks_core_path(self::hc))]
pub fn use_paren_path() {}
