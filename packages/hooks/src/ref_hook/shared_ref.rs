use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Default)]
pub struct SharedRef<T>(Rc<RefCell<T>>);

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
}

impl<T> crate::ShareValue<T> for SharedRef<T> {
    #[inline]
    fn is_shared(&self) -> bool {
        // Weak is never created, thus only need to check strong count.
        Rc::strong_count(&self.0) != 0
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
    fn replace_with<F: FnOnce(&mut T) -> T>(&self, f: F) -> T {
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
