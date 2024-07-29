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

/// Proxy the methods that can't be optimized by [`PartialEq`].
macro_rules! proxy_share_value_non_eq {
    (|$_self:ident| -> $ty:ty { $expr:expr }, |$other:ident| $other_expr:expr) => {
        type Value = <$ty as $crate::ShareValue>::Value;

        fn map<R>(&$_self, f: impl FnOnce(&Self::Value) -> R) -> R {
            <$ty>::map($expr, f)
        }

        fn map_mut<R>(&$_self, f: impl FnOnce(&mut Self::Value) -> R) -> R {
            <$ty>::map_mut($expr, f)
        }

        fn equivalent_to(&$_self, $other: &Self) -> bool {
            <$ty>::equivalent_to($expr, $other_expr)
        }

        fn get(&$_self) -> Self::Value
        where
            Self::Value: Copy,
        {
            <$ty>::get($expr)
        }

        fn get_cloned(&$_self) -> Self::Value
        where
            Self::Value: Clone,
        {
            <$ty>::get_cloned($expr)
        }

        fn replace_mut<F: FnOnce(&mut Self::Value) -> Self::Value>(&$_self, f: F) -> Self::Value {
            <$ty>::replace_mut($expr, f)
        }
    };
}

macro_rules! proxy_share_value {
    (|$_self:ident| -> $ty:ty { $expr:expr }, |$other:ident| $other_expr:expr) => {
        $crate::share_value::proxy_share_value_non_eq! { |$_self| -> $ty { $expr }, |$other| $other_expr }

        fn set(&$_self, new_value: Self::Value) {
            <$ty>::set($expr, new_value)
        }

        fn replace(&$_self, new_value: Self::Value) -> Self::Value {
            <$ty>::replace($expr, new_value)
        }

        fn replace_with<F: FnOnce(&Self::Value) -> Self::Value>(&$_self, f: F) -> Self::Value {
            <$ty>::replace_with($expr, f)
        }
    };
}

pub(crate) use {proxy_share_value, proxy_share_value_non_eq};

impl<S: ?Sized + ShareValue> ShareValue for &S {
    proxy_share_value!(|self| -> S { self }, |other| other);

    fn try_unwrap(self) -> Result<Self::Value, Self>
    where
        Self: Sized,
    {
        Err(self)
    }

    fn unwrap_or_get_cloned(self) -> Self::Value
    where
        Self: Sized,
        Self::Value: Clone,
    {
        self.get_cloned()
    }
}

pub trait ToOwnedShareValue: ShareValue {
    type OwnedShareValue: ShareValue<Value = Self::Value>;
    fn to_owned_share_value(&self) -> Self::OwnedShareValue;
}

impl<S: ?Sized + ToOwnedShareValue> ToOwnedShareValue for &S {
    type OwnedShareValue = S::OwnedShareValue;

    fn to_owned_share_value(&self) -> Self::OwnedShareValue {
        S::to_owned_share_value(self)
    }
}
