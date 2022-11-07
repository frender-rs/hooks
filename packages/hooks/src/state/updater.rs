use std::{cell::RefCell, ops::DerefMut, rc::Rc};

use crate::utils::{
    debug_pointer::{
        DebugHigherKindFnPointerRefMut, DebugHigherKindFnPointerRefOutput, DebugPointerType,
    },
    RcStatus,
};

#[derive(Debug)]
pub enum NewState<'a, T> {
    Value(T),
    Fn(NewStateFn<'a, T>),
}

pub type NewStateDynReplacer<'a, T> = dyn 'a + FnOnce(&T) -> T;
pub type NewStateDynReplacerMaybe<'a, T> = dyn 'a + FnOnce(&T) -> Option<T>;
pub type NewStateDynMutator<'a, T> = dyn 'a + FnOnce(&mut T);

pub enum NewStateFn<'a, T> {
    ReplacerBox(Box<NewStateDynReplacer<'a, T>>),
    ReplacerFnPointer(fn(&T) -> T),
    ReplacerMaybeBox(Box<NewStateDynReplacerMaybe<'a, T>>),
    ReplacerMaybeFnPointer(fn(&T) -> Option<T>),
    MutatorBox(Box<NewStateDynMutator<'a, T>>),
    MutatorFnPointer(fn(&mut T)),
}

impl<'a, T> From<fn(&T) -> T> for NewStateFn<'a, T> {
    #[inline]
    fn from(v: fn(&T) -> T) -> Self {
        Self::ReplacerFnPointer(v)
    }
}

impl<'a, T> From<Box<dyn 'a + FnOnce(&T) -> T>> for NewStateFn<'a, T> {
    #[inline]
    fn from(v: Box<dyn 'a + FnOnce(&T) -> T>) -> Self {
        Self::ReplacerBox(v)
    }
}

impl<'a, T> From<fn(&T) -> Option<T>> for NewStateFn<'a, T> {
    #[inline]
    fn from(v: fn(&T) -> Option<T>) -> Self {
        Self::ReplacerMaybeFnPointer(v)
    }
}

impl<'a, T> From<Box<dyn 'a + FnOnce(&T) -> Option<T>>> for NewStateFn<'a, T> {
    #[inline]
    fn from(v: Box<dyn 'a + FnOnce(&T) -> Option<T>>) -> Self {
        Self::ReplacerMaybeBox(v)
    }
}

impl<'a, T> From<fn(&mut T)> for NewStateFn<'a, T> {
    #[inline]
    fn from(v: fn(&mut T)) -> Self {
        Self::MutatorFnPointer(v)
    }
}

impl<'a, T> From<Box<dyn 'a + FnOnce(&mut T)>> for NewStateFn<'a, T> {
    #[inline]
    fn from(v: Box<dyn 'a + FnOnce(&mut T)>) -> Self {
        Self::MutatorBox(v)
    }
}

impl<'a, T: std::fmt::Debug> std::fmt::Debug for NewStateFn<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ReplacerBox(arg0) => f
                .debug_tuple("ByReplacerBox")
                .field(&DebugPointerType::<Box<dyn 'a + FnOnce(&T) -> T>>(arg0))
                .finish(),
            Self::ReplacerFnPointer(arg0) => f
                .debug_tuple("ByReplacerFnPointer")
                .field(&DebugHigherKindFnPointerRefOutput(arg0))
                .finish(),
            Self::ReplacerMaybeBox(arg0) => f
                .debug_tuple("ReplacerMaybeBox")
                .field(&DebugPointerType(arg0))
                .finish(),
            Self::ReplacerMaybeFnPointer(arg0) => f
                .debug_tuple("ReplacerMaybeFnPointer")
                .field(&DebugHigherKindFnPointerRefOutput(arg0))
                .finish(),
            Self::MutatorBox(arg0) => f
                .debug_tuple("ByMutatorBox")
                .field(&DebugPointerType::<Box<dyn 'a + FnOnce(&mut T)>>(arg0))
                .finish(),
            Self::MutatorFnPointer(arg0) => f
                .debug_tuple("ByMutatorFnPointer")
                .field(&DebugHigherKindFnPointerRefMut(arg0))
                .finish(),
        }
    }
}

pub const STAGING_STATES_DEFAULT_STACK_COUNT: usize = 3;

#[derive(Default, Debug)]
pub struct StagingStates<'a, T, const N: usize = STAGING_STATES_DEFAULT_STACK_COUNT> {
    new_state: Option<T>,
    fns: smallvec::SmallVec<[NewStateFn<'a, T>; N]>,
}

impl<'a, T, const N: usize> StagingStates<'a, T, N> {
    #[inline]
    pub fn new() -> Self {
        Self {
            new_state: None,
            fns: smallvec::SmallVec::new(),
        }
    }

    /// Return `compare(&old_value, &new_value)`.
    pub fn drain_into_and_compare(
        &mut self,
        state: &mut T,
        mut compare: impl FnMut(&T, &T) -> bool,
    ) -> bool {
        if let Some(mut new_state) = self.new_state.take() {
            self.drain_fns_into(&mut new_state);

            let is_equal = compare(&*state, &new_state);

            *state = new_state;

            is_equal
        } else {
            let drain = self.fns.drain(..);

            let mut mutated = false;

            let mut last_state: Option<T> = None;

            for new_state in drain {
                match new_state {
                    NewStateFn::ReplacerBox(f) => {
                        if let Some(last_state) = &mut last_state {
                            *last_state = f(&*last_state)
                        } else {
                            last_state = Some(f(&*state))
                        }
                    }
                    NewStateFn::ReplacerFnPointer(f) => {
                        if let Some(last_state) = &mut last_state {
                            *last_state = f(&*last_state)
                        } else {
                            last_state = Some(f(&*state))
                        }
                    }
                    NewStateFn::ReplacerMaybeBox(f) => {
                        if let Some(last_state) = &mut last_state {
                            if let Some(new_state) = f(&*last_state) {
                                *last_state = new_state;
                            }
                        } else {
                            last_state = f(&*state)
                        }
                    }
                    NewStateFn::ReplacerMaybeFnPointer(f) => {
                        if let Some(last_state) = &mut last_state {
                            if let Some(new_state) = f(&*last_state) {
                                *last_state = new_state;
                            }
                        } else {
                            last_state = f(&*state)
                        }
                    }
                    NewStateFn::MutatorFnPointer(f) => {
                        if let Some(last_state) = &mut last_state {
                            f(last_state)
                        } else {
                            mutated = true;
                            f(state)
                        }
                    }
                    NewStateFn::MutatorBox(f) => {
                        if let Some(last_state) = &mut last_state {
                            f(last_state)
                        } else {
                            mutated = true;
                            f(state)
                        }
                    }
                }
            }

            if let Some(last_state) = last_state {
                let is_equal = !mutated && compare(&*state, &last_state);
                *state = last_state;
                is_equal
            } else {
                !mutated
            }
        }
    }

    /// Returning `true` indicates there are no new states.
    #[inline]
    pub fn drain_into(&mut self, state: &mut T) -> bool {
        if self.is_empty() {
            return true;
        }

        let mut is_equal = true;

        if let Some(new_state) = self.new_state.take() {
            *state = new_state;
            is_equal = false;
        }

        self.drain_fns_into(state) && is_equal
    }

    /// Returning `true` indicates there are no new states.
    fn drain_fns_into(&mut self, state: &mut T) -> bool {
        if self.fns.is_empty() {
            return true;
        }

        let mut is_equal = true;

        let drain = self.fns.drain(..);
        for a in drain {
            match a {
                NewStateFn::ReplacerBox(f) => {
                    *state = f(state);
                    is_equal = false;
                }
                NewStateFn::ReplacerFnPointer(f) => {
                    *state = f(state);
                    is_equal = false;
                }
                NewStateFn::ReplacerMaybeBox(f) => {
                    if let Some(new_state) = f(state) {
                        *state = new_state;
                        is_equal = false;
                    }
                }
                NewStateFn::ReplacerMaybeFnPointer(f) => {
                    if let Some(new_state) = f(state) {
                        *state = new_state;
                        is_equal = false;
                    }
                }
                NewStateFn::MutatorFnPointer(f) => {
                    f(state);
                    is_equal = false;
                }
                NewStateFn::MutatorBox(f) => {
                    f(state);
                    is_equal = false;
                }
            }
        }

        is_equal
    }

    pub fn push(&mut self, new_state: NewState<'a, T>) {
        match new_state {
            NewState::Value(new_state) => {
                self.fns.truncate(0);
                self.new_state = Some(new_state);
            }
            NewState::Fn(f) => self.fns.push(f),
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.new_state.is_none() && self.fns.is_empty()
    }
}

pub struct StateUpdater<'a, T, const N: usize = STAGING_STATES_DEFAULT_STACK_COUNT> {
    waker_and_staging_states: Rc<RefCell<(Option<std::task::Waker>, StagingStates<'a, T, N>)>>,
}

impl<'a, T, const N: usize> Clone for StateUpdater<'a, T, N> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            waker_and_staging_states: self.waker_and_staging_states.clone(),
        }
    }
}

impl<'a, T, const N: usize> Default for StateUpdater<'a, T, N> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T, const N: usize> StateUpdater<'a, T, N> {
    #[inline]
    pub fn new() -> Self {
        Self {
            waker_and_staging_states: Rc::new(RefCell::new((None, StagingStates::new()))),
        }
    }

    pub fn update_by(&self, new_state: NewState<'a, T>) {
        let mut waker_and_staging_states = self.waker_and_staging_states.borrow_mut();
        waker_and_staging_states.1.push(new_state);

        if let Some(waker) = waker_and_staging_states.0.take() {
            waker.wake();
        }
    }

    #[inline]
    pub fn set(&self, new_state: T) {
        self.update_by(NewState::Value(new_state))
    }

    #[inline]
    pub fn update_by_fn(&self, f: impl Into<NewStateFn<'a, T>>) {
        self.update_by(NewState::Fn(f.into()))
    }

    #[inline]
    pub fn replace_with_fn_box(&self, f: impl 'a + FnOnce(&T) -> T) {
        self.update_by(NewState::Fn(NewStateFn::ReplacerBox(Box::new(f))))
    }

    #[inline]
    pub fn replace_with_fn_pointer(&self, f: fn(&T) -> T) {
        self.update_by(NewState::Fn(NewStateFn::ReplacerFnPointer(f)))
    }

    #[inline]
    pub fn replace_maybe_with_fn_box(&self, f: impl 'a + FnOnce(&T) -> Option<T>) {
        self.update_by(NewState::Fn(NewStateFn::ReplacerMaybeBox(Box::new(f))))
    }

    #[inline]
    pub fn replace_maybe_with_fn_pointer(&self, f: fn(&T) -> Option<T>) {
        self.update_by(NewState::Fn(NewStateFn::ReplacerMaybeFnPointer(f)))
    }

    #[inline]
    pub fn mutate_with_fn_box(&self, f: impl 'a + FnOnce(&mut T)) {
        self.update_by(NewState::Fn(NewStateFn::MutatorBox(Box::new(f))))
    }

    #[inline]
    pub fn mutate_with_fn_pointer(&self, f: fn(&mut T)) {
        self.update_by(NewState::Fn(NewStateFn::MutatorFnPointer(f)))
    }

    #[inline]
    pub(crate) fn get_mut(
        &mut self,
    ) -> Option<&mut (Option<std::task::Waker>, StagingStates<'a, T, N>)> {
        Rc::get_mut(&mut self.waker_and_staging_states).map(RefCell::get_mut)
    }

    /// The second argument indicates whether `RefCell::borrow_mut` is called.
    /// - [`RcStatus::Shared`] means the `Rc<RefCell<T>>` is shared, causing a runtime `RefCell::borrow_mut`.
    /// - [`RcStatus::Owned`] means there are no other Rc or Weak pointers to the same allocation.
    #[inline]
    pub(crate) fn map_mut<
        R,
        F: FnOnce(&mut (Option<std::task::Waker>, StagingStates<'a, T, N>), RcStatus) -> R,
    >(
        &mut self,
        f: F,
    ) -> R {
        if let Some(v) = self.get_mut() {
            f(v, RcStatus::Owned)
        } else {
            let mut v = self.waker_and_staging_states.borrow_mut();
            f(v.deref_mut(), RcStatus::Shared)
        }
    }
}

impl<'a, T, const N: usize> Drop for StateUpdater<'a, T, N> {
    /// When [`StateUpdater`] is dropped,
    /// it will wake up the task to notify
    /// the shared count has changed.
    fn drop(&mut self) {
        let mut waker_and_staging_states = self.waker_and_staging_states.borrow_mut();

        if let Some(waker) = waker_and_staging_states.0.take() {
            waker.wake()
        }
    }
}
