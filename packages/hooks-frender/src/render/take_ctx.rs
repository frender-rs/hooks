pub trait HookContext: Sized {
    fn take_context(this: &mut Self) -> Self;
    fn with_context(this: &mut Self, f: impl FnOnce(&mut Self));
}

impl HookContext for crate::Dom {
    #[inline]
    fn take_context(this: &mut Self) -> Self {
        this.clone()
    }

    #[inline]
    fn with_context(this: &mut Self, f: impl FnOnce(&mut Self)) {
        this.with_position(f)
    }
}

impl<W: futures_io::AsyncWrite + Unpin> HookContext for crate::SsrContext<W> {
    #[inline]
    fn take_context(this: &mut Self) -> Self {
        std::mem::replace(this, Self { writer: None })
    }

    #[inline]
    fn with_context(this: &mut Self, f: impl FnOnce(&mut Self)) {
        f(this)
    }
}
