#[derive(Clone, Copy)]
pub enum RcStatus {
    /// There are no other Rc or Weak pointers to the same allocation.
    Owned,
    /// This Rc is shared
    Shared,
}
