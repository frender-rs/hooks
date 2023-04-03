use std::pin::Pin;

pub fn pin_project_or_insert_with<T>(
    mut option: Pin<&mut Option<T>>,
    f: impl FnOnce() -> T,
) -> Pin<&mut T> {
    if option.is_none() {
        option.set(Some(f()));
    }

    option.as_pin_mut().unwrap()
}
