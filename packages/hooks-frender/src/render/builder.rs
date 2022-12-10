pub trait EndBuilder {
    type Output;

    fn end_builder(self) -> Self::Output;
}

pub trait EndBuilderAsSelf: EndBuilder<Output = Self> {}

impl<T> EndBuilderAsSelf for T where T: EndBuilder<Output = Self> {}
