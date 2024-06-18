#![forbid(unsafe_code)]

pub trait Key: Copy + PartialEq {
    type Value;
    type Owner: Owner<Key = Self>;

    fn owner(self) -> Self::Owner;

    fn map<R>(self, f: impl FnOnce(&Self::Value) -> R) -> R;
    fn map_mut<R>(self, f: impl FnOnce(&mut Self::Value) -> R) -> R;
}

pub trait Owner: Clone + PartialEq {
    type Key: Key;
    fn key(&self) -> Self::Key;

    fn shared_count(&self) -> usize;
}

pub trait Store<T> {
    type Key: Key<Value = T>;

    fn insert(&self, value: T) -> <Self::Key as Key>::Owner;
}

pub mod local {
    use std::marker::PhantomData;

    /// Key is !Send + !Sync.
    /// ```compile_fail
    /// # fn test(v: hooks_gen::local::Key<()>) -> impl Send { v };
    /// ```
    /// ```compile_fail
    /// # fn test(v: hooks_gen::local::Key<()>) -> impl Sync { v };
    /// ```
    pub struct Key<T: 'static> {
        index: store::UntypedKey,
        _phantom: PhantomData<*const T>,
    }

    impl<T: std::fmt::Debug> std::fmt::Debug for Key<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut d = f.debug_struct("Key");
            let d = d.field("index", &self.index);
            match self.try_map(|v| {
                if let Some(v) = v {
                    d.field("value", v)
                } else {
                    d.field("<value not present>", &())
                }
            }) {
                Ok(d) => d,
                Err(error) => d.field("store", &error),
            }
            .finish()
        }
    }

    impl<T> Eq for Key<T> {}
    impl<T> PartialEq for Key<T> {
        fn eq(&self, other: &Self) -> bool {
            self.index == other.index
        }
    }

    impl<T> Key<T> {
        const fn new(index: store::UntypedKey) -> Self {
            Self {
                index,
                _phantom: PhantomData,
            }
        }

        fn try_map<R>(self, f: impl FnOnce(Option<&T>) -> R) -> Result<R, std::cell::BorrowError> {
            <Store as StoreImpl>::try_map_at(self, f)
        }
    }

    impl<T> Copy for Key<T> {}
    impl<T> Clone for Key<T> {
        fn clone(&self) -> Self {
            *self
        }
    }

    impl<T: 'static> super::Key for Key<T> {
        type Value = T;

        type Owner = Owner<T>;

        fn owner(self) -> Self::Owner {
            Store::register_owner_at(self)
        }

        fn map<R>(self, f: impl FnOnce(&Self::Value) -> R) -> R {
            Store::map_at(self, f)
        }

        /// Note that this locks the whole store.
        fn map_mut<R>(self, f: impl FnOnce(&mut Self::Value) -> R) -> R {
            Store::map_mut_at(self, f)
        }
    }

    pub struct Owner<T: 'static> {
        key: Key<T>,
    }

    impl<T: std::fmt::Debug> std::fmt::Debug for Owner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_tuple("Owner").field(&self.key).finish()
        }
    }

    impl<T: 'static> PartialEq for Owner<T> {
        fn eq(&self, other: &Self) -> bool {
            self.key == other.key
        }
    }

    pub struct Store;

    trait StoreImpl {
        fn insert<T: 'static>(value: T) -> Owner<T>;
        fn unregister_owner<T>(owner: &mut Owner<T>);
        fn register_owner_at<T>(key: Key<T>) -> Owner<T>;

        fn owners_count_at<T: 'static>(key: Key<T>) -> usize;

        fn map_at<T: 'static, R>(key: Key<T>, f: impl FnOnce(&T) -> R) -> R;
        fn try_map_at<T: 'static, R>(
            key: Key<T>,
            f: impl FnOnce(Option<&T>) -> R,
        ) -> Result<R, std::cell::BorrowError>;

        fn map_mut_at<T: 'static, R>(key: Key<T>, f: impl FnOnce(&mut T) -> R) -> R;
    }

    mod store {
        use std::{
            any::{Any, TypeId},
            cell::{Cell, RefCell},
            collections::HashMap,
        };

        use super::{Key, Owner};

        pub(super) type UntypedKey = generational_slab::Index;

        mod generational_slab {
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub(crate) struct Index {
                index: usize,
                generation: u64,
            }

            pub struct ValueWithGeneration<T> {
                value: T,
                generation: u64,
            }

            pub(super) struct GenerationalSlab<T> {
                slab: slab::Slab<ValueWithGeneration<T>>,
                generation: u64,
            }

            impl<T> GenerationalSlab<T> {
                pub(super) const fn new() -> Self {
                    Self {
                        slab: slab::Slab::new(),
                        generation: 0,
                    }
                }

                pub(super) fn insert(&mut self, value: T) -> Index {
                    let generation = self.generation;
                    let index = self.slab.insert(ValueWithGeneration { value, generation });
                    self.generation = self.generation.wrapping_add(1);
                    Index { index, generation }
                }

                pub(super) fn get(&self, index: Index) -> Option<&T> {
                    match self.slab.get(index.index) {
                        Some(vg) if vg.generation == index.generation => Some(&vg.value),
                        _ => None,
                    }
                }

                pub(super) fn get_mut(&mut self, index: Index) -> Option<&mut T> {
                    match self.slab.get_mut(index.index) {
                        Some(vg) if vg.generation == index.generation => Some(&mut vg.value),
                        _ => None,
                    }
                }

                pub(super) fn remove(&mut self, index: Index) -> T {
                    let vg = self.slab.remove(index.index);
                    assert_eq!(vg.generation, index.generation, "wrong generation");
                    vg.value
                }
            }
        }

        struct Item<T> {
            value: T,
            owners_count: Cell<usize>,
        }

        type TypedStore<T> = generational_slab::GenerationalSlab<Item<T>>;

        thread_local!(
            static STORES: RefCell<
                HashMap<
                    TypeId,
                    // Box<TypedStore<T>>
                    Box<dyn Any>,
                >,
            > = Default::default();
        );

        fn store_mut<T: 'static, R>(f: impl FnOnce(&mut TypedStore<T>) -> R) -> R {
            STORES.with_borrow_mut(|stores| {
                let store = stores
                    .entry(TypeId::of::<T>())
                    .or_insert_with(|| Box::new(TypedStore::<T>::new()))
                    .downcast_mut::<TypedStore<T>>()
                    .unwrap();
                f(store)
            })
        }

        fn store<T: 'static, R>(f: impl FnOnce(&TypedStore<T>) -> R) -> R {
            STORES.with_borrow(|stores| {
                let store = stores
                    .get(&TypeId::of::<T>())
                    .unwrap()
                    .downcast_ref::<TypedStore<T>>()
                    .unwrap();
                f(store)
            })
        }

        fn try_store<T: 'static, R>(
            f: impl FnOnce(Option<&TypedStore<T>>) -> R,
        ) -> Result<R, std::cell::BorrowError> {
            STORES.with(|stores| {
                let stores = stores.try_borrow()?;
                let store = stores
                    .get(&TypeId::of::<T>())
                    .map(|store| store.downcast_ref::<TypedStore<T>>().unwrap());
                Ok(f(store))
            })
        }

        impl super::StoreImpl for super::Store {
            fn insert<T: 'static>(value: T) -> Owner<T> {
                store_mut::<T, Owner<T>>(|s| {
                    let index = s.insert(Item {
                        value,
                        owners_count: Cell::new(1),
                    });
                    let key = Key::new(index);
                    Owner { key }
                })
            }

            fn unregister_owner<T>(owner: &mut Owner<T>) {
                let index = owner.key.index;

                let should_remove = store::<T, bool>(|items| {
                    let item = items.get(index).unwrap();

                    let new_count = item.owners_count.get() - 1;

                    if new_count == 0 {
                        true
                    } else {
                        item.owners_count.set(new_count);
                        false
                    }
                });

                if should_remove {
                    store_mut::<T, ()>(|items| {
                        let _: T = items.remove(index).value;
                    })
                }
            }

            fn register_owner_at<T>(key: Key<T>) -> Owner<T> {
                store::<T, ()>(|items| {
                    let item = items.get(key.index).unwrap();

                    item.owners_count.set(item.owners_count.get() + 1);
                });

                Owner { key }
            }

            fn owners_count_at<T: 'static>(key: Key<T>) -> usize {
                store::<T, usize>(|items| items.get(key.index).unwrap().owners_count.get())
            }

            fn map_at<T: 'static, R>(key: Key<T>, f: impl FnOnce(&T) -> R) -> R {
                store::<T, R>(|items| f(&items.get(key.index).unwrap().value))
            }

            fn try_map_at<T: 'static, R>(
                key: Key<T>,
                f: impl FnOnce(Option<&T>) -> R,
            ) -> Result<R, std::cell::BorrowError> {
                try_store::<T, R>(|items| {
                    let item = if let Some(items) = items {
                        items.get(key.index).map(|item: &Item<T>| &item.value)
                    } else {
                        None
                    };
                    f(item)
                })
            }

            fn map_mut_at<T: 'static, R>(key: Key<T>, f: impl FnOnce(&mut T) -> R) -> R {
                store_mut::<T, R>(|items| f(&mut items.get_mut(key.index).unwrap().value))
            }
        }
    }

    impl<T: 'static> super::Store<T> for Store {
        type Key = Key<T>;

        fn insert(&self, value: T) -> Owner<T> {
            <Store as StoreImpl>::insert::<T>(value)
        }
    }

    impl<T: 'static> super::Owner for Owner<T> {
        type Key = Key<T>;

        fn key(&self) -> Self::Key {
            self.key
        }

        fn shared_count(&self) -> usize {
            Store::owners_count_at(self.key)
        }
    }

    impl<T: 'static> Clone for Owner<T> {
        fn clone(&self) -> Self {
            use super::Key;
            self.key.owner()
        }
    }

    impl<T: 'static> Drop for Owner<T> {
        fn drop(&mut self) {
            Store::unregister_owner::<T>(self)
        }
    }
}
