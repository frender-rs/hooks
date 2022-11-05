pub trait ShareValue<T> {
    fn is_shared(&self) -> bool;

    fn get(&self) -> T
    where
        T: Copy;

    fn get_cloned(&self) -> T
    where
        T: Clone;

    #[inline]
    fn set(&self, new_value: T) {
        self.replace(new_value);
    }

    /// The old value is returned.
    fn replace(&self, new_value: T) -> T;
    /// The old value is returned.
    fn replace_with<F: FnOnce(&mut T) -> T>(&self, f: F) -> T;

    /// The old value is returned.
    #[inline]
    fn replace_from_ref<F: FnOnce(&T) -> T>(&self, f: F) -> T {
        self.replace_with(move |v| f(v))
    }

    fn map<R>(&self, f: impl FnOnce(&T) -> R) -> R;
    fn map_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> R;
}
