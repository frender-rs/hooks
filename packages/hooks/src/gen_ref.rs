use std::cell::RefCell;

use hooks_gen::{local, Key, Owner, Store};

pub struct GenRefOwner<T: 'static>(local::Owner<RefCell<T>>);

impl<T: 'static> Clone for GenRefOwner<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: 'static> GenRefOwner<T> {
    pub fn new(initial_value: T) -> Self {
        Self(local::Store.insert(RefCell::new(initial_value)))
    }

    /// The opposite of [`GenRefKey::upgrade`].
    pub fn key(&self) -> GenRefKey<T> {
        GenRefKey(self.0.key())
    }
}

impl<T: 'static> Unpin for GenRefOwner<T> {}

hooks_core::impl_hook!(
    type For<T: 'static> = GenRefOwner<T>;

    fn unmount() {}

    fn poll_next_update(self, _cx: _) {
        std::task::Poll::Ready(false)
    }

    #[inline]
    fn use_hook(self) -> GenRefKey<T> {
        GenRefKey(self.0.key())
    }
);

pub struct GenRefKey<T: 'static>(local::Key<RefCell<T>>);

impl<T: 'static> GenRefKey<T> {
    pub fn upgrade(self) -> GenRefOwner<T> {
        let owner = self.0.owner();
        GenRefOwner(owner)
    }
}

impl<T: 'static> Copy for GenRefKey<T> {}
impl<T: 'static> Clone for GenRefKey<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: 'static> Eq for GenRefKey<T> {}
impl<T: 'static> PartialEq for GenRefKey<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[cfg(feature = "ShareValue")]
impl<T: 'static> crate::ShareValue for GenRefOwner<T> {
    type Value = T;

    crate::proxy_share_value!(|self| -> GenRefKey<T> { &self.key() }, |other| {
        &other.key()
    });

    fn try_unwrap(self) -> Result<Self::Value, Self> {
        Err(self)
    }
}

#[cfg(feature = "ShareValue")]
impl<T: 'static> crate::ToOwnedShareValue for GenRefOwner<T> {
    type OwnedShareValue = GenRefKey<T>;

    fn to_owned_share_value(&self) -> Self::OwnedShareValue {
        self.key()
    }
}

#[cfg(feature = "ShareValue")]
impl<T: 'static> crate::ShareValue for GenRefKey<T> {
    type Value = T;

    fn try_unwrap(self) -> Result<Self::Value, Self>
    where
        Self: Sized,
    {
        Err(self)
    }

    fn map<R>(&self, f: impl FnOnce(&Self::Value) -> R) -> R {
        self.0.map(|v| f(&v.borrow()))
    }

    fn map_mut<R>(&self, f: impl FnOnce(&mut Self::Value) -> R) -> R {
        self.0.map(|v| f(&mut v.borrow_mut()))
    }

    fn equivalent_to(&self, other: &Self) -> bool {
        *self == *other
    }
}

#[cfg(feature = "ShareValue")]
impl<T: 'static> crate::ToOwnedShareValue for GenRefKey<T> {
    type OwnedShareValue = Self;

    fn to_owned_share_value(&self) -> Self::OwnedShareValue {
        *self
    }
}

pub struct UseGenRef<T: 'static>(pub T);

hooks_core::impl_hook!(
    type For<T: 'static> = UseGenRef<T>;

    fn into_hook(self) -> GenRefOwner<T> {
        GenRefOwner::new(self.0)
    }

    fn update_hook(self, _hook: _) {}

    fn h(self, hook: crate::utils::UninitializedHook<GenRefOwner<T>>) {
        hook.get_mut().use_into_or_update_hook(self)
    }
);

pub struct UseGenRefWith<F>(pub F);

hooks_core::impl_hook!(
    type For<T: 'static, F: FnOnce() -> T> = UseGenRefWith<F>;

    fn into_hook(self) -> GenRefOwner<T> {
        GenRefOwner::new(self.0())
    }

    fn update_hook(self, _hook: _) {}

    fn h(self, hook: crate::utils::UninitializedHook<GenRefOwner<T>>) {
        hook.get_mut().use_into_or_update_hook(self)
    }
);

#[cfg(test)]
#[cfg(feature = "ShareValue")]
#[cfg(feature = "futures-core")]
mod tests {
    use futures_lite::StreamExt;

    use crate::{GenRefOwner, ShareValue};

    #[test]
    #[cfg(feature = "use_effect")]
    fn ref_is_not_reactive() {
        use hooks_core::{hook_fn, IntoHook};

        use crate::{use_effect, use_gen_ref, ShareValue};

        hook_fn!(
            fn use_test() -> i32 {
                let ref_num = h![use_gen_ref(0)];

                let value = ref_num.get();

                h![use_effect(
                    move |v: &_| {
                        if *v < 2 {
                            ref_num.set(*v + 1);
                        }
                    },
                    value,
                )];

                value
            }
        );

        futures_lite::future::block_on(async {
            let values = use_test().into_hook_values();

            let values = values.collect::<Vec<_>>().await;
            assert_eq!(values, [0]);
        });
    }

    mod nested_share_value {
        use crate::{GenRefOwner, ShareValue};

        #[test]
        fn nested_map() {
            // self
            {
                let ref_num = GenRefOwner::new(0);
                ref_num.key().map(|_| ref_num.key().map(|_| {}));
            }
            // two
            {
                let ref_num = GenRefOwner::new(0);
                let ref_num_2 = GenRefOwner::new(0);

                ref_num.key().map(|_| ref_num_2.key().map(|_| {}));
                ref_num_2.key().map(|_| ref_num.key().map(|_| {}));
            }
        }

        #[test]
        #[should_panic(expected = "already borrowed: BorrowMutError")]
        fn nested_self_map_mut_should_panic() {
            let ref_num = GenRefOwner::new(0);
            ref_num.key().map_mut(|_| ref_num.key().map_mut(|_| {}));
        }

        #[test]
        #[should_panic(expected = "already mutably borrowed: BorrowError")]
        fn nested_self_map_mut_and_map_should_panic() {
            let ref_num = GenRefOwner::new(0);
            ref_num.key().map_mut(|_| ref_num.key().map(|_| {}));
            ref_num.key().map(|_| ref_num.key().map_mut(|_| {}));
        }

        #[test]
        #[should_panic(expected = "already borrowed: BorrowMutError")]
        fn nested_self_map_and_map_mut_should_panic() {
            let ref_num = GenRefOwner::new(0);
            ref_num.key().map(|_| ref_num.key().map_mut(|_| {}));
        }

        #[test]
        fn nested_map_mut() {
            // two
            {
                let ref_num = GenRefOwner::new(0);
                let ref_num_2 = GenRefOwner::new(0);

                ref_num.key().map_mut(|_| ref_num_2.key().map_mut(|_| {}));
                ref_num_2.key().map_mut(|_| ref_num.key().map_mut(|_| {}));
            }
        }
    }

    #[test]
    #[should_panic(expected = "already borrowed: BorrowMutError")]
    // This might be implemented with [elsa](https://docs.rs/elsa).
    // The type map of store is FrozenMap<TypeId, Box<Store>>.
    // The store is implemented with FrozenSlab,
    // which can be implemented with FrozenVec<Box<Chunk<Item>>>.
    // The chunk size increases exponentially
    fn insert_while_map() {
        let ref_num = GenRefOwner::new(0);
        ref_num.key().map(|_| {
            GenRefOwner::new(0);
        })
    }
}
