#[doc(hidden)]
#[macro_export]
macro_rules! ignore_first_tt {
    ($ignore:tt $($t:tt)*) => {
        $($t)*
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! expand_a_or_b {
    ([] [$($b:tt)*]) => {
        $($b)*
    };
    ([$($a:tt)+] [$($b:tt)*]) => {
        $($a)*
    };
}

#[macro_export]
macro_rules! __report_wrong_tt {
    () => {};
}

#[macro_export]
macro_rules! __impl_props_types_builder_trait_item {
    ({$($all_fields:ident),*} { non_generic }
        $(#[$($fn_attr:tt)*])*
        $field_name:ident
        $([ $($builder_generics:tt)* ])?
        ($($field_builder_inputs:tt)*)
            -> $field_builder_output:ty
            = type($initial_ty:ty)
             value($initial_value:expr)
            $field_builder_impl:block
    ) => {
        $(#[$($fn_attr)*])*
        fn $field_name
            $(< $($builder_generics)* >)?
            (mut self, $($field_builder_inputs)*) -> Self {
                self.as_mut_taken().$field_name = $field_builder_impl;
                self
            }
    };
    ({$($all_fields:ident),*} { generic $opts:tt }
        $(#[$($fn_attr:tt)*])*
        $field_name:ident
        $([ $($builder_generics:tt)* ])?
        ($($field_builder_inputs:tt)*)
            -> $field_builder_output:ty
            = type($initial_ty:ty)
             value($initial_value:expr)
            $field_builder_impl:block
    ) => {
        $(#[$($fn_attr)*])*
        fn $field_name
            $(< $($builder_generics)* >)?
            (self, $($field_builder_inputs)*) ->
                <<Self as $crate::builder::TakeData<Data<TypesNormalize![[Types] Self]>>>::Left as $crate::builder::JoinData<Data<
                    TypesOverwriteOneField![[Types] ..Self, $field_name = $field_builder_output ]
                >>>::Joined
            where <Self as $crate::builder::TakeData<Data<TypesNormalize![[Types] Self]>>>::Left : $crate::builder::JoinData<Data<
                TypesOverwriteOneField![[Types] ..Self, $field_name = $field_builder_output ]
            >> {
                let _builder_impl_field_new_value = $field_builder_impl;
                let (
                    _builder_impl_field_left,
                    Data {
                        __phantom_type_defs: _,
                    $(
                        $all_fields,
                    )*
                    }
                ) = self.take_data();

                let $field_name = _builder_impl_field_new_value;

                $crate::builder::JoinData::<Data<TypesOverwriteOneField![[Types] ..Self, $field_name = $field_builder_output ]>>::join_data(
                    _builder_impl_field_left,
                    Data {
                        __phantom_type_defs: ::core::marker::PhantomData,
                        $(
                            $all_fields,
                        )*
                    },
                )
            }
    };
    ({$($all_fields:ident),*} { inherit { path($($inherit_path:tt)*) } }
        $(#[$($fn_attr:tt)*])*
        $field_name:ident
        $([ $($builder_generics:tt)* ])?
        ($($field_builder_inputs:tt)*)
            -> $field_builder_output:ty
            = type($initial_ty:ty)
            value($initial_value:expr)
            $field_builder_impl:block
    ) => {
        $(#[$($fn_attr)*])*
        fn $field_name
            $(< $($builder_generics)* >)?
            (self, $($field_builder_inputs)*) ->
                <<Self as $crate::builder::TakeData<Data<TypesNormalize![[Types] Self]>>>::Left as $crate::builder::JoinData<Data<
                    TypesOverwriteOneField![[Types] ..Self, $field_name = $field_builder_output ]
                >>>::Joined
            where <Self as $crate::builder::TakeData<Data<TypesNormalize![[Types] Self]>>>::Left : $crate::builder::JoinData<Data<
                TypesOverwriteOneField![[Types] ..Self, $field_name = $field_builder_output ]
            >> {
                let _builder_impl_field_new_value = $field_builder_impl;
                let (
                    _builder_impl_field_left,
                    Data {
                        __phantom_type_defs: _,
                    $(
                        $all_fields,
                    )*
                    }
                ) = self.take_data();

                let $field_name = _builder_impl_field_new_value;

                $crate::builder::JoinData::<Data<TypesOverwriteOneField![[Types] ..Self, $field_name = $field_builder_output ]>>::join_data(
                    _builder_impl_field_left,
                    Data {
                        __phantom_type_defs: ::core::marker::PhantomData,
                        $(
                            $all_fields,
                        )*
                    },
                )
            }
    };
}

#[macro_export]
macro_rules! __impl_props_types_trait_item {
    ({$($all_fields:ident),*} { non_generic }
        $(#[$($fn_attr:tt)*])*
        $field_name:ident
        $([ $($builder_generics:tt)* ])?
        ($($field_builder_inputs:tt)*)
            -> $field_builder_output:ty
            = type($initial_ty:ty)
             value($initial_value:expr)
            $field_builder_impl:block
    ) => {};
    ({$($all_fields:ident),*} { generic $opts:tt }
        $(#[$($fn_attr:tt)*])*
        $field_name:ident
        $([ $($builder_generics:tt)* ])?
        ($($field_builder_inputs:tt)*)
            -> $field_builder_output:ty
            = type($initial_ty:ty)
             value($initial_value:expr)
            $field_builder_impl:block
    ) => {
        #[allow(non_camel_case_types)]
        type $field_name;
    };
    ({$($all_fields:ident),*} { inherit { path($($inherit_path:tt)*) } }
        $(#[$($fn_attr:tt)*])*
        $field_name:ident
        $([ $($builder_generics:tt)* ])?
        ($($field_builder_inputs:tt)*)
            -> $field_builder_output:ty
            = type($initial_ty:ty)
            value($initial_value:expr)
            $field_builder_impl:block
    ) => {
        #[allow(non_camel_case_types)]
        type $field_name;
    };
}

#[macro_export]
macro_rules! __impl_props_types_impl_types_for_data_field {
    ({$base:ty as $trait_name:path} { non_generic }
        $(#[$($fn_attr:tt)*])*
        $field_name:ident
        $([ $($builder_generics:tt)* ])?
        ($($field_builder_inputs:tt)*)
            -> $field_builder_output:ty
            = type($initial_ty:ty)
            value($initial_value:expr)
            $field_builder_impl:block
    ) => {};
    ({$base:ty as $trait_name:path} $metadata:tt
        $(#[$($fn_attr:tt)*])*
        $field_name:ident
        $([ $($builder_generics:tt)* ])?
        ($($field_builder_inputs:tt)*)
            -> $field_builder_output:ty
            = type($initial_ty:ty)
            value($initial_value:expr)
            $field_builder_impl:block
    ) => {
        type $field_name = <$base as $trait_name>::$field_name;
    };
}

#[macro_export]
macro_rules! __impl_props_types_field_ty {
    ({$base:ty as $trait_types:path} { generic {$($meta_generic:tt)*} }
        $(#[$($fn_attr:tt)*])*
        $field_name:ident
        $([ $($builder_generics:tt)* ])?
        ($($field_builder_inputs:tt)*)
            -> $field_builder_output:ty
            = type($initial_ty:ty)
             value($initial_value:expr)
            $field_builder_impl:block
    ) => {
        <$base as $trait_types>::$field_name
    };
    ({$base:ty as $trait_types:path} { non_generic }
        $(#[$($fn_attr:tt)*])*
        $field_name:ident
        $([ $($builder_generics:tt)* ])?
        ($($field_builder_inputs:tt)*)
            -> $field_builder_output:ty
            = type($initial_ty:ty)
            value($initial_value:expr)
            $field_builder_impl:block
    ) => {
        $initial_ty
    };
    ({$base:ty as $trait_types:path} { inherit { path($($inherit_path:tt)*) } }
        $(#[$($fn_attr:tt)*])*
        $field_name:ident
        $([ $($builder_generics:tt)* ])?
        ($($field_builder_inputs:tt)*)
            -> $field_builder_output:ty
            = type($initial_ty:ty)
            value($initial_value:expr)
            $field_builder_impl:block
    ) => {
        <$base as $trait_types>::$field_name
    };
}

#[macro_export]
macro_rules! __impl_props_types_field_initial_ty {
    ({} $metadata:tt
        $(#[$($fn_attr:tt)*])*
        $field_name:ident
        $([ $($builder_generics:tt)* ])?
        ($($field_builder_inputs:tt)*)
            -> $field_builder_output:ty
            = type($initial_ty:ty)
             value($initial_value:expr)
            $field_builder_impl:block
    ) => {
        $initial_ty
    };
}

#[macro_export]
macro_rules! __impl_props_field_declaration_normalize {
    // builder without default value
    ( [$($macro_path:tt)+] $common_data:tt [
        $(#[$($fn_attr:tt)*])*
        $field_name:ident

        $([ $($field_modifiers_or_builder_generics:tt)* ])?
        ($($field_builder_inputs:tt)*)
            -> $field_builder_output:ty
            $field_builder_impl:block
    ]) => {
        $($macro_path)+! { $common_data
            { generic { valid( = $field_builder_output) } }

            $(#[$($fn_attr)*])*
            $field_name
            $([ $($field_modifiers_or_builder_generics)* ])?
            ($($field_builder_inputs)*)
                -> $field_builder_output
                = type($crate::builder::Unspecified::<$field_builder_output>)
                  value($crate::builder::Unspecified)
                $field_builder_impl

        }
    };
    // builder with default value
    ( [$($macro_path:tt)+] $common_data:tt [
        $(#[$($fn_attr:tt)*])*
        $field_name:ident

        $([ $($field_modifiers_or_builder_generics:tt)* ])?
        ($($field_builder_inputs:tt)*)
            -> $field_builder_output:ty
            = $field_builder_default_output_value:expr =>
            $field_builder_impl:block
    ]) => {
        $($macro_path)+! { $common_data
            { non_generic }

            $(#[$($fn_attr)*])*
            $field_name
            $([ $($field_modifiers_or_builder_generics)* ])?
            ($($field_builder_inputs)*)
                -> $field_builder_output
                = type($field_builder_output)
                  value($field_builder_default_output_value)
                $field_builder_impl

        }
    };
    // field: Type
    ( [$($macro_path:tt)+] $common_data:tt [
        $(#[$($fn_attr:tt)*])*
        $field_name:ident
        : $field_ty:ty
    ]) => {
        $($macro_path)+! { $common_data
            { generic { valid( = $field_ty ) }}

            $(#[$($fn_attr)*])*
            $field_name
            (new_value: $field_ty)
                -> $field_ty
                = type($crate::builder::Unspecified<$field_ty>)
                  value($crate::builder::Unspecified)
                { new_value }
        }
    };
    // field: Type = get_default_value()
    ( [$($macro_path:tt)+] $common_data:tt [
        $(#[$($fn_attr:tt)*])*
        $field_name:ident
        : $field_ty:ty = $field_default_value:expr
    ]) => {
        $($macro_path)+! { $common_data
            { non_generic }

            $(#[$($fn_attr)*])*
            $field_name
            (new_value: $field_ty)
                -> $field_ty
                = type($field_ty)
                  value($field_default_value)
                { new_value }
        }
    };
    // field[inherit Type] $( : InitialType = get_initial_value() )?
    ( [$($macro_path:tt)+] $common_data:tt [
        $(#[$($fn_attr:tt)*])*
        $field_name:ident

        [ inherit $($inherit_path:tt)* ]
        $(: $field_ty:ty = $field_default_value:expr)?
    ]) => {
        $($macro_path)+! { $common_data
            { inherit { path($($inherit_path)*) } }
            $(#[$($fn_attr)*])*
            $field_name
            [ NewTypeDefs: ?::core::marker::Sized + $($inherit_path)* :: Types ]
            (new_value: $($inherit_path)* :: Data<NewTypeDefs> )
                -> $($inherit_path)* :: Data<NewTypeDefs>
                = type($($inherit_path)* :: DataInitial)
                 value($($inherit_path)* ())
                { new_value }
        }
    };
    // field[borrow? Type] $( : InitialType = get_initial_value() )?
    //      will be normalized to:
    // field[impl MaybeBorrow<Type>] $( : InitialType = get_initial_value() )?
    ( [$($macro_path:tt)+] $common_data:tt [
        $(#[$($fn_attr:tt)*])*
        $field_name:ident

        [ borrow? $for_ty:ty ]
        $(: $field_ty:ty = $field_default_value:expr)?
    ]) => {
        $crate::__impl_props_field_declaration_normalize! { [$($macro_path)+] $common_data [
            $(#[$($fn_attr)*])*
            $field_name
            [impl $crate::builder::MaybeBorrow<$for_ty>]
            $(: $field_ty = $field_default_value)?
        ]}
    };
    // field[? Type] $( : InitialType = get_initial_value() )?
    //      will be normalized to:
    // field[impl Maybe<Type>] $( : InitialType = get_initial_value() )?
    ( [$($macro_path:tt)+] $common_data:tt [
        $(#[$($fn_attr:tt)*])*
        $field_name:ident

        [ ? $for_ty:ty ]
        $(: $field_ty:ty = $field_default_value:expr)?
    ]) => {
        $crate::__impl_props_field_declaration_normalize! { [$($macro_path)+] $common_data [
            $(#[$($fn_attr)*])*
            $field_name
            [impl $crate::builder::Maybe<$for_ty>]
            $(: $field_ty = $field_default_value)?
        ]}
    };
    // field[impl Trait]
    //      will be normalized to:
    // field[impl Trait]: UnspecifiedField<tag::field> = UnspecifiedField
    ( [$($macro_path:tt)+] $common_data:tt [
        $(#[$($fn_attr:tt)*])*
        $field_name:ident

        [ impl $($field_bound:tt)* ]
    ]) => {
        $crate::__impl_props_field_declaration_normalize! {
            [$($macro_path)+] $common_data { tag() } [
                $(#[$($fn_attr)*])*
                $field_name

                [ impl $($field_bound)* ]: $crate::builder::UnspecifiedField<builder_impl_tag::$field_name> = $crate::builder::UnspecifiedField
            ]
        }
    };
    // field[impl Trait]: Type = get_default_value()
    ( [$($macro_path:tt)+] $common_data:tt $({ $($other_meta:tt)* })? [
        $(#[$($fn_attr:tt)*])*
        $field_name:ident

        [ impl $($field_bound:tt)* ]
        : $field_ty:ty = $field_default_value:expr
    ]) => {
        $($macro_path)+! { $common_data
            { generic { valid( : $($field_bound)* ) $($($other_meta)*)? } }

            $(#[$($fn_attr)*])*
            $field_name
            [ NewValue: $($field_bound)* ]
            (new_value: NewValue)
                -> NewValue
                = type($field_ty)
                  value($field_default_value)
                { new_value }
        }
    };
    // error
    ( [$($macro_path:tt)+] $common_data:tt [
        $($err:tt)*
    ]) => {
        // $crate::__report_wrong_tt! {$($err)*}
        ::core::compile_error!{::core::concat!(
            "invalid field declaration: `",
            ::core::stringify!($($err)*),
            "`"
        )}
    };
}

#[macro_export]
macro_rules! __impl_props_field_declaration_normalize_iter {
    ( [$macro_path:path] $common_data:tt $($field_declaration:tt)*) => {
        $(
            $crate::__impl_props_field_declaration_normalize! {
                [$macro_path] $common_data $field_declaration
            }
        )*
    };
}

#[macro_export]
macro_rules! __impl_props_types_macro_rules {
    ( { $dollar:tt }
        $([
            $field_name:ident

            $([ $($field_modifier:tt)* ] $(: $(= $initial_v_mod:expr)? , $initial_ty_mod:ty )? ;)?
            $(
                = $field_builder_default_output_value:expr =>
                ($($field_builder_inputs:tt)*)
                    -> $field_builder_output:ty
                    $field_builder_impl:block
                $([ $($builder_generics:tt)* ])? ;
            )?
            $(
                ($($generic_field_builder_inputs:tt)*)
                    -> $generic_field_builder_output:ty
                    $generic_field_builder_impl:block
                $([ $($builder_generics_generic:tt)* ])? ;
            )?
            $(
                : = $field_default_value:expr , $field_ty:ty;
            )?
            $(  : , $generic_field_ty:ty;  )?
        ])*
    ) => {
        macro_rules! use_a_if_field_name_match {
            $(
                $( ($ignore:tt $field_name $field_name [$a:ty][$b:ty]) => { $crate::ignore_first_tt!{ {$($initial_ty_mod)?} $a } }; )?
                $( ($ignore:tt $field_name $field_name [$a:ty][$b:ty]) => { $crate::ignore_first_tt!{ {$generic_field_builder_output} $a } }; )?
                $( ($ignore:tt $field_name $field_name [$a:ty][$b:ty]) => { $crate::ignore_first_tt!{ {$generic_field_ty} $a } }; )?
            )*
            ($ignore:tt $name1:ident $name2:ident [$a:ty][$b:ty]) => {
                $b
            };
        }

        macro_rules! TypesNormalize {
            (
                [$dollar ($trait_name:tt)*]
                $base:ty
            ) => {
                dyn $dollar ($trait_name)* <$(
                    $( $field_name = $crate::ignore_first_tt![ {$($initial_ty_mod)?} <$base as Types>::$field_name ] , )?
                    $( $field_name = $crate::ignore_first_tt![ {$generic_field_builder_output} <$base as Types>::$field_name ] , )?
                    $( $field_name = $crate::ignore_first_tt![ {$generic_field_ty    } <$base as Types>::$field_name ] , )?
                )*>
            };
        }

        macro_rules! TypesOverwriteOneField {
            (
                [$dollar ($trait_name:tt)*]
                ..$inherit:ty,
                $overwrite_field:ident = $overwrite_field_ty:ty
            ) => {
                dyn $dollar ($trait_name)* <$(
                    $( $field_name = use_a_if_field_name_match![ {$($initial_ty_mod)?} $field_name $overwrite_field [$overwrite_field_ty] [<$inherit as $dollar ($trait_name)*>::$field_name] ], )?
                    $( $field_name = use_a_if_field_name_match![ {$generic_field_builder_output} $field_name $overwrite_field [$overwrite_field_ty] [<$inherit as $dollar ($trait_name)*>::$field_name] ], )?
                    $( $field_name = use_a_if_field_name_match![ {$generic_field_ty} $field_name $overwrite_field [$overwrite_field_ty] [<$inherit as $dollar ($trait_name)*>::$field_name] ], )?
                )*>
            };
        }
    };
}

#[macro_export]
macro_rules! __impl_props_types_field_initial_ty_iter {
    (
        [$($full_prefix:tt)*][$($full_suffix:tt)*]
        $([
            $field_name:ident

            $([ ?    $($field_modifier_maybe:tt)* ] $(: $(= $initial_v_maybe:expr)? , $initial_ty_maybe:ty )? ;)?
            $([ impl $($field_modifier_impl:tt)*  ] $(: $(= $initial_v_impl:expr )? , $initial_ty_impl:ty  )? ;)?
            $([ borrow? $($field_modifier_bm:tt)* ] $(: $(= $initial_v_bm:expr   )? , $initial_ty_bm:ty    )? ;)?
            $([ inherit $($field_modifier_ih:tt)* ] $(: $(= $initial_v_ih:expr   )? , $initial_ty_ih:ty    )? ;)?
            $(
                = $field_builder_default_output_value:expr =>
                ($($field_builder_inputs:tt)*)
                    -> $field_builder_output:ty
                    $field_builder_impl:block
                $([ $($builder_generics:tt)* ])? ;
            )?
            $(
                ($($generic_field_builder_inputs:tt)*)
                    -> $generic_field_builder_output:ty
                    $generic_field_builder_impl:block
                $([ $($builder_generics_generic:tt)* ])? ;
            )?
            $(
                : = $field_default_value:expr , $field_ty:ty;
            )?
            $(  : , $generic_field_ty:ty;  )?
        ])*
    ) => {
        $($full_prefix)*
        $(
            $( $field_name = $crate::expand_a_or_b![ [$($initial_ty_maybe)?] [$crate::builder::Unspecified<$($field_modifier_maybe)*>] ], )?
            $( $field_name = $crate::expand_a_or_b![ [$($initial_ty_impl)? ] [$crate::builder::UnspecifiedField<builder_impl_tag::$field_name>]], )?
            $( $field_name = $crate::expand_a_or_b![ [$($initial_ty_bm  )? ] [$crate::builder::Unspecified<$($field_modifier_bm   )*>] ], )?
            $( $field_name = $crate::expand_a_or_b![ [$($initial_ty_ih  )? ] [$($field_modifier_ih)* :: DataInitial] ], )?
            $( $field_name = $crate::builder::Unspecified<$generic_field_builder_output>, )?
            $( $field_name = $crate::builder::Unspecified<$generic_field_ty>, )?
        )*
        $($full_suffix)*
    };
}

#[macro_export]
macro_rules! __impl_props_types_valid_trait {
    (
        $([
            $field_name:ident

            $([ ?    $($field_modifier_maybe:tt)* ] $(: $(= $initial_v_maybe:expr)? , $initial_ty_maybe:ty )? ;)?
            $([ impl $($field_modifier_impl:tt)*  ] $(: $(= $initial_v_impl:expr )? , $initial_ty_impl:ty  )? ;)?
            $([ borrow? $($field_modifier_bm:tt)* ] $(: $(= $initial_v_bm:expr   )? , $initial_ty_bm:ty    )? ;)?
            $([ inherit $($field_modifier_ih:tt)* ] $(: $(= $initial_v_ih:expr   )? , $initial_ty_ih:ty    )? ;)?
            $(
                = $field_builder_default_output_value:expr =>
                ($($field_builder_inputs:tt)*)
                    -> $field_builder_output:ty
                    $field_builder_impl:block
                $([ $($builder_generics:tt)* ])? ;
            )?
            $(
                ($($generic_field_builder_inputs:tt)*)
                    -> $generic_field_builder_output:ty
                    $generic_field_builder_impl:block
                $([ $($builder_generics_generic:tt)* ])? ;
            )?
            $(
                : = $field_default_value:expr , $field_ty:ty;
            )?
            $(  : , $generic_field_ty:ty;  )?
        ])*
    ) => {
        pub trait ValidTypes: Types<
            $(
                $( $field_name = $generic_field_builder_output, )?
                $( $field_name = $generic_field_ty, )?
                $(
                    $field_name = $crate::ignore_first_tt![{$($field_modifier_maybe)*} <Self as ValidTypes>::$field_name],
                )?
                $(
                    $field_name = $crate::ignore_first_tt![{$($field_modifier_impl )*} <Self as ValidTypes>::$field_name],
                )?
                $(
                    $field_name = $crate::ignore_first_tt![{$($field_modifier_bm)*   } <Self as ValidTypes>::$field_name],
                )?
                $(
                    $field_name = $($field_modifier_ih)* ::Data< <Self as ValidTypes>::$field_name >,
                )?
            )*
        >
        {
            #![allow(non_camel_case_types)]

            $(
                $( type $field_name : $crate::builder::Maybe<$($field_modifier_maybe)*>; )?

                $( type $field_name : $($field_modifier_impl)*; )?

                $( type $field_name : $crate::builder::MaybeBorrow<$($field_modifier_bm)*>; )?

                $( type $field_name : ?::core::marker::Sized + $($field_modifier_ih)* :: ValidTypes; )?
            )*
        }

        impl<
            T: ?::core::marker::Sized,
            $(
                $( #[allow(non_camel_case_types)] $field_name : ?::core::marker::Sized + $($field_modifier_ih)* :: ValidTypes )?
            )*
        > ValidTypes for T where T: Types<
            $(
                $( $field_name = $generic_field_builder_output, )?
                $( $field_name = $generic_field_ty, )?
                $(
                    $field_name = $($field_modifier_ih)* ::Data< $field_name >,
                )?
            )*
        >,
        $(
            $(
                <T as Types>::$field_name : $crate::builder::Maybe<$($field_modifier_maybe)*>,
            )?
            $( <T as Types>::$field_name: $($field_modifier_impl)*, )?
            $(
                <T as Types>::$field_name : $crate::builder::MaybeBorrow<$($field_modifier_bm)*>,
            )?
        )*
    {
        $(
            $(
                #[doc = stringify!($($field_modifier_maybe)*)]
                type $field_name = <T as Types>::$field_name;
            )?

            $(
                #[doc = stringify!($($field_modifier_impl)*)]
                type $field_name = <T as Types>::$field_name;
            )?

            $(
                #[doc = stringify!($($field_modifier_bm)*)]
                type $field_name = <T as Types>::$field_name;
            )?

            $(
                #[doc = stringify!($($field_modifier_ih)*)]
                type $field_name = $field_name;
            )?
        )*
    }

    };
}

#[macro_export]
macro_rules! __impl_props_types_field_initial_value {
    ({} $metadata:tt
        $(#[$($fn_attr:tt)*])*
        $field_name:ident
        $([ $($builder_generics:tt)* ])?
        ($($field_builder_inputs:tt)*)
            -> $field_builder_output:ty
            = type($initial_ty:ty)
             value($initial_value:expr)
            $field_builder_impl:block
    ) => {
        $initial_value
    };
}

#[macro_export]
macro_rules! __impl_props_field_tag {
    ({} { generic { valid $valid_meta:tt tag() }}
        $(#[$($fn_attr:tt)*])*
        $field_name:ident
        $([ $($builder_generics:tt)* ])?
        ($($field_builder_inputs:tt)*)
            -> $field_builder_output:ty
            = type($initial_ty:ty)
             value($initial_value:expr)
            $field_builder_impl:block
    ) => {
        #[allow(non_camel_case_types)]
        pub enum $field_name {}
    };
    ({} { inherit $opts:tt }
        $(#[$($fn_attr:tt)*])*
        $field_name:ident
        $([ $($builder_generics:tt)* ])?
        ($($field_builder_inputs:tt)*)
            -> $field_builder_output:ty
            = type($initial_ty:ty)
            value($initial_value:expr)
            $field_builder_impl:block
    ) => {
        #[allow(non_camel_case_types)]
        pub enum $field_name {}
    };
    ({} $($other:tt)*) => {};
}

#[macro_export]
macro_rules! __impl_props_prelude {
    ({} { inherit { path($($inherit_path:tt)*) } }
        $(#[$($fn_attr:tt)*])*
        $field_name:ident
        $([ $($builder_generics:tt)* ])?
        ($($field_builder_inputs:tt)*)
            -> $field_builder_output:ty
            = type($initial_ty:ty)
            value($initial_value:expr)
            $field_builder_impl:block
    ) => {
        pub use $($inherit_path)* ::prelude::*;
    };
    ({} $($other:tt)*) => {
    };
}

#[macro_export]
macro_rules! __impl_props_inherit_take_data {
    ({ $($all_fields:ident),* } { inherit { path($($inherit_path:tt)*) } }
        $(#[$($fn_attr:tt)*])*
        $field_name:ident
        $([ $($builder_generics:tt)* ])?
        ($($field_builder_inputs:tt)*)
            -> $field_builder_output:ty
            = type($initial_ty:ty)
            value($initial_value:expr)
            $field_builder_impl:block
    ) => {
        impl<
            NewInheritedTypeDefs: ?Sized + $($inherit_path)*::Types,
            TypeDefs: ?Sized + super::Types,
        > $crate::builder::JoinData<$($inherit_path)*::Data<NewInheritedTypeDefs>> for super::Data<TypeDefs> {
            type Joined = super::Data<
                TypesOverwriteOneField![[super::Types]
                    ..TypeDefs,
                    $field_name = $($inherit_path)*::Data<NewInheritedTypeDefs>
                ]
            >;

            #[inline]
            fn join_data(self, __builder_impl_v_data: $($inherit_path)*::Data<NewInheritedTypeDefs>) -> Self::Joined {
                let Self {
                    __phantom_type_defs,
                    $($all_fields),*
                } = self;
                let _ = $field_name;
                let $field_name = __builder_impl_v_data;
                super::Data {
                    __phantom_type_defs: ::core::marker::PhantomData,
                    $($all_fields),*
                }
            }
        }

        impl<
            InheritedTypeDefs: ?Sized + $($inherit_path)*::Types,
            TypeDefs: ?Sized + super::Types< $field_name = $($inherit_path)*::Data<InheritedTypeDefs> >,
        > $($inherit_path)*::Inherit for super::Data<TypeDefs> {
            type InheritedTypeDefs = InheritedTypeDefs;
        }

        impl<
            InheritedTypeDefs: ?Sized + $($inherit_path)*::Types,
            TypeDefs: ?Sized + super::Types< $field_name = $($inherit_path)*::Data<InheritedTypeDefs> >,
        > $crate::builder::TakeData<
            $($inherit_path)*::Data<InheritedTypeDefs>
        > for super::Data<TypeDefs> {
            type Left = super::Data<TypesOverwriteOneField![[super::Types]
                ..TypeDefs,
                $field_name = $crate::builder::UnspecifiedField<super::builder_impl_tag:: $field_name>
            ]>;

            #[inline]
            fn take_data(self) -> (Self::Left, $($inherit_path)*::Data<InheritedTypeDefs>) {
                let Self {
                    __phantom_type_defs,
                    $($all_fields),*
                } = self;
                let __builder_impl_v_taken = $field_name;
                let $field_name = $crate::builder::UnspecifiedField;
                (
                    super::Data {
                        __phantom_type_defs: ::core::marker::PhantomData,
                        $($all_fields),*
                    }
                    ,
                    __builder_impl_v_taken
                )
            }

            #[inline]
            fn as_mut_taken(&mut self) -> &mut $($inherit_path)*::Data<InheritedTypeDefs> {
                &mut self. $field_name
            }
        }
    };
    ({ $($all_fields:ident),* } $($other:tt)*) => {};
}

#[macro_export]
macro_rules! def_props {
    (
        $(#![$($data_struct_attr:tt)*])*
        $(#[$($mod_and_fn_attr:tt)*])*
        $vis:vis struct $name:ident
        {
            $(
                $(#[$($fn_attr:tt)*])*
                $field_name:ident

                $([ $($field_modifiers_or_builder_generics:tt)* ])?
                $(
                    ($($field_builder_inputs:tt)*)
                        -> $field_builder_output:ty
                        $(= $field_builder_default_output_value:expr =>)?
                        $field_builder_impl:block
                )?

                $(
                    : $field_ty:ty $( = $field_default_value:expr)?
                )?
            ),* $(,)?
        }
    ) => {
        $(#[$($mod_and_fn_attr)*])*
        #[allow(non_snake_case)]
        $vis mod $name {
            use super::*;

            $crate::__impl_props_types_macro_rules! {
                {$}
                $([
                    $field_name

                    $(
                        $(= $field_builder_default_output_value =>)?
                        ($($field_builder_inputs)*)
                            -> $field_builder_output
                            $field_builder_impl
                    )?

                    $([ $($field_modifiers_or_builder_generics)* ])?

                    $(
                        : $( = $field_default_value)? , $field_ty
                    )?
                    ;
                ])*
            }

            pub mod builder_impl_tag {
                use super::super::*;

                $crate::__impl_props_field_declaration_normalize_iter! {
                    [$crate::__impl_props_field_tag]
                    {}
                    $([
                        // $(#[$($fn_attr)*])* // ignore attributes
                        $field_name

                        $([ $($field_modifiers_or_builder_generics)* ])?
                        $(
                            ($($field_builder_inputs)*)
                                -> $field_builder_output
                                $(= $field_builder_default_output_value =>)?
                                $field_builder_impl
                        )?

                        $(
                            : $field_ty $( = $field_default_value)?
                        )?
                    ])*
                }
            }

            pub trait Types {
                $crate::__impl_props_field_declaration_normalize_iter! {
                    [$crate::__impl_props_types_trait_item]
                    {}
                    $([
                        $(#[$($fn_attr)*])*
                        $field_name

                        $([ $($field_modifiers_or_builder_generics)* ])?
                        $(
                            ($($field_builder_inputs)*)
                                -> $field_builder_output
                                $(= $field_builder_default_output_value =>)?
                                $field_builder_impl
                        )?

                        $(
                            : $field_ty $( = $field_default_value)?
                        )?
                    ])*
                }
            }

            pub trait Builder: Sized + Types + $crate::builder::TakeData<Data<TypesNormalize![[Types] Self]>> {
                $crate::__impl_props_field_declaration_normalize_iter! {
                    [$crate::__impl_props_types_builder_trait_item]
                    {$($field_name),*}
                    $([
                        $(#[$($fn_attr)*])*
                        $field_name

                        $([ $($field_modifiers_or_builder_generics)* ])?
                        $(
                            ($($field_builder_inputs)*)
                                -> $field_builder_output
                                $(= $field_builder_default_output_value =>)?
                                $field_builder_impl
                        )?

                        $(
                            : $field_ty $( = $field_default_value)?
                        )?
                    ])*
                }
            }

            impl<B> Builder for B
                where B: Sized + Types + $crate::builder::TakeData<Data<TypesNormalize![[Types] B]>> {}

            pub mod builder_impl_data {
                use super::super::*;

                $(#[$($data_struct_attr)*])*
                pub struct $name <TypeDefs: ?::core::marker::Sized + super::Types> {
                    pub(super) __phantom_type_defs: ::core::marker::PhantomData<*const TypeDefs>,
                    $(
                    pub $field_name : $crate::__impl_props_field_declaration_normalize! {
                        [$crate::__impl_props_types_field_ty] {TypeDefs as super::Types} [
                            $(#[$($fn_attr)*])*
                            $field_name

                            $([ $($field_modifiers_or_builder_generics)* ])?
                            $(
                                ($($field_builder_inputs)*)
                                    -> $field_builder_output
                                    $(= $field_builder_default_output_value =>)?
                                    $field_builder_impl
                            )?

                            $(
                                : $field_ty $( = $field_default_value)?
                            )?
                        ]
                    },
                    )*
                }
            }

            pub mod builder_impl_fn_uninitialized {
                use super::super::*;

                $(#[$($mod_and_fn_attr)*])*
                #[inline]
                #[allow(non_snake_case)]
                pub fn $name() -> super::DataInitial {
                    super::builder_impl_data::$name {
                        __phantom_type_defs: ::core::marker::PhantomData,
                        $(
                        $field_name : $crate::__impl_props_field_declaration_normalize! {
                            [$crate::__impl_props_types_field_initial_value] {} [
                                $(#[$($fn_attr)*])*
                                $field_name

                                $([ $($field_modifiers_or_builder_generics)* ])?
                                $(
                                    ($($field_builder_inputs)*)
                                        -> $field_builder_output
                                        $(= $field_builder_default_output_value =>)?
                                        $field_builder_impl
                                )?

                                $(
                                    : $field_ty $( = $field_default_value)?
                                )?
                            ]
                        },
                        )*
                    }
                }
            }

            pub use self::builder_impl_data::$name as Data;

            pub trait Inherit {
                type InheritedTypeDefs: ?::core::marker::Sized;
            }

            impl<
                TypeDefs: ?::core::marker::Sized + Types,
            > Inherit for Data<TypeDefs> {
                type InheritedTypeDefs = TypeDefs;
            }

            impl<
                TypeDefs: ?::core::marker::Sized + Types,
                T: Inherit<InheritedTypeDefs = TypeDefs>,
            > Types for T {
                $crate::__impl_props_field_declaration_normalize_iter! {
                    [$crate::__impl_props_types_impl_types_for_data_field]
                    {TypeDefs as Types}
                    $([
                        $(#[$($fn_attr)*])*
                        $field_name

                        $([ $($field_modifiers_or_builder_generics)* ])?
                        $(
                            ($($field_builder_inputs)*)
                                -> $field_builder_output
                                $(= $field_builder_default_output_value =>)?
                                $field_builder_impl
                        )?

                        $(
                            : $field_ty $( = $field_default_value)?
                        )?
                    ])*
                }
            }

            pub type TypesInitial = $crate::__impl_props_types_field_initial_ty_iter! {
                [dyn Types<][>]
                $([
                    $field_name

                    $(
                        $(= $field_builder_default_output_value =>)?
                        ($($field_builder_inputs)*)
                            -> $field_builder_output
                            $field_builder_impl
                    )?

                    $([ $($field_modifiers_or_builder_generics)* ])?

                    $(
                        : $( = $field_default_value)? , $field_ty
                    )?
                    ;
                ])*
            };

            $crate::__impl_props_types_valid_trait! {
                $([
                    $field_name

                    $(
                        $(= $field_builder_default_output_value =>)?
                        ($($field_builder_inputs)*)
                            -> $field_builder_output
                            $field_builder_impl
                    )?

                    $([ $($field_modifiers_or_builder_generics)* ])?

                    $(
                        : $( = $field_default_value)? , $field_ty
                    )?
                    ;
                ])*
            }

            pub type DataInitial = Data<TypesInitial>;

            mod builder_impl_inherit {
                use super::super::*;

                $crate::__impl_props_field_declaration_normalize_iter! {
                    [$crate::__impl_props_inherit_take_data]
                    { $($field_name),* }
                    $([
                        // $(#[$($fn_attr)*])* // ignore attributes
                        $field_name

                        $([ $($field_modifiers_or_builder_generics)* ])?
                        $(
                            ($($field_builder_inputs)*)
                                -> $field_builder_output
                                $(= $field_builder_default_output_value =>)?
                                $field_builder_impl
                        )?

                        $(
                            : $field_ty $( = $field_default_value)?
                        )?
                    ])*
                }
            }


            mod inherited_prelude {
                use super::super::*;

                $crate::__impl_props_field_declaration_normalize_iter! {
                    [$crate::__impl_props_prelude]
                    {}
                    $([
                        // $(#[$($fn_attr)*])* // ignore attributes
                        $field_name

                        $([ $($field_modifiers_or_builder_generics)* ])?
                        $(
                            ($($field_builder_inputs)*)
                                -> $field_builder_output
                                $(= $field_builder_default_output_value =>)?
                                $field_builder_impl
                        )?

                        $(
                            : $field_ty $( = $field_default_value)?
                        )?
                    ])*
                }
            }

            pub mod prelude {
                pub use super::Builder as _;
                pub use super::inherited_prelude::*;
            }
        }

        $vis use $name::builder_impl_fn_uninitialized::$name;
    };
}

#[macro_export]
macro_rules! Build {
    (
        $($name:ident)? $(:: p:ident)*
        $(< $($ty_field:tt)*)?
    ) => {
        $($name)? $(:: p)* < impl ?::core::marker::Sized + $($name)? $(:: p)*::Types $(< $($ty_field)*)? >
    };
}

#[macro_export]
macro_rules! build {
    (
        $($name:ident)? $(:: $p:ident)* {
            .. $start:expr
            $( ,
                $field:ident
                $(: $field_value:expr)?
            )*
            $(,)?
        }
    ) => {{
        #[allow(unused_imports)]
        use $($name)? $(:: $p)* ::prelude::*;

        $start $(
            . $field (
                $crate::expand_a_or_b!([$($field_value)?][$field])
            )
        )*
    }};
    (
        $($name:ident)? $(:: $p:ident)* {
            $(
                $field:ident
                $(: $field_value:expr)?
            ),*
            $(,)?
        }
    ) => {
        $crate::valid! (
            $($name)? $(:: $p)* {
                .. $($name)? $(:: $p)* (),
                $(
                    $field
                    $(: $field_value)?
                ),*
            }
        )
    };
}

#[macro_export]
macro_rules! Valid {
    (
        $($name:ident)? $(:: $p:ident)*
        $(< $($ty_field:tt)*)?
    ) => {
        $($name)? $(:: $p)* ::Data:: < impl ?::core::marker::Sized + $($name)? $(:: $p)* ::ValidTypes $(< $($ty_field)*)? >
    };
}

#[macro_export]
macro_rules! valid {
    (
        // [$e:expr]
        $($name:ident)? $(:: $p:ident)* {
            $(
                $field:ident
                $(: $field_value:expr)?
                ,
            )*
            .. $base:expr
        }
    ) => {{
        #[allow(unused_imports)]
        use $($name)? $(:: $p)* ::prelude::*;

        #[inline]
        fn __assert_build_valid<TypeDefs: ?::core::marker::Sized + $($name)? $(:: $p)* ::ValidTypes>(
            v: $($name)? $(:: $p)* ::Data<TypeDefs>
        )   -> $($name)? $(:: $p)* ::Data<TypeDefs> {
            v
        }

        __assert_build_valid(
            $base $(
                . $field (
                    $crate::expand_a_or_b!([$($field_value)?][$field])
                )
            )*
        )
    }};
    (
        // [$e:expr]
        $($name:ident)? $(:: $p:ident)* {
            $(
                $field:path
                $(: $field_value:expr )?
            ),*
            $(,)?
        }
    ) => {
        $crate::valid! (
            $($name)? $(:: $p)* {
                $(
                    $field
                    $(: $field_value )?
                    ,
                )*
                .. $($name)? $(:: $p)* ()
            }
        )
    };
}
