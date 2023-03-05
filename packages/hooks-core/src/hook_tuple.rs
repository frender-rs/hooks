use super::*;

#[derive(Default)]
pub struct HookTuple<T>(pub T);

macro_rules! impl_tuple {
        // ignore zero length
        () => {};
        ($($v:ident ,)+) => {
            impl_tuple! {
                - $($v ,)+
            }

            impl<$($v : HookUnmount ,)+> HookUnmount for HookTuple<($($v,)+)> {
                fn unmount(self: std::pin::Pin<&mut Self>) {
                    #[allow(non_snake_case)]
                    // SAFETY: pin projection
                    let ($($v,)+) = unsafe {
                        #[allow(non_snake_case)]
                        let HookTuple(($($v,)+)) = self.get_unchecked_mut();
                        ($(
                            ::core::pin::Pin::new_unchecked($v)
                        ,)+)
                    };
                    $(
                        <$v as HookUnmount>::unmount($v);
                    )+
                }
            }

            impl<$($v : HookPollNextUpdate ,)+> HookPollNextUpdate for HookTuple<($($v,)+)> {
                fn poll_next_update(
                    self: std::pin::Pin<&mut Self>,
                    cx: &mut std::task::Context<'_>,
                ) -> std::task::Poll<bool> {
                    #[allow(non_snake_case)]
                    // SAFETY: pin projection
                    let ($($v,)+) = unsafe {
                        #[allow(non_snake_case)]
                        let HookTuple(($($v,)+)) = self.get_unchecked_mut();
                        ($(
                            ::core::pin::Pin::new_unchecked($v)
                        ,)+)
                    };

                    #[allow(non_snake_case)]
                    #[allow(unused_variables)]
                    match ($(
                        $v.poll_next_update(cx)
                    ,)+) {
                        ($($v @ std::task::Poll::Ready(false),)+) => std::task::Poll::Ready(false),
                        #[allow(unreachable_patterns)]
                        ($($v @ (std::task::Poll::Ready(false) | std::task::Poll::Pending),)+) => std::task::Poll::Pending,
                        _ => std::task::Poll::Ready(true),
                    }
                }
            }
        };
        (
            - $v0:ident,
            $($v:ident ,)*
        ) => {
            impl_tuple! {
                $($v ,)*
            }
        };
    }

impl_tuple!(T9, T8, T7, T6, T5, T4, T3, T2, T1, T0,);
