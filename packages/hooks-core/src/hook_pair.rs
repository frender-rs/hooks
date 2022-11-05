use std::{pin::Pin, task::Poll};

use crate::HookPollNextUpdate;

pin_project_lite::pin_project! {
    #[derive(Debug, Clone, Copy)]
    pub struct HookPair<H1, H2> {
        #[pin]
        hook_1: H1,
        #[pin]
        hook_2: H2,
    }
}

impl<H1, H2> HookPair<H1, H2> {
    #[inline]
    pub fn new(hook_1: H1, hook_2: H2) -> Self {
        Self { hook_1, hook_2 }
    }

    #[inline]
    pub fn pin_project(self: Pin<&mut Self>) -> (Pin<&mut H1>, Pin<&mut H2>) {
        let this = self.project();
        (this.hook_1, this.hook_2)
    }

    #[inline]
    pub fn chain<H3>(self, hook_3: H3) -> HookPair<Self, H3> {
        HookPair {
            hook_1: self,
            hook_2: hook_3,
        }
    }
}

impl<H1: HookPollNextUpdate, H2: HookPollNextUpdate> HookPollNextUpdate for HookPair<H1, H2> {
    fn poll_next_update(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<bool> {
        let this = self.project();

        let hook_1 = this.hook_1.poll_next_update(cx);
        let hook_2 = this.hook_2.poll_next_update(cx);

        match (hook_1, hook_2) {
            // The HookChain is dynamic if either of the two hooks is dynamic
            (Poll::Ready(v1), Poll::Ready(v2)) => Poll::Ready(v1 || v2),
            _ => Poll::Pending,
        }
    }
}
