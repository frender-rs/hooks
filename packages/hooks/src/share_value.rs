/// Common behaviors of types that share a value with inner mutability.
pub trait ShareValue<T> {
    fn is_shared(&self) -> bool;

    #[inline]
    fn get(&self) -> T
    where
        T: Copy,
    {
        self.map(|v| *v)
    }

    #[inline]
    fn get_cloned(&self) -> T
    where
        T: Clone,
    {
        self.map(T::clone)
    }

    #[inline]
    fn set(&self, new_value: T) {
        self.replace(new_value);
    }

    /// The old value is returned.
    #[inline]
    fn replace(&self, new_value: T) -> T {
        self.replace_mut(|_| new_value)
    }
    /// The old value is returned.
    #[inline]
    fn replace_mut<F: FnOnce(&mut T) -> T>(&self, f: F) -> T {
        self.map_mut(|old| {
            let new_value = f(old);
            std::mem::replace(old, new_value)
        })
    }

    /// The old value is returned.
    #[inline]
    fn replace_with<F: FnOnce(&T) -> T>(&self, f: F) -> T {
        self.replace_mut(move |v| f(v))
    }

    fn map<R>(&self, f: impl FnOnce(&T) -> R) -> R;
    fn map_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> R;
}
