pub trait PhantomTypeParam<T: ?Sized> {
    type Out: ?Sized;
}

impl<T: ?Sized, R: ?Sized> PhantomTypeParam<R> for T {
    type Out = R;
}
