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
                <Self as super::Inherit>::as_mut_inherited(&mut self).$field_name = $field_builder_impl;
                self
            }
    };
    ({$($all_fields:ident),*} $metadata:tt
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
                $crate::builder::Joined<
                    <Self as super::Inherit>::Left,
                    super::Data<
                        super::overwrite:: $field_name ::<<Self as super::Inherit>::InheritedTypeDefs, $field_builder_output>
                    >
                >
            where <Self as super::Inherit>::Left: $crate::builder::JoinData<
                super::Data<
                    super::overwrite:: $field_name ::<<Self as super::Inherit>::InheritedTypeDefs, $field_builder_output>
                >
            > {
                let _builder_impl_field_new_value = $field_builder_impl;
                let (
                    _builder_impl_field_left,
                    super::Data {
                        __phantom_type_defs: _,
                    $(
                        $all_fields,
                    )*
                    }
                ) = <Self as super::Inherit>::take_inherited(self);

                let $field_name = _builder_impl_field_new_value;

                $crate::builder::JoinData::<super::Data<
                    super::overwrite:: $field_name ::<<Self as super::Inherit>::InheritedTypeDefs, $field_builder_output>
                >>::join_data(
                    _builder_impl_field_left,
                    super::Data {
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
                <<Self as $crate::builder::TakeData<Data<TypesNormalize<Self>>>>::Left as $crate::builder::JoinData<Data<
                    self::overwrite:: $field_name ::<Self, $field_builder_output>
                >>>::Joined
            where <Self as $crate::builder::TakeData<Data<TypesNormalize<Self>>>>::Left : $crate::builder::JoinData<Data<
                self::overwrite:: $field_name ::<Self, $field_builder_output>
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
                ) = <Self as $crate::builder::TakeData<Data<TypesNormalize<Self>>>>::take_data(self);

                let $field_name = _builder_impl_field_new_value;

                $crate::builder::JoinData::<Data<
                    self::overwrite:: $field_name ::<Self, $field_builder_output>
                >>::join_data(
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
macro_rules! __impl_props_overwrite_field {
    (
        $common_data:tt
        { non_generic }
        $type_and_field_name:ident
        $($other:tt)*
    ) => {};
    (
        {[
            $field_name:ident

            $([ $($field_modifier_mod:tt)* ] $(: $(= $initial_v_mod:expr   )? , $initial_ty_mod:ty    )? ;)?
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
        ]}
        $metadata:tt
        $type_and_field_name:ident
        $($other:tt)*
    ) => {
        #[allow(non_camel_case_types)]
        pub type $type_and_field_name <TypeDefs, $type_and_field_name> = dyn super::Types<
            $field_name = <TypeDefs as $crate::builder::PhantomTypeParam<$type_and_field_name>>::Out
        >;
    };
    (
        {$([
            $field_name:ident

            $([ $($field_modifier_mod:tt)* ] $(: $(= $initial_v_mod:expr   )? , $initial_ty_mod:ty    )? ;)?
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
        ])*}
        $metadata:tt
        $type_and_field_name:ident
        $($other:tt)*
    ) => {
        $(
            $(
                #[doc = ::core::stringify!($($initial_ty_mod)?)]
                #[allow(unused_macros)]
                macro_rules! $field_name {
                    ($type_and_field_name) => { $type_and_field_name };
                    ($f:ident) => {
                        <TypeDefs as super::Types>::$f
                    };
                }
            )?
            $(
                #[doc = ::core::stringify!($generic_field_builder_output)]
                #[allow(unused_macros)]
                macro_rules! $field_name {
                    ($type_and_field_name) => { $type_and_field_name };
                    ($f:ident) => {
                        <TypeDefs as super::Types>::$f
                    };
                }
            )?
            $(
                #[doc = ::core::stringify!($generic_field_ty)]
                #[allow(unused_macros)]
                macro_rules! $field_name {
                    ($type_and_field_name) => { $type_and_field_name };
                    ($f:ident) => {
                        <TypeDefs as super::Types>::$f
                    };
                }
            )?
        )*

        #[allow(non_camel_case_types)]
        pub type $type_and_field_name <TypeDefs, $type_and_field_name> = dyn super::Types<$(
            $(
                $field_name = $crate::ignore_first_tt![
                    { $($initial_ty_mod)? }
                    $type_and_field_name ![$field_name]
                ],
            )?
            $(
                $field_name = $crate::ignore_first_tt![
                    { $generic_field_builder_output }
                    $type_and_field_name ![$field_name]
                ],
            )?
            $(
                $field_name = $crate::ignore_first_tt![
                    { $generic_field_ty }
                    $type_and_field_name ![$field_name]
                ],
            )?
        )*>;
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
                $( #[allow(non_camel_case_types)] $field_name : ?::core::marker::Sized + $($field_modifier_ih)* :: ValidTypes ,)?
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
        // pub use $($inherit_path)* ::Builder as _;
        // pub use $($inherit_path)* ::Inherit as _;
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
            TypeDefs: ?Sized + super::Types<$field_name = $crate::builder::UnspecifiedField<super::builder_impl_tag::$field_name>>,
        > $crate::builder::JoinData<$($inherit_path)*::Data<NewInheritedTypeDefs>> for super::Data<TypeDefs> {
            type Joined = super::Data<
                super::overwrite:: $field_name ::<TypeDefs, $($inherit_path)*::Data<NewInheritedTypeDefs>>
            >;

            #[inline]
            fn join_data(
                Self {
                    __phantom_type_defs,
                    $($all_fields),*
                }: Self,
                __builder_impl_v_data: $($inherit_path)*::Data<NewInheritedTypeDefs>
            ) -> Self::Joined {
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
            type Left = super::Data<
                super::overwrite:: $field_name ::<
                    TypeDefs,
                    $crate::builder::UnspecifiedField<super::builder_impl_tag:: $field_name>,
                >
            >;

            #[inline]
            fn take_inherited(
                Self {
                    __phantom_type_defs,
                    $($all_fields),*
                }: Self
            ) -> (Self::Left, $($inherit_path)*::Data<InheritedTypeDefs>) {
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
            fn as_mut_inherited(this: &mut Self) -> &mut $($inherit_path)*::Data<InheritedTypeDefs> {
                &mut this. $field_name
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

            pub mod overwrite {
                $crate::__impl_props_field_declaration_normalize_iter! {
                    [$crate::__impl_props_overwrite_field]
                    {$([
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
                    ])*}
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

            mod builder_impl_builder_trait {
                use super::super::*;

                pub trait $name: ::core::marker::Sized + super::Inherit {
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

                impl<B> $name for B
                    where B: ::core::marker::Sized + super::Inherit {}
            }

            pub use builder_impl_builder_trait::$name as Builder;

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
                type InheritedTypeDefs: ?::core::marker::Sized + Types;

                type Left;
                fn take_inherited(this: Self) -> (Self::Left, Data<Self::InheritedTypeDefs>);
                fn as_mut_inherited(this: &mut Self) -> &mut Data<Self::InheritedTypeDefs>;
            }

            impl<
                TypeDefs: ?::core::marker::Sized + Types,
            > Inherit for Data<TypeDefs> {
                type InheritedTypeDefs = TypeDefs;

                type Left = $crate::builder::NothingLeft;
                #[inline]
                fn take_inherited(this: Self) -> (Self::Left, Data<Self::InheritedTypeDefs>) {
                    ($crate::builder::NothingLeft, this)
                }
                #[inline]
                fn as_mut_inherited(this: &mut Self) -> &mut Data<Self::InheritedTypeDefs> {
                    this
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

            pub mod prelude {
                pub use super::Builder as _;

                #[allow(unused_imports)]
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

        $base $(
            . $field (
                $crate::expand_a_or_b!([$($field_value)?][$field])
            )
        )*
    }};
    (
        $($name:ident)? $(:: $p:ident)* {
            $(
                $field:ident
                $(: $field_value:expr )?
            ),*
            $(,)?
        }
    ) => {
        $crate::build! (
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
        $($name:ident)? $(:: $p:ident)* {
            $($field:tt)*
        }
    ) => {{
        #[inline]
        fn __assert_build_valid<TypeDefs: ?::core::marker::Sized + $($name)? $(:: $p)* ::ValidTypes>(
            v: $($name)? $(:: $p)* ::Data<TypeDefs>
        )   -> $($name)? $(:: $p)* ::Data<TypeDefs> {
            v
        }

        __assert_build_valid($crate::build!(
            $($name)? $(:: $p)* { $($field)* }
        ))
    }};
}
