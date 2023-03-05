pub trait Identity {
    type This: ?Sized;

    fn identity(this: Self) -> Self::This
    where
        Self: Sized,
        Self::This: Sized;
}

impl<T: ?Sized> Identity for T {
    type This = T;

    #[inline(always)]
    fn identity(this: Self) -> Self::This
    where
        Self: Sized,
        Self::This: Sized,
    {
        this
    }
}
