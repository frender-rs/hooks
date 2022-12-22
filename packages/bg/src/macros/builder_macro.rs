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
                <Self as super::ReplaceInherited<
                    super::overwrite:: $field_name ::<<Self as super::Inherit>::InheritedTypeDefs, $field_builder_output>
                >>::Replaced
            where Self: super::ReplaceInherited<
                    super::overwrite:: $field_name ::<<Self as super::Inherit>::InheritedTypeDefs, $field_builder_output>
                >
            {
                let _builder_impl_field_new_value = $field_builder_impl;
                <Self as super::ReplaceInherited<
                    super::overwrite:: $field_name ::<<Self as super::Inherit>::InheritedTypeDefs, $field_builder_output>
                >>::replace_inherited(
                    self,
                    move |super::Data { __phantom_type_defs: _, $($all_fields,)* }| {
                        let _ = $field_name;
                        let $field_name = _builder_impl_field_new_value;
                        super::Data {
                            __phantom_type_defs: ::core::marker::PhantomData,
                            $($all_fields,)*
                        }
                    }
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
                <<Self as $crate::TakeData<Data<TypesNormalize<Self>>>>::Left as $crate::JoinData<Data<
                    self::overwrite:: $field_name ::<Self, $field_builder_output>
                >>>::Joined
            where <Self as $crate::TakeData<Data<TypesNormalize<Self>>>>::Left : $crate::JoinData<Data<
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
                ) = <Self as $crate::TakeData<Data<TypesNormalize<Self>>>>::take_data(self);

                let $field_name = _builder_impl_field_new_value;

                $crate::JoinData::<Data<
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
macro_rules! __impl_builder_associated_item {
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
        pub fn $field_name
            $(< $($builder_generics)* >)?
            (mut self, $($field_builder_inputs)*) -> Self {
                self.0.$field_name = $field_builder_impl;
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
        pub fn $field_name
            $(< $($builder_generics)* >)?
            (self, $($field_builder_inputs)*) ->
                super::Building<
                    super::overwrite:: $field_name ::<TypeDefs, $field_builder_output>
                >
            {
                let _builder_impl_field_new_value = $field_builder_impl;
                let super::Building(super::Data { __phantom_type_defs: _, $($all_fields,)* }) = self;
                let _ = $field_name;
                let $field_name = _builder_impl_field_new_value;
                super::Building(super::Data {
                    __phantom_type_defs: ::core::marker::PhantomData,
                    $($all_fields,)*
                })
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
        pub fn $field_name
            $(< $($builder_generics)* >)?
            (self, $($field_builder_inputs)*) ->
                super::Building<
                    super::overwrite:: $field_name ::<TypeDefs, $field_builder_output>
                >
            {
                let _builder_impl_field_new_value = $field_builder_impl;
                let (
                    _builder_impl_field_left,
                    Data {
                        __phantom_type_defs: _,
                    $(
                        $all_fields,
                    )*
                    }
                ) = <Self as $crate::TakeData<Data<TypesNormalize<Self>>>>::take_data(self);

                let $field_name = _builder_impl_field_new_value;

                $crate::JoinData::<Data<
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
                = type($crate::Unspecified::<$field_builder_output>)
                  value($crate::Unspecified)
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
                = type($crate::Unspecified<$field_ty>)
                  value($crate::Unspecified)
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
                 value($($inherit_path)*::build($($inherit_path)* ()))
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
            [impl $crate::MaybeBorrow<$for_ty>]
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
            [impl $crate::Maybe<$for_ty>]
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

                [ impl $($field_bound)* ]: $crate::UnspecifiedField<field_tag::$field_name> = $crate::UnspecifiedField
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
            $( $field_name = $crate::expand_a_or_b![ [$($initial_ty_maybe)?] [$crate::Unspecified<$($field_modifier_maybe)*>] ], )?
            $( $field_name = $crate::expand_a_or_b![ [$($initial_ty_impl)? ] [$crate::UnspecifiedField<super::field_tag::$field_name>]], )?
            $( $field_name = $crate::expand_a_or_b![ [$($initial_ty_bm  )? ] [$crate::Unspecified<$($field_modifier_bm   )*>] ], )?
            $( $field_name = $crate::expand_a_or_b![ [$($initial_ty_ih  )? ] [$($field_modifier_ih)* :: DataInitial] ], )?
            $( $field_name = $crate::Unspecified<$generic_field_builder_output>, )?
            $( $field_name = $crate::Unspecified<$generic_field_ty>, )?
        )*
        $($full_suffix)*
    };
}

#[macro_export]
macro_rules! __impl_props_field_macro {
    (
        {$(
            $field_name:ident $ignored:tt
        )*}
    ) => {
        $(
            #[allow(unused_macros)]
            macro_rules! $field_name {
                ($field_name) => { $field_name };
                ($f:ident) => {
                    <TypeDefs as super::Types>::$f
                };
            }
        )*
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
        {
            $field_name:ident $ignored:tt
        }
        $metadata:tt
        $type_and_field_name:ident
        $($other:tt)*
    ) => {
        #[allow(non_camel_case_types)]
        pub type $type_and_field_name <TypeDefs, $type_and_field_name> =
        <TypeDefs as $crate::__private::PhantomTypeParam<
        dyn super::Types<
            $field_name =
                $type_and_field_name ![$field_name],
        >
        >>::Out;
    };
    (
        {$(
            $field_name:ident $ignored:tt
        )*}
        $metadata:tt
        $type_and_field_name:ident
        $($other:tt)*
    ) => {
        #[allow(non_camel_case_types)]
        pub type $type_and_field_name <TypeDefs, $type_and_field_name> =
            dyn super::Types<$(
                $field_name =
                    $type_and_field_name ![$field_name],
            )*>
        ;
    };
}

#[macro_export]
macro_rules! __impl_props_overwrite_fields_impl {
    (
        $common_data:tt
        {$($field_declaration:tt)*}
    ) => {
        $crate::__impl_props_field_macro! {
            $common_data
        }

        $(
            $crate::__impl_props_field_declaration_normalize! {
                [$crate::__impl_props_overwrite_field]
                $common_data $field_declaration
            }
        )*
    };
}

#[macro_export]
macro_rules! __impl_props_overwrite_fields {
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

        $field_declarations:tt
    ) => {
        $crate::__impl_props_overwrite_fields_impl! {
            {
                $(
                    $($field_name ( $($($initial_v_mod)?)? ))?
                    $($field_name ( $generic_field_builder_output ))?
                    $($field_name ( $generic_field_ty ))?
                )*
            }
            $field_declarations
        }
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
        pub trait ValidTypes: super::Types<
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
                $( type $field_name : $crate::Maybe<$($field_modifier_maybe)*>; )?

                $( type $field_name : $($field_modifier_impl)*; )?

                $( type $field_name : $crate::MaybeBorrow<$($field_modifier_bm)*>; )?

                $( type $field_name : ?::core::marker::Sized + $($field_modifier_ih)* :: ValidTypes; )?
            )*
        }

        impl<
            T: ?::core::marker::Sized,
            $(
                $( #[allow(non_camel_case_types)] $field_name : ?::core::marker::Sized + $($field_modifier_ih)* :: ValidTypes ,)?
            )*
        > ValidTypes for T where T: super::Types<
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
                <T as super::Types>::$field_name : $crate::Maybe<$($field_modifier_maybe)*>,
            )?
            $( <T as super::Types>::$field_name: $($field_modifier_impl)*, )?
            $(
                <T as super::Types>::$field_name : $crate::MaybeBorrow<$($field_modifier_bm)*>,
            )?
        )*
    {
        $(
            $(
                #[doc = stringify!($($field_modifier_maybe)*)]
                type $field_name = <T as super::Types>::$field_name;
            )?

            $(
                #[doc = stringify!($($field_modifier_impl)*)]
                type $field_name = <T as super::Types>::$field_name;
            )?

            $(
                #[doc = stringify!($($field_modifier_bm)*)]
                type $field_name = <T as super::Types>::$field_name;
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
    ({ $name:ident { $($all_fields:ident),* } } { inherit { path($($inherit_path:tt)*) } }
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
            InheritedTypeDefs: ?Sized + $($inherit_path)*::Types,
            TypeDefs: ?Sized + super::Types< $field_name = $($inherit_path)*::Data<InheritedTypeDefs> >,
        > $($inherit_path)*::Inherit for super::Building<TypeDefs> {
            type InheritedTypeDefs = InheritedTypeDefs;

            #[inline]
            fn as_mut_inherited(this: &mut Self) -> &mut $($inherit_path)*::Data<InheritedTypeDefs> {
                &mut this.0. $field_name
            }
        }

        impl<
            InheritedTypeDefs: ?Sized + $($inherit_path)*::Types,
            NewTypeDefs: ?Sized + $($inherit_path)*::Types,
            TypeDefs: ?Sized + super::Types< $field_name = $($inherit_path)*::Data<InheritedTypeDefs> >,
        > $($inherit_path)*::ReplaceInherited<NewTypeDefs> for super::Building<TypeDefs> {
            type Replaced =
                super::Building<
                    super::overwrite:: $field_name ::<TypeDefs, $($inherit_path)*::Data<NewTypeDefs>>
                >
            ;

            fn replace_inherited<
                F: FnOnce($($inherit_path)* ::Data<Self::InheritedTypeDefs>) -> $($inherit_path)* ::Data<NewTypeDefs>,
            >(
                Self($name{
                    __phantom_type_defs,
                    $($all_fields),*
                }): Self,
                __bg_impl_v_replace: F,
            ) -> Self::Replaced {
                let $field_name = __bg_impl_v_replace($field_name);
                super::Building($name {
                    __phantom_type_defs: ::core::marker::PhantomData,
                    $($all_fields),*
                })
            }
        }
    };
    ($($other:tt)*) => {};
}

#[macro_export]
macro_rules! __impl_builder_fns {
    (
        [inheritable $($inheritable_nothing:tt)?]
        [$name:ident]
        $($data:tt)*
    ) => {
        pub trait $name: ::core::marker::Sized + super::Inherit {
            $crate::__impl_props_field_declaration_normalize_iter! {
                [$crate::__impl_props_types_builder_trait_item]
                $($data)*
            }
        }

        impl<B> $name for B
            where B: ::core::marker::Sized + super::Inherit {}
    };
    (
        []
        [$name:ident]
        $($data:tt)*
    ) => {
        pub type $name = ();

        impl<TypeDefs: ?::core::marker::Sized + super::Types> super::Building<TypeDefs> {
            $crate::__impl_props_field_declaration_normalize_iter! {
                [$crate::__impl_builder_associated_item]
                $($data)*
            }
        }
    };
}

#[macro_export]
macro_rules! __impl_trait_inherit {
    (
        [inheritable $($inheritable_nothing:tt)?]
    ) => {
        pub trait Inherit {
            type InheritedTypeDefs: ?::core::marker::Sized + Types;

            fn as_mut_inherited(this: &mut Self) -> &mut Data<Self::InheritedTypeDefs>;
        }

        pub trait ReplaceInherited<NewTypeDefs: ?::core::marker::Sized + Types>: Inherit {
            type Replaced: Inherit<InheritedTypeDefs = NewTypeDefs>;

            fn replace_inherited<F: FnOnce(Data<Self::InheritedTypeDefs>) -> Data<NewTypeDefs>>(
                this: Self,
                replace: F,
            ) -> Self::Replaced;
        }

        impl<TypeDefs: ?::core::marker::Sized + Types> Inherit for Building<TypeDefs> {
            type InheritedTypeDefs = TypeDefs;

            #[inline]
            fn as_mut_inherited(this: &mut Self) -> &mut Data<Self::InheritedTypeDefs> {
                &mut this.0
            }
        }

        impl<
                TypeDefs: ?::core::marker::Sized + Types,
                NewTypeDefs: ?::core::marker::Sized + Types,
            > ReplaceInherited<NewTypeDefs> for Building<TypeDefs>
        {
            type Replaced = Building<NewTypeDefs>;

            #[inline]
            fn replace_inherited<F: FnOnce(Data<Self::InheritedTypeDefs>) -> Data<NewTypeDefs>>(
                this: Self,
                replace: F,
            ) -> Self::Replaced {
                Building(replace(this.0))
            }
        }
    };
    ([]) => {};
}

#[macro_export]
macro_rules! builder {
    (
        $(#[$($mod_and_fn_attr:tt)*])*
        $vis:vis struct $name:ident {}
        $($other_items:tt)*
    ) => {
        $(#[$($mod_and_fn_attr)*])*
        #[allow(non_snake_case)]
        pub mod $name {
            pub trait Types {}
            // pub type Builder = ();

            pub struct $name<TypeDefs: ?::core::marker::Sized + Types> {
                __phantom_type_defs: ::core::marker::PhantomData<*const TypeDefs>,
            }

            pub(super) fn data_initial() -> Data<TypesInitial> {
                Data {
                    __phantom_type_defs: ::core::marker::PhantomData,
                }
            }

            pub use $name as Data;

            pub type TypesInitial = dyn Types;

            pub use Types as ValidTypes;

            pub type DataInitial = Data<TypesInitial>;
            pub struct Building<TypeDefs: ?Sized + Types>(pub Data<TypeDefs>);

            #[inline]
            pub fn build<TypeDefs: ?Sized + Types>(building: Building<TypeDefs>) -> Data<TypeDefs> {
                building.0
            }

            #[inline]
            pub fn valid<TypeDefs: ?Sized + ValidTypes>(building: Building<TypeDefs>) -> Data<TypeDefs> {
                building.0
            }

            pub mod prelude {}

            $($other_items)*
        }

        $(#[$($mod_and_fn_attr)*])*
        #[inline]
        #[allow(non_snake_case)]
        pub fn $name() -> $name::Building<$name::TypesInitial> {
            $name::Building($name::data_initial())
        }
    };
    (
        $(#![inheritable $($inheritable_nothing:tt)?])?
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
            $crate::__report_wrong_tt! { $($($inheritable_nothing)?)? }

            pub mod overwrite {
                $crate::__impl_props_overwrite_fields! {
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
                    {$([
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
                    ])*}
                }
            }

            pub mod field_tag {
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

            mod trait_types {
                use super::super::*;
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
            }

            pub use trait_types::Types;

            mod builder {
                use super::super::*;

                $crate::__impl_builder_fns! {
                    [$(inheritable $($inheritable_nothing)?)?]
                    [$name]
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

            pub use builder::$name as Builder;

            pub mod struct_data {
                use super::super::*;

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

                $crate::__impl_props_field_declaration_normalize_iter! {
                    [$crate::__impl_props_inherit_take_data]
                    { $name { $($field_name),* } }
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

            pub(super) fn data_initial() -> Data<TypesInitial> {
                use super::*;
                Data {
                    __phantom_type_defs: ::core::marker::PhantomData,
                    $(
                    $field_name : $crate::__impl_props_field_declaration_normalize! {
                        [$crate::__impl_props_types_field_initial_value] {} [
                            // $(#[$($fn_attr)*])* // ignore attrs
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

            pub use struct_data::$name as Data;

            $crate::__impl_trait_inherit! {
                [$(inheritable $($inheritable_nothing)?)?]
            }

            mod types_initial {
                use super::super::*;
                pub type TypesInitial = $crate::__impl_props_types_field_initial_ty_iter! {
                    [dyn super::Types<][>]
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
            }

            pub use types_initial::TypesInitial;

            mod trait_valid_types {
                use super::super::*;
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
            }

            pub use trait_valid_types::ValidTypes;

            pub type DataInitial = Data<TypesInitial>;

            pub struct Building<TypeDefs: ?Sized + Types>(pub Data<TypeDefs>);

            #[inline]
            pub fn build<TypeDefs: ?Sized + Types>(building: Building<TypeDefs>) -> Data<TypeDefs> {
                building.0
            }

            #[inline]
            pub fn valid<TypeDefs: ?Sized + ValidTypes>(building: Building<TypeDefs>) -> Data<TypeDefs> {
                building.0
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

        $(#[$($mod_and_fn_attr)*])*
        #[inline]
        #[allow(non_snake_case)]
        $vis fn $name() -> $name::Building<$name::TypesInitial> {
            $name::Building($name::data_initial())
        }
    };
    (
        $(#[$($mod_and_fn_attr:tt)*])*
        $vis:vis struct $name:ident
        ($($base_builder:tt)+);

        $($other_items:tt)*
    ) => {
        $(#[$($mod_and_fn_attr)*])*
        #[allow(non_snake_case)]
        $vis mod $name {
            mod reuse {
                use super::super::*;
                pub use $($base_builder)+ ::{
                    prelude, Building, Types, TypesInitial, ValidTypes,
                };
            }

            pub use reuse::{prelude, Building, Types, TypesInitial, ValidTypes};

            mod struct_data {
                use super::super::*;
                pub struct $name <
                    TypeDefs: ?::core::marker::Sized + $($base_builder)+ ::Types,
                >(pub $($base_builder)+ ::Data<TypeDefs>);
            }

            pub use struct_data::$name  as Data;

            pub type DataInitial = Data<TypesInitial>;

            #[inline]
            pub fn build<TypeDefs: ?::core::marker::Sized + Types>(
                building: Building<TypeDefs>,
            ) -> Data<TypeDefs> {
                use super::*;
                self::Data($($base_builder)+ ::build(building))
            }

            #[inline]
            pub fn valid<TypeDefs: ?::core::marker::Sized + ValidTypes>(
                building: Building<TypeDefs>,
            ) -> Data<TypeDefs> {
                build(building)
            }

            $($other_items)*
        }

        $(#[$($mod_and_fn_attr)*])*
        #[inline]
        #[allow(non_snake_case)]
        $vis fn $name (
        ) -> $name ::Building<$name ::TypesInitial> {
            $($base_builder)+ ()
        }
    };
}
