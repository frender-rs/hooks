/// Common behaviors of types that share a value with inner mutability.
pub trait ShareValue {
    type Value;

    fn try_unwrap(self) -> Result<Self::Value, Self>
    where
        Self: Sized;

    fn unwrap_or_get_cloned(self) -> Self::Value
    where
        Self: Sized,
        Self::Value: Clone,
    {
        match self.try_unwrap() {
            Ok(v) => v,
            Err(this) => this.get_cloned(),
        }
    }

    #[inline]
    fn get(&self) -> Self::Value
    where
        Self::Value: Copy,
    {
        self.map(|v| *v)
    }

    #[inline]
    fn get_cloned(&self) -> Self::Value
    where
        Self::Value: Clone,
    {
        self.map(<_>::clone)
    }

    #[inline]
    fn set(&self, new_value: Self::Value) {
        self.replace(new_value);
    }

    /// The old value is returned.
    #[inline]
    fn replace(&self, new_value: Self::Value) -> Self::Value {
        self.replace_mut(|_| new_value)
    }
    /// The old value is returned.
    #[inline]
    fn replace_mut<F: FnOnce(&mut Self::Value) -> Self::Value>(&self, f: F) -> Self::Value {
        self.map_mut(|old| {
            let new_value = f(old);
            std::mem::replace(old, new_value)
        })
    }

    /// The old value is returned.
    #[inline]
    fn replace_with<F: FnOnce(&Self::Value) -> Self::Value>(&self, f: F) -> Self::Value {
        self.replace_mut(move |v| f(v))
    }

    fn map<R>(&self, f: impl FnOnce(&Self::Value) -> R) -> R;
    fn map_mut<R>(&self, f: impl FnOnce(&mut Self::Value) -> R) -> R;

    /// Returns `true` if `self` and `other` are sharing values from the same allocation.
    /// In that case, `self` and `other` are equivalent to each other
    /// because calling the same method on either of them leads to the same result.
    fn equivalent_to(&self, other: &Self) -> bool;
}

impl<S: ?Sized + ShareValue> ShareValue for &S {
    type Value = S::Value;

    fn try_unwrap(self) -> Result<Self::Value, Self>
    where
        Self: Sized,
    {
        Err(self)
    }

    fn map<R>(&self, f: impl FnOnce(&Self::Value) -> R) -> R {
        S::map(self, f)
    }

    fn map_mut<R>(&self, f: impl FnOnce(&mut Self::Value) -> R) -> R {
        S::map_mut(self, f)
    }

    fn equivalent_to(&self, other: &Self) -> bool {
        S::equivalent_to(self, other)
    }

    fn unwrap_or_get_cloned(self) -> Self::Value
    where
        Self: Sized,
        Self::Value: Clone,
    {
        self.get_cloned()
    }

    fn get(&self) -> Self::Value
    where
        Self::Value: Copy,
    {
        S::get(self)
    }

    fn get_cloned(&self) -> Self::Value
    where
        Self::Value: Clone,
    {
        S::get_cloned(self)
    }

    fn set(&self, new_value: Self::Value) {
        S::set(self, new_value)
    }

    fn replace(&self, new_value: Self::Value) -> Self::Value {
        S::replace(self, new_value)
    }

    fn replace_mut<F: FnOnce(&mut Self::Value) -> Self::Value>(&self, f: F) -> Self::Value {
        S::replace_mut(self, f)
    }

    fn replace_with<F: FnOnce(&Self::Value) -> Self::Value>(&self, f: F) -> Self::Value {
        S::replace_with(self, f)
    }
}
