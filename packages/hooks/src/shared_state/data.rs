use std::task::{Context, Poll, Waker};

use crate::{ShareValue, SharedRef};

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

pub struct SharedState<T> {
    shared_ref: SharedRef<(T, WakerStatus)>,
}

impl<T> Drop for SharedState<T> {
    fn drop(&mut self) {
        // This is the last rc.
        // Or, after this is dropped, rc will be no longer shared
        if self.shared_ref.shared_count() <= 2 {
            self.shared_ref.borrow_mut(|(_, waker), _| match waker {
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
        self.shared_ref.try_borrow(|v| {
            if let Some(v) = v {
                d.field(&v.0);
            } else {
                d.field(&"borrowed");
            }
        });

        d.finish()
    }
}

impl<T> Clone for SharedState<T> {
    fn clone(&self) -> Self {
        Self {
            shared_ref: self.shared_ref.clone(),
        }
    }
}

impl<T> SharedState<T> {
    #[inline]
    pub fn new(initial_value: T) -> Self {
        Self {
            shared_ref: SharedRef::new((initial_value, WakerStatus::Unregistered)),
        }
    }

    #[inline]
    pub fn new_with_waker(initial_value: T, waker: Option<Waker>) -> Self {
        Self {
            shared_ref: SharedRef::new((
                initial_value,
                waker.map_or(WakerStatus::Unregistered, WakerStatus::Registered),
            )),
        }
    }

    #[inline]
    pub fn notify_changed(&self) {
        self.shared_ref.map_mut(|(_, w)| w.notify_changed());
    }

    pub fn map_mut_and_notify_if<R>(&self, f: impl FnOnce(&mut T) -> (R, bool)) -> R {
        self.shared_ref.map_mut(move |(v, w)| {
            let (r, changed) = f(v);

            if changed {
                w.notify_changed();
            }

            r
        })
    }

    pub(super) fn impl_poll_next_update(&mut self, cx: &mut Context<'_>) -> Poll<bool> {
        self.shared_ref.borrow_mut(|(_, waker), status| {
            match waker {
                WakerStatus::Unregistered => {
                    if status.is_owned() {
                        // no updates are possible
                        Poll::Ready(false)
                    } else {
                        *waker = WakerStatus::Registered(cx.waker().clone());
                        Poll::Ready(true)
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
                WakerStatus::Updated => {
                    *waker = WakerStatus::Registered(cx.waker().clone());
                    Poll::Ready(true)
                }
            }
        })
    }
}

impl<T> ShareValue<T> for SharedState<T> {
    #[inline]
    fn is_shared(&self) -> bool {
        self.shared_ref.is_shared()
    }

    #[inline]
    fn get(&self) -> T
    where
        T: Copy,
    {
        self.shared_ref.map(|(v, _)| *v)
    }

    #[inline]
    fn get_cloned(&self) -> T
    where
        T: Clone,
    {
        self.shared_ref.map(|(v, _)| v.clone())
    }

    #[inline]
    fn replace(&self, new_value: T) -> T {
        self.map_mut(|v| std::mem::replace(v, new_value))
    }

    #[inline]
    fn map<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        self.shared_ref.map(|(v, _)| f(v))
    }

    #[inline]
    fn map_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        self.shared_ref.map_mut(|(v, w)| {
            w.notify_changed();
            f(v)
        })
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
        self.get_mut()
    }
];
