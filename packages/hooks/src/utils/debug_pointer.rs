pub struct DebugPointerType<'a, P>(pub &'a P);

impl<'a, P: std::fmt::Pointer> std::fmt::Debug for DebugPointerType<'a, P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}@{:p}", std::any::type_name::<P>(), *self.0)
    }
}

pub struct DebugHigherKindFnPointerRefMut<'a, T: ?Sized>(pub &'a fn(&mut T));

impl<'a, T: ?Sized> std::fmt::Debug for DebugHigherKindFnPointerRefMut<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "fn(&mut {})@{}",
            std::any::type_name::<T>(),
            *self.0 as usize,
        )
    }
}
