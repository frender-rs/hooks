use crate::Hook;

use super::IterHook;

/// [`Hook`]s that takes no args.
pub trait AsyncIterableHook: Hook<()> {
    #[inline]
    fn into_iter(self) -> IterHook<Self>
    where
        Self: Sized,
    {
        IterHook::new(self)
    }

    #[inline]
    fn iter_mut(&mut self) -> IterHook<&'_ mut Self> {
        IterHook::new(self)
    }
}

impl<H: Hook<()>> AsyncIterableHook for H {}
