pub trait IntoEq {
    type IntoEq;
    fn into_eq(self) -> Self::IntoEq;
}

macro_rules! wrap_signal_eq {
    () => {
        type IntoEq = crate::SignalEq<Self>;

        fn into_eq(self) -> Self::IntoEq {
            crate::SignalEq(self)
        }
    };
}

#[cfg(feature = "use_shared_state")]
impl<T: PartialEq> IntoEq for crate::shared_state::SharedState<T> {
    wrap_signal_eq! {}
}

#[cfg(feature = "use_shared_state")]
impl<T: PartialEq> IntoEq for &crate::shared_state::SharedState<T> {
    wrap_signal_eq! {}
}
