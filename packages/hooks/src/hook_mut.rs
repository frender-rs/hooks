#[derive(Debug, Default)]
pub struct Mut<T>(pub T);

impl<T> Unpin for Mut<T> {}

crate::utils::impl_hook! {
    impl[T] for Mut<T> {
        #[inline]
        poll_next_update(self) {
            false.into()
        }
        #[inline]
        use_hook(self) -> &'hook mut T {
            &mut self.get_mut().0
        }
    }
}

#[inline]
pub fn use_mut_default<T: Default>() -> Mut<T> {
    Default::default()
}
