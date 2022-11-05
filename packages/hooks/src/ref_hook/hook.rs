use hooks_core::HookExt;

use super::SharedRef;

pub struct RefWith<T>(Option<SharedRef<T>>);

impl<T> Unpin for RefWith<T> {}

pub struct Ref<T>(RefWith<T>);

impl<T> Unpin for Ref<T> {}

crate::utils::impl_hook! {
    impl [T] for Ref<T> {
        #[inline]
        poll_next_update(self) {
            ::core::task::Poll::Ready(false)
        }
        #[inline]
        use_hook(self, initial_value: T) -> &'hook SharedRef<T> {
            self.get_mut().0.use_hook((move || initial_value,))
        }
    }
}

crate::utils::impl_hook! {
    impl [T] for RefWith<T> {
        #[inline]
        poll_next_update(self) {
            ::core::task::Poll::Ready(false)
        }
        #[inline]
        use_hook[F: FnOnce() -> T](self, get_initial_value: F) -> &'hook SharedRef<T> {
            self.get_mut().0.get_or_insert_with(move || SharedRef::new(get_initial_value()))
        }
    }
}

#[inline]
pub fn use_ref<T>() -> Ref<T> {
    Ref(use_ref_with())
}

#[inline]
fn use_ref_with<T>() -> RefWith<T> {
    RefWith(None)
}
