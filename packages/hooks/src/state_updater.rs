use std::{cell::RefCell, ops::DerefMut, rc::Rc};

use crate::utils::debug_pointer::{DebugHigherKindFnPointerRefMut, DebugPointerType};

pub enum NewState<'a, T> {
    ByValue(T),
    ByFnPointer(fn(&mut T)),
    ByFnOnce(Box<dyn 'a + FnOnce(&mut T)>),
}

impl<'a, T: std::fmt::Debug> std::fmt::Debug for NewState<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[allow(clippy::borrowed_box)]
        struct BoxedType<'a, A: ?Sized>(&'a Box<A>);

        impl<A: ?Sized> std::fmt::Debug for BoxedType<'_, A> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "Box<{}>@{:p}", std::any::type_name::<A>(), *self.0)
            }
        }

        match self {
            Self::ByValue(arg0) => f.debug_tuple("ByValue").field(arg0).finish(),
            Self::ByFnPointer(arg0) => f
                .debug_tuple("ByFnPointer")
                .field(&DebugHigherKindFnPointerRefMut(arg0))
                .finish(),
            Self::ByFnOnce(arg0) => f
                .debug_tuple("ByFnOnce")
                .field(&DebugPointerType::<Box<dyn 'a + FnOnce(&mut T)>>(arg0))
                .finish(),
        }
    }
}

pub const STAGING_STATES_DEFAULT_STACK_COUNT: usize = 3;

#[derive(Default, Debug)]
pub struct StagingStates<'a, T, const N: usize = STAGING_STATES_DEFAULT_STACK_COUNT>(
    smallvec::SmallVec<[NewState<'a, T>; N]>,
);

impl<'a, T, const N: usize> StagingStates<'a, T, N> {
    #[inline]
    pub fn new() -> Self {
        Self(smallvec::SmallVec::new())
    }

    pub fn drain_into(&mut self, state: &mut T) {
        let drain = self.0.drain(..);
        for a in drain {
            match a {
                NewState::ByValue(v) => *state = v,
                NewState::ByFnPointer(f) => f(state),
                NewState::ByFnOnce(f) => f(state),
            }
        }
    }

    pub fn push(&mut self, new_state: NewState<'a, T>) {
        if let NewState::ByValue(_) = &new_state {
            self.0.truncate(0);
        }

        self.0.push(new_state)
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

pub struct DeferredStateUpdater<'a, T, const N: usize = STAGING_STATES_DEFAULT_STACK_COUNT> {
    waker_and_staging_states: Rc<RefCell<(Option<std::task::Waker>, StagingStates<'a, T, N>)>>,
}

impl<'a, T, const N: usize> Clone for DeferredStateUpdater<'a, T, N> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            waker_and_staging_states: self.waker_and_staging_states.clone(),
        }
    }
}

impl<'a, T, const N: usize> Default for DeferredStateUpdater<'a, T, N> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T, const N: usize> DeferredStateUpdater<'a, T, N> {
    #[inline]
    pub fn new() -> Self {
        Self {
            waker_and_staging_states: Rc::new(RefCell::new((None, StagingStates::new()))),
        }
    }

    pub fn update(&self, new_state: NewState<'a, T>) {
        let mut waker_and_staging_states = self.waker_and_staging_states.borrow_mut();
        waker_and_staging_states.1.push(new_state);

        if let Some(waker) = waker_and_staging_states.0.take() {
            waker.wake();
        }
    }

    #[inline]
    pub fn update_by_value(&self, new_value: T) {
        self.update(NewState::ByValue(new_value))
    }

    #[inline]
    pub fn update_by_fn_once<F: 'a + FnOnce(&mut T)>(&self, f: F) {
        self.update(NewState::ByFnOnce(Box::new(f)))
    }

    #[inline]
    pub fn update_by_fn_pointer(&self, f: fn(&mut T)) {
        self.update(NewState::ByFnPointer(f))
    }

    #[inline]
    pub(crate) fn get_mut(
        &mut self,
    ) -> Option<&mut (Option<std::task::Waker>, StagingStates<'a, T, N>)> {
        Rc::get_mut(&mut self.waker_and_staging_states).map(RefCell::get_mut)
    }

    /// `bool` argument indicates whether `RefCell::borrow_mut` is called.
    /// - `true` means the `Rc<RefCell<T>>` is shared, causing a runtime `RefCell::borrow_mut`.
    /// - `false` means there are no other Rc or Weak pointers to the same allocation.
    #[inline]
    pub(crate) fn map_mut<
        R,
        F: FnOnce(&mut (Option<std::task::Waker>, StagingStates<'a, T, N>), bool) -> R,
    >(
        &mut self,
        f: F,
    ) -> R {
        if let Some(v) = self.get_mut() {
            f(v, false)
        } else {
            let mut v = self.waker_and_staging_states.borrow_mut();
            f(v.deref_mut(), true)
        }
    }

    pub(crate) fn drain_into<'this>(
        &'this mut self,
        old_state: &'this mut T,
    ) -> (&'this mut T, &'this Self) {
        self.map_mut(|(_, staging_states), _| staging_states.drain_into(old_state));

        (old_state, self)
    }
}

impl<'a, T, const N: usize> Drop for DeferredStateUpdater<'a, T, N> {
    /// When [`DeferredStateUpdater`] is dropped,
    /// it will wake up the task to notify
    /// the shared count has changed.
    fn drop(&mut self) {
        let mut waker_and_staging_states = self.waker_and_staging_states.borrow_mut();

        if let Some(waker) = waker_and_staging_states.0.take() {
            waker.wake()
        }
    }
}
