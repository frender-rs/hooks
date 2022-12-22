#[macro_export]
macro_rules! __impl_rsx_no_children {
    ([$($t:tt)+]) => {
        $crate::bg::finish_builder_with!(
            [build_element]
            $($t)+
        )
    };
}

#[macro_export]
macro_rules! __impl_rsx_one {
    ([$($t:tt)+] $($children:tt)+) => {
        $crate::bg::finish_builder_with!(
            [build_element]
            $($t)+
            .children($($children)+)
        )
    };
}

#[macro_export]
macro_rules! __impl_finish_rsx {
    () => {};
}

#[macro_export]
macro_rules! __impl_rsx {
    (
        {$($v:tt)*}
        {$start_0:tt $start_1:tt $start_2:tt $start_3:tt $($start:tt)+}
        {$spath_0:tt $spath_1:tt $spath_2:tt $spath_3:tt $($spath:tt)+}
        $nested_path:tt
        $rest:tt
    ) => {
        $crate::__impl_finish_rsx!(
            $($v)*
            {
                $crate::__impl_rsx_one!(
                    $start_0
                    $crate::__impl_rsx_one!(
                        $start_1
                        $crate::__impl_rsx_one!(
                            $start_2
                            $crate::__impl_rsx_one!(
                                $start_3
                                $crate::__impl_rsx!(
                                    {$($start)+}
                                    {$($spath:tt)+}
                                    { $spath_3 { $spath_2 { $spath_1 { $spath_0 $nested_path } } } }
                                    $rest
                                )
                            )
                        )
                    )
                )
            }
        )
    };
    (
        {$($v:tt)*}
        {$start_0:tt $start_1:tt $start_2:tt $($start:tt)+}
        {$spath_0:tt $spath_1:tt $spath_2:tt $($spath:tt)+}
        $start_path:tt $rest:tt
    ) => {
        $crate::__impl_finish_rsx!(
            $($v)*
            {
                $crate::__impl_rsx_one!(
                    $start_0
                    $crate::__impl_rsx_one!(
                        $start_1
                        $crate::__impl_rsx_one!(
                            $start_2
                            $crate::__impl_rsx!(
                                {$($start)+}
                                {$($spath:tt)+}
                                { $spath_2 { $spath_1 { $spath_0 $nested_path } } }
                                $rest
                            )
                        )
                    )
                )
            }
        )
    };
    (
        {$($v:tt)*}
        {$start_0:tt $start_1:tt $($start:tt)+}
        {$spath_0:tt $spath_1:tt $($spath:tt)+}
        $start_path:tt $rest:tt
    ) => {
        $crate::__impl_finish_rsx!(
            $($v)*
            {
                $crate::__impl_rsx_one!(
                    $start_0
                    $crate::__impl_rsx_one!(
                        $start_1
                        $crate::__impl_rsx!(
                            {$($start)+}
                            {$($spath:tt)+}
                            { $spath_1 { $spath_0 $nested_path } }
                            $rest
                        )
                    )
                )
            }
        )
    };
    (
        {$($v:tt)*}
        {$start_0:tt $($start:tt)+}
        {$spath_0:tt $($spath:tt)+}
        $start_path:tt $rest:tt
    ) => {
        $crate::__impl_finish_rsx!(
            $($v)*
            {
                $crate::__impl_rsx_one!(
                    $start_0
                    $crate::__impl_rsx!(
                        {$($start)+}
                        {$($spath:tt)+}
                        { $spath_0 $nested_path }
                        $rest
                    )
                )
            }
        )
    };
    (
        {$($v:tt)*}
        {$start:tt}
        {$this_start_path:tt $start_path:tt}
        {
            </ $($(:: $ignore:vis)? $leading_path:ident $(:: $p:ident)*)? >
            $($rest:tt)*
        }
    ) => {
        $crate::__impl_rsx!(
            {
                $($v)*
                {
                    (
                        $crate::__impl_finish_rsx!(
                            $($v)*
                            {$crate::__impl_rsx_no_children!($start)}
                        ),
                        {
                            $crate::tt_matches!(
                                $this_start_path
                                [$($(:: $ignore)? $leading_path $(:: $p)*)?]
                            );
                        }
                    ).0
                }
            }
            {}
            $start_path
            {$($rest)*}
        )
    };
}

#[macro_export]
macro_rules! rsx {
    ( <
        $(:: $ignore:vis)? $leading_path:ident $(:: $p:ident)*
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
        $crate::bg::finish_builder_with!(
            [build_element]
            $(:: $ignore)? $leading_path $(:: $p)* ()
            $(
                . $field
                $(::<
                    $(
                        $($field_generics_lt)?
                        $($field_generics_ty)?
                    ),*
                >)?
                ($crate::bg::expand_a_or_b!([$($value)?][$crate::Omitted]))
            )*
        )
    };
    ( $(<
        $(:: $ignore:vis)? leading_path:ident $(:: $p:ident)*
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
            {}
            {
                $([
                    $(:: $ignore)? $leading_path $(:: $p)* ()
                    $(
                        . $field
                        $(::<
                            $(
                                $($field_generics_lt)?
                                $($field_generics_ty)?
                            ),*
                        >)?
                        ($crate::bg::expand_a_or_b!([$($value)?][$crate::Omitted]))
                    )*
                ])+
            }
            {
                $([
                    $(:: $ignore)? $leading_path $(:: $p)*
                ])+
            }
            {}
            {$($rest)*}
        )
    };
}
