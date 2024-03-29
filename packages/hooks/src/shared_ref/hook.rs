use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Default)]
pub struct SharedRef<T>(Rc<RefCell<T>>);

impl<T> Unpin for SharedRef<T> {}

impl<T> Clone for SharedRef<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> SharedRef<T> {
    #[inline]
    pub fn new(initial_value: T) -> Self {
        Self(Rc::new(RefCell::new(initial_value)))
    }

    #[inline]
    pub(crate) fn shared_count(&self) -> usize {
        // Weak is never created, thus only need strong count.
        Rc::strong_count(&self.0)
    }

    #[inline]
    pub(crate) fn borrow_mut<R>(
        &mut self,
        f: impl FnOnce(&mut T, crate::utils::RcStatus) -> R,
    ) -> R {
        crate::utils::rc_ref_cell_borrow_mut(&mut self.0, f)
    }

    #[inline]
    pub(crate) fn try_borrow<R>(&self, f: impl FnOnce(Option<&T>) -> R) -> R {
        let res = self.0.try_borrow();
        f(res.ok().as_deref())
    }
}

impl<T> crate::ShareValue<T> for SharedRef<T> {
    #[inline]
    fn is_shared(&self) -> bool {
        self.shared_count() != 0
    }

    #[inline]
    fn get(&self) -> T
    where
        T: Copy,
    {
        *self.0.borrow()
    }

    #[inline]
    fn get_cloned(&self) -> T
    where
        T: Clone,
    {
        self.0.borrow().clone()
    }

    #[inline]
    fn set(&self, new_value: T) {
        *self.0.borrow_mut() = new_value;
    }

    #[inline]
    fn replace(&self, new_value: T) -> T {
        self.0.replace(new_value)
    }

    #[inline]
    fn replace_mut<F: FnOnce(&mut T) -> T>(&self, f: F) -> T {
        self.0.replace_with(f)
    }

    #[inline]
    fn map<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        let v = self.0.borrow();
        f(&*v)
    }

    #[inline]
    fn map_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        let mut v = self.0.borrow_mut();
        f(&mut *v)
    }
}

hooks_core::impl_hook![
    type For<T> = SharedRef<T>;
    fn unmount() {}
    #[inline(always)]
    fn poll_next_update(self, _cx: _) {
        std::task::Poll::Ready(false)
    }
    #[inline]
    fn use_hook(self) -> &'hook Self {
        self.get_mut()
    }
];
