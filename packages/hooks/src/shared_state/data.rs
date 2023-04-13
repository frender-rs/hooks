use std::{
    cell::RefCell,
    rc::Rc,
    task::{Context, Poll, Waker},
};

use crate::{utils::RcStatus, ShareValue};

#[derive(Debug)]
enum WakerStatus {
    Unregistered,
    Registered(Waker),
    /// waker is taken due to updates
    Updated,
}

impl WakerStatus {
    #[inline]
    fn notify_changed(&mut self) {
        if let WakerStatus::Registered(w) = std::mem::replace(self, WakerStatus::Updated) {
            w.wake()
        }
    }
}

struct SharedStateInner<T> {
    value: RefCell<T>,
    waker_status: RefCell<WakerStatus>,
}

impl<T> SharedStateInner<T> {
    fn borrow_mut_waker_status<R>(
        self: &mut Rc<Self>,
        f: impl FnOnce(&mut WakerStatus, RcStatus) -> R,
    ) -> R {
        if let Some(this) = Rc::get_mut(self) {
            f(this.waker_status.get_mut(), RcStatus::Owned)
        } else {
            f(&mut self.waker_status.borrow_mut(), RcStatus::Shared)
        }
    }
}

pub struct SharedState<T> {
    inner: Rc<SharedStateInner<T>>,
}

impl<T> Drop for SharedState<T> {
    fn drop(&mut self) {
        // This is the last rc.
        // Or, after this is dropped, rc will be no longer shared
        if Rc::strong_count(&self.inner) <= 2 {
            self.inner.borrow_mut_waker_status(|waker, _| match waker {
                WakerStatus::Unregistered => {}
                WakerStatus::Registered(_) => {
                    if let WakerStatus::Registered(w) =
                        std::mem::replace(waker, WakerStatus::Unregistered)
                    {
                        w.wake()
                    }
                }
                WakerStatus::Updated => {}
            });
        }
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for SharedState<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_tuple("SharedState");
        match self.inner.value.try_borrow() {
            Ok(v) => {
                d.field(&v);
            }
            Err(_) => {
                d.field(&"borrowed");
            }
        }

        d.finish()
    }
}

impl<T> Clone for SharedState<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> SharedState<T> {
    #[inline]
    pub fn new(initial_value: T) -> Self {
        Self {
            inner: Rc::new(SharedStateInner {
                value: RefCell::new(initial_value),
                waker_status: RefCell::new(WakerStatus::Updated),
            }),
        }
    }

    #[inline]
    pub fn notify_changed(&self) {
        self.inner.waker_status.borrow_mut().notify_changed();
    }

    pub fn map_mut_and_notify_if<R>(&self, f: impl FnOnce(&mut T) -> (R, bool)) -> R {
        let (r, changed) = f(&mut self.inner.value.borrow_mut());

        if changed {
            self.notify_changed();
        }

        r
    }

    pub(super) fn impl_poll_next_update(&mut self, cx: &mut Context<'_>) -> Poll<bool> {
        self.inner.borrow_mut_waker_status(|waker, status| {
            match waker {
                WakerStatus::Unregistered => {
                    if status.is_owned() {
                        // no updates are possible
                        Poll::Ready(false)
                    } else {
                        *waker = WakerStatus::Registered(cx.waker().clone());
                        Poll::Pending
                    }
                }
                WakerStatus::Registered(w) => {
                    if status.is_owned() {
                        // no updates are possible
                        Poll::Ready(false)
                    } else {
                        // no updates happened yet.
                        *w = cx.waker().clone();
                        Poll::Pending
                    }
                }
                WakerStatus::Updated => Poll::Ready(true),
            }
        })
    }

    #[inline]
    pub(super) fn mark_as_unregistered(&mut self) {
        self.inner.borrow_mut_waker_status(|waker_status, _| {
            *waker_status = WakerStatus::Unregistered;
        });
    }
}

impl<T> ShareValue<T> for SharedState<T> {
    #[inline]
    fn is_shared(&self) -> bool {
        Rc::strong_count(&self.inner) != 0
    }

    #[inline]
    fn get(&self) -> T
    where
        T: Copy,
    {
        *self.inner.value.borrow()
    }

    #[inline]
    fn get_cloned(&self) -> T
    where
        T: Clone,
    {
        self.inner.value.borrow().clone()
    }

    #[inline]
    fn replace(&self, new_value: T) -> T {
        self.map_mut(|v| std::mem::replace(v, new_value))
    }

    #[inline]
    fn map<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        f(&self.inner.value.borrow())
    }

    #[inline]
    fn map_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        self.notify_changed();
        f(&mut self.inner.value.borrow_mut())
    }
}

hooks_core::impl_hook![
    type For<T> = SharedState<T>;
    fn unmount() {}
    #[inline]
    fn poll_next_update(self, cx: _) {
        self.get_mut().impl_poll_next_update(cx)
    }
    #[inline]
    fn use_hook(self) -> &'hook Self {
        let this = self.get_mut();
        this.mark_as_unregistered();
        this
    }
];
