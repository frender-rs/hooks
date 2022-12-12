#![deny(clippy::undocumented_unsafe_blocks)]

mod macros;
mod props;
mod render;

pub use macros::*;
pub use props::*;
pub use render::*;

pub mod builder;

mod utils;

use builder::{Unspecified, UnwrapData, WrapData};

pub type CounterWithInitialValuePropsTypesAllUnspecified = dyn CounterWithInitialValuePropsTypes<
    //
    initial_value = Unspecified,
>;

#[allow(non_camel_case_types)]
pub trait CounterWithInitialValuePropsTypes {
    type initial_value;

    fn initial_value<V>(self, new_value: V) -> Self::Wrapped
    where
        Self: Sized
            + UnwrapData<
                Data = CounterWithInitialValueProps<
                    dyn CounterWithInitialValuePropsTypes<
                        //
                        initial_value = Self::initial_value,
                    >,
                >,
            > + WrapData<
                CounterWithInitialValueProps<
                    dyn CounterWithInitialValuePropsTypes<
                        //
                        initial_value = V,
                    >,
                >,
            >,
    {
        #[allow(unused_variables)]
        let props = Self::unwrap_data(self);
        let props = CounterWithInitialValueProps {
            initial_value: new_value,
        };
        Self::wrap_data(props)
    }
}

impl<T, PropsTypesDef: ?Sized + CounterWithInitialValuePropsTypes> CounterWithInitialValuePropsTypes
    for T
where
    T: UnwrapData<Data = CounterWithInitialValueProps<PropsTypesDef>>,
{
    type initial_value = PropsTypesDef::initial_value;
}

pub struct CounterWithInitialValueProps<
    PropsTypesDef: ?Sized + CounterWithInitialValuePropsTypes = CounterWithInitialValuePropsTypesAllUnspecified
> {
    pub initial_value: PropsTypesDef::initial_value,
}

pub fn CounterWithInitialValueProps() -> CounterWithInitialValueProps {
    CounterWithInitialValueProps {
        initial_value: Unspecified,
    }
}

impl<PropsTypesDef: ?Sized + CounterWithInitialValuePropsTypes> UnwrapData
    for CounterWithInitialValueProps<PropsTypesDef>
{
    type Data = Self;

    #[inline]
    fn unwrap_data(self) -> Self::Data {
        self
    }

    fn unwrap_as_data(&self) -> &Self::Data {
        self
    }

    fn unwrap_as_mut_data(&mut self) -> &mut Self::Data {
        self
    }
}

impl<
        PropsTypesDef: ?Sized + CounterWithInitialValuePropsTypes,
        PropsTypesDefNew: ?Sized + CounterWithInitialValuePropsTypes,
    > WrapData<CounterWithInitialValueProps<PropsTypesDefNew>>
    for CounterWithInitialValueProps<PropsTypesDef>
{
    type Wrapped = CounterWithInitialValueProps<PropsTypesDefNew>;

    fn wrap_data(props: CounterWithInitialValueProps<PropsTypesDefNew>) -> Self::Wrapped {
        props
    }
}

pub trait CounterWithInitialValueProps__initial_value {
    fn specified_or(&self, default_value: i32) -> i32;
}

pub trait CounterWithInitialValuePropsTypesValid:
    CounterWithInitialValuePropsTypes<
    initial_value = <Self as CounterWithInitialValuePropsTypesValid>::initial_value,
>
{
    type initial_value: CounterWithInitialValueProps__initial_value;
}

impl<T, initial_value: CounterWithInitialValueProps__initial_value>
    CounterWithInitialValuePropsTypesValid for T
where
    T: CounterWithInitialValuePropsTypes<initial_value = initial_value>,
{
    type initial_value = initial_value;
}

#[repr(transparent)]
pub struct CounterWithInitialValue<Props> {
    pub props: Props,
}

impl<Props, NewProps> WrapData<NewProps> for CounterWithInitialValue<Props> {
    type Wrapped = CounterWithInitialValue<NewProps>;

    #[inline]
    fn wrap_data(props: NewProps) -> Self::Wrapped {
        CounterWithInitialValue { props }
    }
}

impl<Props> UnwrapData for CounterWithInitialValue<Props> {
    type Data = Props;

    #[inline]
    fn unwrap_data(self) -> Self::Data {
        self.props
    }

    fn unwrap_as_data(&self) -> &Self::Data {
        &self.props
    }

    fn unwrap_as_mut_data(&mut self) -> &mut Self::Data {
        &mut self.props
    }
}

impl<PropsTypesDef: ?Sized + CounterWithInitialValuePropsTypesValid>
    CounterWithInitialValue<CounterWithInitialValueProps<PropsTypesDef>>
{
    pub fn finish_builder(
        self,
    ) -> HookElementWithProps<
        impl FnOnceOutputElementHookWithProps<Dom, CounterWithInitialValueProps<PropsTypesDef>>,
        CounterWithInitialValueProps<PropsTypesDef>,
    > {
        #[hooks::hook(args_generics = "'render_ctx")]
        pub fn use_impl_render<PropsTypesDef: ?Sized + CounterWithInitialValuePropsTypesValid>(
            ctx: ContextAndState<'render_ctx, Dom, dyn std::any::Any>,
            props: &CounterWithInitialValueProps<PropsTypesDef>,
        ) -> ContextAndState<'render_ctx, Dom, impl render::RenderState + 'static> {
            let ctx = ctx.downcast_state().unwrap();

            let (state, updater) = hooks::use_state(props.initial_value.specified_or(4));

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
                // Box::new(ElementFnOnce(Counter)),
                // rsx!(<button
                //         on_click={move |_: &_| {
                //             web_sys::console::log_1(&"on_click".into());
                //             updater.replace_with_fn_pointer(|v| *v + 1);
                //         }}
                //         children={format!("state = {}", state)}
                // />
                // ),
            ))
        }

        HookElementWithProps(use_impl_render, self.props)
    }
}

pub fn CounterWithInitialValue() -> CounterWithInitialValue<CounterWithInitialValueProps> {
    CounterWithInitialValue {
        props: CounterWithInitialValueProps(),
    }
}

pub trait Counter {}

pub fn Counter() -> impl render::UpdateRenderState<Dom> + Copy {
    #[hooks::hook(args_generics = "'render_ctx")]
    pub fn use_impl_render(
        ctx: ContextAndState<'render_ctx, Dom, dyn std::any::Any>,
    ) -> ContextAndState<'render_ctx, Dom, impl render::RenderState + 'static> {
        let ctx = ctx.downcast_state().unwrap();

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
            // Box::new(ElementFnOnce(Counter)),
            // rsx!(<button
            //         on_click={move |_: &_| {
            //             web_sys::console::log_1(&"on_click".into());
            //             updater.replace_with_fn_pointer(|v| *v + 1);
            //         }}
            //         children={format!("state = {}", state)}
            // />
            // ),
        ))
    }

    render::HookElement(use_impl_render)
}
