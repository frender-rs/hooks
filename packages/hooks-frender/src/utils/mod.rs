mod option;
mod pin;
mod reentrant;

pub use option::*;
pub use pin::*;
pub use reentrant::*;

macro_rules! def_dom_element {
    (
        $vis:vis $name:ident (
            props: $name_props:ident,
            builder: $name_builder:ident,
            state: $name_state:ident,
            element: $ty_element:ty,
            props_types: $props_types:ident,
        )
        $([ $($impl_generics:tt)* ])?
        {
            $(
                $field:ident
                $(
                    $({ $($field_impl_generics_lt:tt)* })?
                    $([$field_tg:ident $($field_impl_generics:tt)* ])?
                    ($field_pat:ident : $field_prop_ty:tt) -> $field_state_ty:ty
                    { $($field_impl:tt)* }
                )?
                $(: $field_ty:ty)?
            )*
        }
    ) => {
        $vis trait $props_types $(< $($impl_generics)* >)? {
            $(
                $(type $field_tg;)?
            )*
        }

        #[non_exhaustive]
        $vis struct $name_state <$( $($impl_generics)* ,)? PropsTypes: ?Sized + $props_types> {
            pub node: ::core::option::Option<$ty_element>,
            $(
                pub $field: $($field_state_ty)? $($field_ty)? ,
            )*
        }

        $vis struct $name_props $(< $($impl_generics)* >)? {
            $(
                pub $field: $($field_prop_ty)? $($field_ty)? ,
            )*
        }

        #[non_exhaustive]
        $vis struct $name_builder $(< $($field)* >)? {
            $(
                pub $field: $($field)* ,
            )*
        }

        $vis struct $name
        $(< $($impl_generics)* >)?
        (pub $name_props < $($($field_prop_ty)? $($field_ty)?)* >);


    };
}

#[cfg(aaa)]
def_dom_element! {
    Button(
        props: ButtonProps,
        builder: ButtonPropsBuilder,
        state: ButtonState,
        element: web_sys::HtmlButtonElement,
        props_types: ButtonPropsTypes,
    ){
        // children
    }
}
