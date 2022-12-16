mod maybe;
mod maybe_borrow;

pub use maybe::*;
pub use maybe_borrow::*;

/// Instead of defining a new struct,
///
/// ```
/// struct Unspecified<T: ?Sized> {
///     _phantom: std::marker::PhantomData<T>
/// }
/// ```
///
/// We just use [`PhantomData`](std::marker::PhantomData)
/// as [`Unspecified`], so that traits like [`Copy`], [`Default`] and
/// even [`serde::Serialize`] are implemented for this type.
///
/// https://github.com/rust-lang/rust-analyzer/issues/1666
pub use std::marker::PhantomData as Unspecified;

/// Marks a field is unspecified.
pub use std::marker::PhantomData as UnspecifiedField;

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Specified<T>(pub T);

#[cfg(test)]
mod tests {
    use super::Unspecified;

    #[test]
    fn size() {
        let _: Unspecified<usize> = Unspecified;
        assert_eq!(std::mem::size_of::<Unspecified::<usize>>(), 0);
    }
}

pub trait PhantomTypeParam<T: ?Sized> {
    type Out: ?Sized;
}

impl<T: ?Sized, R: ?Sized> PhantomTypeParam<R> for T {
    type Out = R;
}
