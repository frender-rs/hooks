#[macro_export]
macro_rules! __impl_build {
    (
        [$finish:ident]
        $($name:ident)? $(:: $p:ident)* {
            .. $base:expr
        }
    ) => {
        $($name)? $(:: $p)* :: $finish (
            $($name)? $(:: $p)* ::Building(
                $base
            )
        )
    };
    (
        [$finish:ident]
        $($name:ident)? $(:: $p:ident)* {
            $(
                $field:ident
                $(: $field_value:expr)?
            ),*
            $(
                ,
                $(.. $base:expr)?
            )?
        }
    ) => {
        $($name)? $(:: $p)* :: $finish ({
            #[allow(unused_imports)]
            use $($name)? $(:: $p)* ::prelude::*;

            $crate::__impl_base_expr!(
                [$($($base)?)?]
                [$($name)? $(:: $p)*]
            )
            $(
                . $field (
                    $crate::expand_a_or_b!([$($field_value)?][$field])
                )
            )*
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
