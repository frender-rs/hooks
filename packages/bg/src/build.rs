#[macro_export]
macro_rules! valid {
    (
        $($tt:tt)*
    ) => {
        $crate::__impl_build! {
            [valid]
            $($tt)*
        }
    };
}

#[macro_export]
macro_rules! build {
    (
        $($tt:tt)*
    ) => {
        $crate::__impl_build! {
            [build]
            $($tt)*
        }
    };
}

#[macro_export]
macro_rules! finish_builder_with {
    (
        [$($path:tt)+]
        $($tt:tt)*
    ) => {
        $crate::__impl_build! {
            [$($path)+]
            $($tt)*
        }
    };
}
