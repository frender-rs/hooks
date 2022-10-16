mod ext;
mod hook;
// mod run_hook;

pub use ext::*;
pub use hook::*;
// pub use run_hook::*;

pub(crate) mod sealed {
    pub trait HookLifetimeBounds<'hook, This: ?Sized> {}
    impl<'hook, T: super::HookBounds<Bounds = B> + ?Sized, B: ?Sized> HookLifetimeBounds<'hook, T>
        for &'hook B
    {
    }

    // pub trait RunHookLifetimeBounds<'hook, This: ?Sized, H: ?Sized> {}
    // impl<'hook, H: ?Sized, T: super::RunHookBounds<H, Bounds = B> + ?Sized, B: ?Sized>
    //     RunHookLifetimeBounds<'hook, T, H> for &'hook B
    // {
    // }
}
