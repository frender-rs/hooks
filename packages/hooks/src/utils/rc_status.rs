#[derive(Clone, Copy)]
pub enum RcStatus {
    /// There are no other Rc or Weak pointers to the same allocation.
    Owned,
    /// This Rc is shared
    Shared,
}

impl RcStatus {
    #[inline]
    pub fn is_shared(self) -> bool {
        match self {
            RcStatus::Shared => true,
            RcStatus::Owned => false,
        }
    }
}
