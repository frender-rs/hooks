use std::{
    cell::RefCell,
    rc::Rc,
    task::{Context, Poll, Waker},
};

use crate::{utils::RcStatus, ShareValue};

struct SharedStateInner<T> {
    value: RefCell<T>,
    notifiers: RefCell<Notifiers>,
}

impl<T> SharedStateInner<T> {
    fn borrow_mut_notifiers<R>(
        self: &mut Rc<Self>,
        f: impl FnOnce(&mut Notifiers, RcStatus) -> R,
    ) -> R {
        if let Some(this) = Rc::get_mut(self) {
            f(this.notifiers.get_mut(), RcStatus::Owned)
        } else {
            f(&mut self.notifiers.borrow_mut(), RcStatus::Shared)
        }
    }
}

struct Notifier {
    seen: bool,
    waker: Option<Waker>,
}

struct Notifiers(slab1::Slab1<Notifier>);

impl Notifiers {
    /// Panics if `key` points to vacant entry.
    fn get_mut_occupied(&mut self, key: usize) -> &mut Notifier {
        self.0.get_mut(key).unwrap()
    }

    fn new_1() -> Self {
        Self(slab1::Slab1::new_1(Notifier {
            seen: false,
            waker: None,
        }))
    }

    fn add(&mut self) -> usize {
        self.0.insert(Notifier {
            seen: false,
            waker: None,
        })
    }

    /// Panics if `key` points to vacant entry.
    fn remove_at_occupied(&mut self, key: usize) {
        _ = self.0.remove(key)
    }

    fn notify(&mut self) {
        self.0.for_each_mut(|Notifier { waker, .. }| {
            if let Some(waker) = waker.take() {
                waker.wake()
            }
        })
    }

    fn notify_changed(&mut self) {
        self.0.for_each_mut(|Notifier { seen, waker }| {
            *seen = false;
            if let Some(waker) = waker.take() {
                waker.wake()
            }
        })
    }
}

pub struct SharedState<T> {
    inner: Rc<SharedStateInner<T>>,
    key: usize,
}

impl<T> Drop for SharedState<T> {
    fn drop(&mut self) {
        let mut notifiers = self.inner.notifiers.borrow_mut();
        notifiers.remove_at_occupied(self.key);

        // This is the last rc.
        // Or, after this is dropped, rc will be no longer shared
        if Rc::strong_count(&self.inner) <= 2 {
            notifiers.notify();
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
            key: self.inner.notifiers.borrow_mut().add(),
        }
    }
}

impl<T> SharedState<T> {
    #[inline]
    pub fn new(initial_value: T) -> Self {
        Self {
            inner: Rc::new(SharedStateInner {
                value: RefCell::new(initial_value),
                notifiers: RefCell::new(Notifiers::new_1()),
            }),
            key: 0,
        }
    }

    #[inline]
    pub fn notify_changed(&self) {
        self.inner.notifiers.borrow_mut().notify_changed();
    }

    pub fn map_mut_and_notify_if<R>(&self, f: impl FnOnce(&mut T) -> (R, bool)) -> R {
        let (r, changed) = f(&mut self.inner.value.borrow_mut());

        if changed {
            self.notify_changed();
        }

        r
    }

    pub(super) fn impl_poll_next_update(&mut self, cx: &mut Context<'_>) -> Poll<bool> {
        self.inner.borrow_mut_notifiers(|notifiers, status| {
            let notifier = notifiers.get_mut_occupied(self.key);
            if notifier.seen {
                if status.is_owned() {
                    // no updates are possible
                    Poll::Ready(false)
                } else {
                    let new_waker = cx.waker();
                    if !(notifier
                        .waker
                        .as_ref()
                        .is_some_and(|old_waker| old_waker.will_wake(new_waker)))
                    {
                        notifier.waker = Some(new_waker.clone());
                    }
                    Poll::Pending
                }
            } else {
                Poll::Ready(true)
            }
        })
    }

    #[inline]
    pub(super) fn mark_as_seen(&mut self) {
        let mut notifiers = self.inner.notifiers.borrow_mut();
        let notifier = notifiers.get_mut_occupied(self.key);
        notifier.seen = true;
    }
}

impl<T> ShareValue for SharedState<T> {
    type Value = T;

    fn try_unwrap(self) -> Result<Self::Value, Self>
    where
        Self: Sized,
    {
        if Rc::strong_count(&self.inner) == 1 {
            let inner = self.inner.clone();
            drop(self);

            match Rc::try_unwrap(inner) {
                Ok(inner) => Ok(inner.value.into_inner()),
                Err(_) => unreachable!(),
            }
        } else {
            Err(self)
        }
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

    fn equivalent_to(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.inner, &other.inner)
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
        this.mark_as_seen();
        this
    }
];

/// Inspired by [`slab`](https://docs.rs/slab).
mod slab1 {
    use std::mem;

    enum Entry<T> {
        /// next vacant index
        Vacant(usize),
        Occupied(T),
    }

    impl<T> Entry<T> {
        fn as_mut_occupied(&mut self) -> Option<&mut T> {
            match self {
                Entry::Vacant(_) => None,
                Entry::Occupied(v) => Some(v),
            }
        }
    }

    struct Vec1<T> {
        first: T,
        rest: Vec<T>,
    }

    impl<T> Vec1<T> {
        fn len(&self) -> usize {
            1 + self.rest.len()
        }

        fn push(&mut self, value: T) {
            self.rest.push(value)
        }

        fn get_mut(&mut self, index: usize) -> Option<&mut T> {
            if index == 0 {
                Some(&mut self.first)
            } else {
                self.rest.get_mut(index - 1)
            }
        }

        fn for_each_mut(&mut self, mut f: impl FnMut(&mut T)) {
            f(&mut self.first);
            self.rest.iter_mut().for_each(f);
        }
    }

    pub(super) struct Slab1<T> {
        entries: Vec1<Entry<T>>,
        next: usize,
    }

    impl<T> Slab1<T> {
        pub(super) fn new_1(value: T) -> Self {
            Self {
                entries: Vec1 {
                    first: Entry::Occupied(value),
                    rest: vec![],
                },
                next: 1,
            }
        }

        pub(super) fn get_mut(&mut self, key: usize) -> Option<&mut T> {
            self.entries.get_mut(key).and_then(Entry::as_mut_occupied)
        }

        pub(super) fn for_each_mut(&mut self, mut f: impl FnMut(&mut T)) {
            self.entries.for_each_mut(|entry| match entry {
                Entry::Vacant(_) => {}
                Entry::Occupied(v) => f(v),
            })
        }

        fn try_remove(&mut self, key: usize) -> Option<T> {
            if let Some(entry) = self.entries.get_mut(key) {
                // Swap the entry at the provided value
                let prev = mem::replace(entry, Entry::Vacant(self.next));

                match prev {
                    Entry::Occupied(val) => {
                        // self.len -= 1;
                        self.next = key;
                        return val.into();
                    }
                    _ => {
                        // Woops, the entry is actually vacant, restore the state
                        *entry = prev;
                    }
                }
            }
            None
        }

        pub fn remove(&mut self, key: usize) -> T {
            self.try_remove(key).expect("invalid key")
        }

        pub(super) fn insert(&mut self, val: T) -> usize {
            let key = self.next;

            self.insert_at_vacant(key, val);

            key
        }

        /// Panics if the entry at `key` is not vacant.
        fn insert_at_vacant(&mut self, key: usize, val: T) {
            // self.len += 1;

            if key == self.entries.len() {
                self.entries.push(Entry::Occupied(val));
                self.next = key + 1;
            } else {
                match self.entries.get_mut(key) {
                    Some(entry) => {
                        let next = if let Entry::Vacant(next) = entry {
                            *next
                        } else {
                            unreachable!()
                        };
                        self.next = next;
                        *entry = Entry::Occupied(val);
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}
