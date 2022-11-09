use std::{marker::PhantomData, pin::Pin};

pub trait GetArgs<H: ?Sized> {
    type Args;

    fn get_args(self, hook: Pin<&mut H>) -> Self::Args;
}

pub struct GetArgsFnOnce<F>(pub F);

impl<H: ?Sized, Args, F: FnOnce(Pin<&mut H>) -> Args> GetArgs<H> for GetArgsFnOnce<F> {
    type Args = Args;

    #[inline]
    fn get_args(self, hook: Pin<&mut H>) -> Args {
        (self.0)(hook)
    }
}

pub struct GetArgsMove<Args>(pub Args);

impl<H: ?Sized, Args> GetArgs<H> for GetArgsMove<Args> {
    type Args = Args;

    #[inline]
    fn get_args(self, _: Pin<&mut H>) -> Args {
        self.0
    }
}

pub struct GetArgsDefault<Args>(PhantomData<Args>);

impl<Args> Default for GetArgsDefault<Args> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<H: ?Sized, Args: Default> GetArgs<H> for GetArgsDefault<Args> {
    type Args = Args;

    #[inline]
    fn get_args(self, _: Pin<&mut H>) -> Args {
        Args::default()
    }
}
