/// ```
/// # use bg::{Build, builder};
/// # builder! { struct MyProps {} }
/// # fn test(v:
/// Build![MyProps]
/// # ) ->
/// // is equivalent to
/// MyProps::Data<impl ?Sized + MyProps::Types>
/// # { v }
/// ```
#[macro_export]
macro_rules! Build {
    (
        $($name:ident)? $(:: $p:ident)*
        $(< $($ty_field:tt)*)?
    ) => {
        $($name)? $(:: $p)* ::Data:: < impl ?::core::marker::Sized + $($name)? $(:: $p)* ::Types $(< $($ty_field)*)? >
    };
}

/// ```
/// # use bg::{Valid, builder};
/// # builder! { struct MyProps {} }
/// # fn test(v:
/// Valid![MyProps]
/// # ) ->
/// // is equivalent to
/// MyProps::Data<impl ?Sized + MyProps::ValidTypes>
/// # { v }
/// ```
#[macro_export]
macro_rules! Valid {
    (
        $($name:ident)? $(:: $p:ident)*
        $(< $($ty_field:tt)*)?
    ) => {
        $($name)? $(:: $p)* ::Data:: < impl ?::core::marker::Sized + $($name)? $(:: $p)* ::ValidTypes $(< $($ty_field)*)? >
    };
}
