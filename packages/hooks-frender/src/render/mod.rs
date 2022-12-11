mod builder;
mod core_elements;
mod ctx_and_state;
mod dom;
mod hook;
mod hook_with_props;
mod render_state;
mod ssr;
mod tuple;

pub use builder::*;
pub use core_elements::*;
pub use ctx_and_state::*;
pub use dom::*;
pub use hook::*;
pub use hook_with_props::*;
pub use render_state::*;
pub use ssr::*;
pub use tuple::*;

mod element;
pub use element::button::button;

#[derive(Debug, Default)]
pub struct Unset;

#[macro_export]
macro_rules! rsx {
    ( < $start_builder:ident
        $(
            $field:ident
            $(::<
                $(
                    $($field_generics_lt:lifetime)?
                    $($field_generics_ty:ty)?
                ),* $(,)?
            >)?
            =$value:tt
        )*
        />
    ) => {
        $start_builder()
            $(
                . $field
                $(::<
                    $(
                        $($field_generics_lt)?
                        $($field_generics_ty)?
                    ),*
                >)?
                ($value)
            )*
            .finish_builder()
    };
    ( < $start_builder:ident
        $(
            $field:ident
            $(::<
                $(
                    $($field_generics_lt:lifetime)?
                    $($field_generics_ty:ty)?
                ),* $(,)?
            >)?
            =$value:tt
        )*
        >
        $($rest:tt)*
    ) => {
        $crate::rsx! (
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
