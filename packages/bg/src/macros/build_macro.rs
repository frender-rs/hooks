#[macro_export]
macro_rules! __impl_build_tolerant {
    (
        [$($path:tt)+] {
            .. $base:expr
        }
    ) => {
        $($path)+ ::Building($base)
    };
    (
        [$($path:tt)+] {
            $(
                $field_or_suggest:ident
                $($real_field:ident)?
                $(: $field_value:expr)?
            ),*
            $(
                ,
                $(.. $base:expr)?
            )?
        }
    ) => {
        $crate::__impl_base_expr!([$($($base)?)?][$($path)+])
            $(
                . $field_or_suggest (
                    $crate::expand_a_or_b!([$($field_value)?][$field_or_suggest])
                )
                $(. $real_field (_))?
            )*
    };
}

#[macro_export]
macro_rules! __check_build_fields {
    (($e:expr)
        .. $base:expr
    ) => {
        $e
    };
    (($e:expr)
        $(
            $field_or_suggest:ident
            $(: $field_value:expr)?
        ),*
        $(
            ,
            $(.. $base:expr)?
        )?
    ) => {
        $e
    };
    (($e:expr) $($err:tt)*) => {
        (
            $e,
            $crate::__report_wrong_tt!($($err)*)
        ).0
    };
}

#[macro_export]
macro_rules! __impl_build {
    (
        [$finish:ident]
        $($name:ident)? $(:: $p:ident)* {
            $($field:tt)*
        }
    ) => {
        $($name)? $(:: $p)* :: $finish ({
            #[allow(unused_imports)]
            use $($name)? $(:: $p)* ::prelude::*;

            $crate::__check_build_fields! {
                (
                    $crate::__impl_build_tolerant! (
                        [$($name)? $(:: $p)*] {
                            $($field)*
                        }
                    )
                )
                $($field)*
            }
        })
    };
    (
        [$finish:ident]
        $($name:ident)? $(:: $p:ident)* (
            $($base:expr)?
        ) $($call:tt)*
    ) => {
        $($name)? $(:: $p)* :: $finish ({
            #[allow(unused_imports)]
            use $($name)? $(:: $p)* ::prelude::*;

            $crate::__impl_base_expr!([$($base)?][$($name)? $(:: $p)*])
                $($call)*
        })
    };
}
