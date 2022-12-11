#[macro_export]
macro_rules! rsx {
    ( <
        $($start_builder_path:path :)?
        $($start_builder_simple_path_0:ident)?
        $(:: $start_builder_simple_path:ident)*
        $(
            $field:ident
            $(::<
                $(
                    $($field_generics_lt:lifetime)?
                    $($field_generics_ty:ty)?
                ),* $(,)?
            >)?
            $(=$value:tt)?
        )*
        />
    ) => {
        $($start_builder_simple_path_0)?
        $(:: $start_builder_simple_path)*
        $($start_builder_path)?
        ()
            $(
                . $field
                $(::<
                    $(
                        $($field_generics_lt)?
                        $($field_generics_ty)?
                    ),*
                >)?
                ($($value)?)
            )*
            .__frender_finish_builder()
    };
    ( $(<
        $($start_builder_simple_path_0:ident)?
        $(:: $start_builder_simple_path:ident)*
        $($start_builder_path:path :)?
        $(
            $field:ident
            $(::<
                $(
                    $($field_generics_lt:lifetime)?
                    $($field_generics_ty:ty)?
                ),* $(,)?
            >)?
            $(=$value:tt)?
        )*
        >)+
        $($rest:tt)*
    ) => {
        $crate::__impl_rsx! (
            {
                $([
                    $($start_builder_simple_path_0)?
                    $(:: $start_builder_simple_path)*
                    $($start_builder_path:path)?
                    {
                        $field:ident
                        $([
                            $(
                                $($field_generics_lt:lifetime)?
                                $($field_generics_ty:ty)?
                            ),*
                        ])?
                        ($($value)?)
                    }
                ])+
            }
            <$start_builder
                $(
                    $field
                    $(::< $( $($field_generics_lt)? $($field_generics_ty)? ),* >)?
                    =$value
                )*
                children={
                    $crate::__rsx_children_and_check_enclose_tag! (
                        $start_builder
                        $($rest)*
                    )
                }
            />
        )
    };
}
