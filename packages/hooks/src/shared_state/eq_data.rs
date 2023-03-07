use std::task::{Context, Poll};

use crate::ShareValue;

use super::SharedState;

#[derive(Debug)]
pub struct SharedStateEq<T: PartialEq>(SharedState<T>);

impl<T: PartialEq> Unpin for SharedStateEq<T> {}

impl<T: PartialEq> Clone for SharedStateEq<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: PartialEq> SharedStateEq<T> {
    #[inline]
    pub fn new(initial_value: T) -> Self {
        Self(SharedState::new(initial_value))
    }

    #[inline]
    pub fn new_with_waker(initial_value: T, waker: Option<std::task::Waker>) -> SharedStateEq<T> {
        Self(SharedState::new_with_waker(initial_value, waker))
    }

    #[inline]
    pub fn inner(&self) -> &SharedState<T> {
        &self.0
    }

    pub(super) fn impl_poll_next_update(&mut self, cx: &mut Context<'_>) -> Poll<bool> {
        self.0.impl_poll_next_update(cx)
    }
}

impl<T: PartialEq> ShareValue<T> for SharedStateEq<T> {
    #[inline]
    fn is_shared(&self) -> bool {
        self.0.is_shared()
    }

    #[inline]
    fn get(&self) -> T
    where
        T: Copy,
    {
        self.0.get()
    }

    #[inline]
    fn get_cloned(&self) -> T
    where
        T: Clone,
    {
        self.0.get_cloned()
    }

    #[inline]
    fn set(&self, new_value: T) {
        self.0.map_mut_and_notify_if(move |old| {
            let changed = *old != new_value;
            *old = new_value;
            ((), changed)
        })
    }

    #[inline]
    fn replace(&self, new_value: T) -> T {
        self.0.map_mut_and_notify_if(move |old| {
            let changed = *old != new_value;
            let old = std::mem::replace(old, new_value);
            (old, changed)
        })
    }

    /// Always notify the value is changed
    /// because the caller can mutate the value inside `f`.
    ///
    /// To notify changes only when `new_value != old_value`,
    /// use [`replace_from_ref`](ShareValue::replace_from_ref) or
    /// [`self.inner().map_mut_and_notify_if()`](SharedStateData::map_mut_and_notify_if)
    /// instead.
    #[inline]
    fn replace_with<F: FnOnce(&mut T) -> T>(&self, f: F) -> T {
        self.0.replace_with(f)
    }

    #[inline]
    fn replace_from_ref<F: FnOnce(&T) -> T>(&self, f: F) -> T {
        self.0.map_mut_and_notify_if(move |v| {
            let new_value = f(v);
            let changed = new_value != *v;
            let old = std::mem::replace(v, new_value);
            (old, changed)
        })
    }

    #[inline]
    fn map<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        self.0.map(f)
    }

    #[inline]
    fn map_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        self.0.map_mut(f)
    }
}

hooks_core::impl_hook![
    type For<T: PartialEq> = SharedStateEq<T>;
    fn unmount() {}
    #[inline]
    fn poll_next_update(self, cx: _) {
        self.get_mut().impl_poll_next_update(cx)
    }
    #[inline]
    fn use_hook(self) -> &'hook Self {
        self.get_mut()
    }
];
