#[macro_export]
macro_rules! __check_build_fields {
    (($e:expr)
        .. $base:expr
    ) => {
        $e
    };
    (($e:expr)

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
macro_rules! build {
    (
        $($name:ident)? $(:: $p:ident)* {
            .. $base:expr
        }
    ) => {
        $base
    };
    (
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
    ) => {{
        #[allow(unused_imports)]
        use $($name)? $(:: $p)* ::prelude::*;

        $crate::expand_a_or_b!([$($($base)?)?][$($name)? $(:: $p)* ()])
        $(
            . $field (
                $crate::expand_a_or_b!([$($field_value)?][$field])
            )
        )*
    }};
    (
        $($name:ident)? $(:: $p:ident)* (
            $($base:expr)?
        ) $($call:tt)*
    ) => {{
        #[allow(unused_imports)]
        use $($name)? $(:: $p)* ::prelude::*;

        $crate::expand_a_or_b!([$($base)?][$($name)? $(:: $p)* ()])
            $($call)*
    }};
}
