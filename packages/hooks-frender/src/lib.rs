#![deny(clippy::undocumented_unsafe_blocks)]

mod macros;
mod props;
mod render;

pub use macros::*;
pub use props::*;
pub use render::*;

mod utils;

pub use bg;
pub use hooks;
pub use hooks::component;
pub use hooks::hook;

use bg::{builder, Maybe};

builder! {
    struct CounterWithInitialValueProps {
        initial_value[? i32],
    }
}

#[component]
pub fn CounterWithInitialValue(ctx: _, props: &CounterWithInitialValueProps) {
    let (state, updater) =
        hooks::use_state_with(|| props.initial_value.as_some().copied().unwrap_or(4));

    let updater = updater.clone();

    web_sys::console::log_1(&"render".into());

    ctx.render((
        if *state % 3 == 0 { None } else { Some("help ") },
        // "please",
        render::button()
            .on_click(move |_: &_| {
                web_sys::console::log_1(&"on_click".into());
                updater.replace_with_fn_pointer(|v| *v + 1);
            })
            .children(format!("state = {}", state))
            .end_builder(),
        if *state % 2 == 0 {
            Some("state is even")
        } else {
            None
        },
        if *state % 2 == 1 {
            Some(format!("{state} is odd"))
        } else {
            None
        },
        " Last",
    ))
}

#[component]
pub fn Counter(ctx: _) {
    let (state, updater) = hooks::use_state(0);

    let updater = updater.clone();

    web_sys::console::log_1(&"render".into());

    ctx.render((
        if *state % 3 == 0 { None } else { Some("help ") },
        // "please",
        button()
            .on_click(move |_: &_| {
                web_sys::console::log_1(&"on_click".into());
                updater.replace_with_fn_pointer(|v| *v + 1);
            })
            .children(format!("state = {}", state))
            .end_builder(),
        if *state % 2 == 0 {
            Some("state is even")
        } else {
            None
        },
        if *state % 2 == 1 {
            Some(format!("{state} is odd"))
        } else {
            None
        },
        " Last",
    ))
}
