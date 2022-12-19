use super::{Specified, Unspecified};

pub trait MaybeBorrow<Borrowed: ?Sized> {
    fn borrow_some(&self) -> Option<&Borrowed>;
}

impl<T: std::borrow::Borrow<B>, B: ?Sized> MaybeBorrow<B> for Option<T> {
    #[inline]
    fn borrow_some(&self) -> Option<&B> {
        self.as_ref().map(T::borrow)
    }
}

impl<T: std::borrow::Borrow<B>, B: ?Sized> MaybeBorrow<B> for Specified<T> {
    #[inline]
    fn borrow_some(&self) -> Option<&B> {
        Some(T::borrow(&self.0))
    }
}

impl<B: ?Sized> MaybeBorrow<B> for Unspecified<B> {
    fn borrow_some(&self) -> Option<&B> {
        None
    }
}

#[macro_export]
macro_rules! impl_borrow_some_for {
    (
        $(
            $(@[$($generics:tt)+])?
            $for_ty:ty
            $(where [$($where_clause:tt)*])?
        ),* $(,)?
    ) => {
        $(
            impl<$($($generics)+,)? ImplMaybeBorrowBorrowed: ?Sized> $crate::MaybeBorrow<ImplMaybeBorrowBorrowed> for $for_ty
            where
                Self: ::core::borrow::Borrow<ImplMaybeBorrowBorrowed>,
                $($($where_clause)*)?
            {
                #[inline]
                fn borrow_some(&self) -> ::core::option::Option<&ImplMaybeBorrowBorrowed> {
                    ::core::option::Option::Some(<Self as ::core::borrow::Borrow<ImplMaybeBorrowBorrowed>>::borrow(self))
                }
            }
        )*
    };
}

impl_borrow_some_for! {
    str,
    String,
    @[T: ?Sized] &T,
    @[T] Vec<T>,
    @[T] [T],
    @[T, const N: usize] [T; N],
    @[T: ToOwned] std::borrow::Cow<'_, T>,
    @[T: ?Sized] Box<T>,
    @[T: ?Sized] std::rc::Rc<T>,
    @[T: ?Sized] std::sync::Arc<T>,
}
