pub trait IntoEq {
    type IntoEq;
    fn into_eq(self) -> Self::IntoEq;
}

#[cfg(any(feature = "use_shared_signal", feature = "use_gen_signal"))]
macro_rules! wrap_signal_eq {
    () => {
        type IntoEq = crate::SignalEq<Self>;

        fn into_eq(self) -> Self::IntoEq {
            crate::SignalEq(self)
        }
    };
}

#[cfg(feature = "use_shared_signal")]
impl<T: PartialEq> IntoEq for crate::SharedSignal<T> {
    wrap_signal_eq! {}
}

#[cfg(feature = "use_shared_signal")]
impl<T: PartialEq> IntoEq for &crate::SharedSignal<T> {
    wrap_signal_eq! {}
}

#[cfg(feature = "use_gen_signal")]
impl<T: PartialEq> IntoEq for crate::GenSignal<T> {
    wrap_signal_eq! {}
}

#[cfg(feature = "use_gen_signal")]
impl<T: PartialEq> IntoEq for crate::GenSignalHook<T> {
    wrap_signal_eq! {}
}
