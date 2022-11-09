use std::pin::Pin;

use crate::NextValue;

pin_project_lite::pin_project! {
    pub struct IterHook<H> {
        #[pin]
        hook: H,
    }
}

impl<H> IterHook<H> {
    #[inline]
    pub fn new(hook: H) -> Self {
        Self { hook }
    }

    #[inline]
    pub fn into_inner(self) -> H {
        self.hook
    }

    #[inline]
    pub fn pin_project_hook(self: Pin<&mut Self>) -> Pin<&mut H> {
        self.project().hook
    }

    #[inline]
    pub fn next_value(&mut self) -> NextValue<H, ()>
    where
        Self: Unpin,
    {
        NextValue::new(Pin::new(self).pin_project_hook(), ())
    }
}
