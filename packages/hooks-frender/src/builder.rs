pub trait WrapData<Data> {
    type Wrapped;
    fn wrap_data(props: Data) -> Self::Wrapped;
}

pub trait UnwrapData {
    type Data;
    fn unwrap_data(self) -> Self::Data;
    fn unwrap_as_data(&self) -> &Self::Data;
    fn unwrap_as_mut_data(&mut self) -> &mut Self::Data;
}

pub trait Maybe<T: ?Sized> {
    fn some(self) -> Option<T>
    where
        Self: Sized,
        T: Sized;
    fn as_some(&self) -> Option<&T>;
    fn as_mut_some(&mut self) -> Option<&mut T>;
}

impl<T: ?Sized> Maybe<T> for T {
    #[inline]
    fn some(self) -> Option<T>
    where
        Self: Sized,
        T: Sized,
    {
        Some(self)
    }

    #[inline]
    fn as_some(&self) -> Option<&T> {
        Some(self)
    }

    #[inline]
    fn as_mut_some(&mut self) -> Option<&mut T> {
        Some(self)
    }
}

impl<T> Maybe<T> for Option<T> {
    #[inline]
    fn some(self) -> Option<T>
    where
        Self: Sized,
        T: Sized,
    {
        self
    }

    #[inline]
    fn as_some(&self) -> Option<&T> {
        self.as_ref()
    }

    #[inline]
    fn as_mut_some(&mut self) -> Option<&mut T> {
        self.as_mut()
    }
}

impl<T> Maybe<T> for Unspecified<T> {
    #[inline]
    fn some(self) -> Option<T>
    where
        Self: Sized,
        T: Sized,
    {
        None
    }

    #[inline]
    fn as_some(&self) -> Option<&T> {
        None
    }

    #[inline]
    fn as_mut_some(&mut self) -> Option<&mut T> {
        None
    }
}

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

#[cfg(test)]
mod tests {
    use super::Unspecified;

    #[test]
    fn size() {
        let _: Unspecified<usize> = Unspecified;
        assert_eq!(std::mem::size_of::<Unspecified::<usize>>(), 0);
    }
}
