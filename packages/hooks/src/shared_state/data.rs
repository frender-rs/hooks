use std::{
    cell::Cell,
    rc::Rc,
    task::{Context, Poll, Waker},
};

use crate::{ShareValue, SharedRef};

pub struct SharedStateData<T> {
    shared_ref: SharedRef<T>,
    waker: Rc<Cell<Option<Waker>>>,
}

impl<T> Drop for SharedStateData<T> {
    fn drop(&mut self) {
        self.map_mut_waker(|w| {
            if let Some(w) = w.take() {
                w.wake()
            }
        })
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for SharedStateData<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("SharedStateData")
            .field(&self.shared_ref)
            .finish()
    }
}

impl<T> Clone for SharedStateData<T> {
    fn clone(&self) -> Self {
        Self {
            shared_ref: self.shared_ref.clone(),
            waker: self.waker.clone(),
        }
    }
}

impl<T> SharedStateData<T> {
    #[inline]
    pub fn new(initial_value: T) -> Self {
        Self {
            shared_ref: SharedRef::new(initial_value),
            waker: Default::default(),
        }
    }

    #[inline]
    pub fn notify_changed(&self) {
        if let Some(w) = self.waker.take() {
            w.wake()
        }
    }

    pub fn map_mut_and_notify_if<R>(&self, f: impl FnOnce(&mut T) -> (R, bool)) -> R {
        let (r, changed) = self.shared_ref.map_mut(f);
        if changed {
            self.notify_changed()
        }

        r
    }

    fn map_mut_waker<R>(&mut self, f: impl FnOnce(&mut Option<Waker>) -> R) -> R {
        if let Some(waker) = Rc::get_mut(&mut self.waker) {
            f(waker.get_mut())
        } else {
            let mut waker = self.waker.take();
            let r = f(&mut waker);
            self.waker.set(waker);
            r
        }
    }

    pub(super) fn impl_poll_next_update(&mut self, cx: &mut Context<'_>) -> Poll<bool> {
        if self.is_shared() {
            self.map_mut_waker(|w| {
                if w.is_some() {
                    // no updates happened yet.
                    Poll::Pending
                } else {
                    *w = Some(cx.waker().clone());
                    Poll::Ready(true)
                }
            })
        } else {
            Poll::Ready(false)
        }
    }
}

impl<T> ShareValue<T> for SharedStateData<T> {
    #[inline]
    fn is_shared(&self) -> bool {
        self.shared_ref.is_shared()
    }

    #[inline]
    fn get(&self) -> T
    where
        T: Copy,
    {
        self.shared_ref.get()
    }

    #[inline]
    fn get_cloned(&self) -> T
    where
        T: Clone,
    {
        self.shared_ref.get_cloned()
    }

    #[inline]
    fn replace(&self, new_value: T) -> T {
        self.notify_changed();
        self.shared_ref.replace(new_value)
    }

    #[inline]
    fn replace_with<F: FnOnce(&mut T) -> T>(&self, f: F) -> T {
        self.notify_changed();
        self.shared_ref.replace_with(f)
    }

    #[inline]
    fn map<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        self.shared_ref.map(f)
    }

    #[inline]
    fn map_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        self.notify_changed();
        self.shared_ref.map_mut(f)
    }
}
