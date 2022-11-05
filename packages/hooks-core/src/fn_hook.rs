use std::{marker::PhantomData, pin::Pin, task::Poll};

use crate::{Hook, HookBounds, HookLifetime, HookPollNextUpdate};

pin_project_lite::pin_project! {
    pub struct FnHook<Data, P, U, H> {
        #[pin]
        data: Data,
        poll_next_update: P,
        use_hook: U,
        hook_hint: PhantomData<H>,
    }
}

pub fn new_fn_hook<Args, Data, H: for<'a> HookLifetime<'a, Args>>(
    data: Data,
    poll_next_update: impl Fn(Pin<&mut Data>, &mut std::task::Context<'_>) -> Poll<bool>,
    use_hook: impl for<'hook> Fn(Pin<&'hook mut Data>, Args) -> <H as HookLifetime<'hook, Args>>::Value,
) -> FnHook<
    Data,
    impl Fn(Pin<&mut Data>, &mut std::task::Context) -> Poll<bool>,
    impl for<'hook> Fn(Pin<&'hook mut Data>, Args) -> <H as HookLifetime<'hook, Args>>::Value,
    H,
> {
    FnHook::<Data, _, _, H> {
        data,
        poll_next_update,
        use_hook,
        hook_hint: PhantomData,
    }
}

impl<Data, P, U, H: HookBounds> HookBounds for FnHook<Data, P, U, H> {
    type Bounds = H::Bounds;
}

impl<'hook, Args, Data, P, U, H: HookLifetime<'hook, Args>> HookLifetime<'hook, Args>
    for FnHook<Data, P, U, H>
{
    type Value = <H as HookLifetime<'hook, Args>>::Value;
}

impl<Data, P, U, H> HookPollNextUpdate for FnHook<Data, P, U, H>
where
    P: FnMut(Pin<&mut Data>, &mut std::task::Context<'_>) -> Poll<bool>,
{
    #[inline]
    fn poll_next_update(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<bool> {
        let this = self.project();
        (this.poll_next_update)(this.data, cx)
    }
}

impl<Args, Data, P, U, H> Hook<Args> for FnHook<Data, P, U, H>
where
    H: for<'a> HookLifetime<'a, Args>,
    P: FnMut(Pin<&mut Data>, &mut std::task::Context<'_>) -> Poll<bool>,
    U: for<'hook> FnMut(Pin<&'hook mut Data>, Args) -> <H as HookLifetime<'hook, Args>>::Value,
{
    #[inline]
    fn use_hook<'hook>(
        self: Pin<&'hook mut Self>,
        args: Args,
    ) -> <Self as HookLifetime<'hook, Args>>::Value
    where
        Self: 'hook,
    {
        let this = self.project();
        (this.use_hook)(this.data, args)
    }
}

pub fn poll_next_update_ready_false<Data>(
    _: Pin<&mut Data>,
    _: &mut std::task::Context<'_>,
) -> Poll<bool> {
    Poll::Ready(false)
}
