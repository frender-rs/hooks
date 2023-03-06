use std::marker::PhantomData;

#[derive(Debug, Default)]
pub struct Mut<T>(pub T);

impl<T> Unpin for Mut<T> {}

hooks_core::impl_hook![
    type For<T> = Mut<T>;

    fn unmount() {}

    #[inline]
    fn poll_next_update(self) {
        std::task::Poll::Ready(false)
    }
    #[inline]
    fn use_hook(self) -> &'hook mut T {
        &mut self.get_mut().0
    }

    #[inline]
    fn update_hook(self, _hook: _) {}

    fn h(self, hook: Mut<Option<T>>) {
        hook.get_mut().0.get_or_insert(self.0)
    }
];

pub use Mut as use_mut;

pub struct UseMutWith<T, F: FnOnce() -> T>(pub F);
pub use UseMutWith as use_mut_with;

hooks_core::impl_hook![
    type For<T, F> = UseMutWith<T, F>
        where __![F: FnOnce() -> T]: __;

    #[inline]
    fn into_hook(self) -> Mut<T> {
        Mut(self.0())
    }
    #[inline]
    fn update_hook(self, _hook: _) {}

    fn h(self, hook: Mut<Option<T>>) {
        hook.get_mut().0.get_or_insert_with(self.0)
    }
];

pub struct UseMutDefault<T: Default>(PhantomData<T>);
#[inline(always)]
pub fn use_mut_default<T: Default>() -> UseMutDefault<T> {
    UseMutDefault(PhantomData)
}

hooks_core::impl_hook![
    type For<T: Default> = UseMutDefault<T>;
    #[inline]
    fn into_hook(self) -> Mut<T> {
        Mut::default()
    }
    #[inline]
    fn update_hook(self, _hook: _) {}
    #[inline]
    fn h(self, hook: Mut<T>) {
        hooks_core::Hook::use_hook(hook)
    }
];
