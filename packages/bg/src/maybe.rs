use super::{Specified, Unspecified};

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

impl<T> Maybe<T> for Specified<T> {
    #[inline]
    fn some(self) -> Option<T>
    where
        Self: Sized,
        T: Sized,
    {
        Some(self.0)
    }

    #[inline]
    fn as_some(&self) -> Option<&T> {
        Some(&self.0)
    }

    #[inline]
    fn as_mut_some(&mut self) -> Option<&mut T> {
        Some(&mut self.0)
    }
}
