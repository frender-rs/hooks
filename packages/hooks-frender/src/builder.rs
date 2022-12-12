pub struct Unspecified;

pub trait WrapData<Data> {
    type Wrapped;
    fn wrap_data(props: Data) -> Self::Wrapped;
}

pub trait UnwrapData {
    type Data;
    fn unwrap_data(self) -> Self::Data;
    fn unwrap_as_data(&self) -> &Self::Data;
    fn unwrap_as_mut_data(&mut self) -> &mut Self::Data;
}

pub trait FieldTag {
    type Field;
}

pub trait MaybeSpecifiedFor<Tag: FieldTag> {
    fn specified(self) -> Option<Tag::Field>;
    fn as_specified(&self) -> Option<&Tag::Field>;
    fn as_mut_specified(&mut self) -> Option<&mut Tag::Field>;
}
