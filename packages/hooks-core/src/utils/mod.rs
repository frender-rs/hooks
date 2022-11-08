use std::{ops::DerefMut, pin::Pin};

/// Re-implement [`Pin::as_deref_mut`] which is unstable for now.
///
/// [`Pin::as_deref_mut`]: https://doc.rust-lang.org/std/pin/struct.Pin.html#method.as_deref_mut
#[must_use = "`self` will be dropped if the result is not used"]
#[inline(always)]
pub fn pin_as_deref_mut<P: DerefMut>(this: Pin<&mut Pin<P>>) -> Pin<&mut P::Target> {
    // SAFETY: See Pin::as_deref_mut
    unsafe { this.get_unchecked_mut() }.as_mut()
}
