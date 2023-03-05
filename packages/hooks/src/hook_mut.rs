#[derive(Debug, Default)]
pub struct Mut<T>(pub T);

impl<T> Unpin for Mut<T> {}

crate::utils::impl_hook![
    const _: Mut<T> = Generics![T];

    #[inline]
    fn poll_next_update(self) {
        false.into()
    }
    #[inline]
    fn use_hook(self) -> &'hook mut T {
        &mut self.get_mut().0
    }
];

#[inline]
pub fn use_mut_default<T: Default>() -> Mut<T> {
    Default::default()
}
