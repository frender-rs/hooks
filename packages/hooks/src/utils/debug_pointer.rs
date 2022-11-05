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

pub struct DebugHigherKindFnPointerRefOutput<'a, T: ?Sized, R>(pub &'a fn(&T) -> R);

impl<'a, T: ?Sized, R> std::fmt::Debug for DebugHigherKindFnPointerRefOutput<'a, T, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "fn(&{})->{}@{}",
            std::any::type_name::<T>(),
            std::any::type_name::<R>(),
            *self.0 as usize,
        )
    }
}
