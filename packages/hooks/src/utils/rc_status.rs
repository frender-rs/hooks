use std::{cell::RefCell, rc::Rc};

#[derive(Clone, Copy, Debug)]
pub enum RcStatus {
    /// There are no other Rc or Weak pointers to the same allocation.
    Owned,
    /// This Rc is shared
    Shared,
}

impl RcStatus {
    #[inline]
    pub fn is_owned(self) -> bool {
        match self {
            RcStatus::Owned => true,
            RcStatus::Shared => false,
        }
    }
}

/// panics if the RefCell is already borrowed as mut.
#[inline]
pub fn rc_ref_cell_borrow_mut<T, R>(
    rc: &mut Rc<RefCell<T>>,
    f: impl FnOnce(&mut T, RcStatus) -> R,
) -> R {
    if let Some(data) = Rc::get_mut(rc) {
        f(data.get_mut(), RcStatus::Owned)
    } else {
        f(&mut rc.borrow_mut(), RcStatus::Shared)
    }
}
