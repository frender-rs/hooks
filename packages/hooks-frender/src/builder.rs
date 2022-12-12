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

impl<T, Tag: FieldTag> MaybeSpecifiedFor<Tag> for Option<T>
where
    T: MaybeSpecifiedFor<Tag>,
{
    #[inline]
    fn specified(self) -> Option<<Tag as FieldTag>::Field> {
        self.and_then(T::specified)
    }

    #[inline]
    fn as_specified(&self) -> Option<&<Tag as FieldTag>::Field> {
        self.as_ref().and_then(T::as_specified)
    }

    #[inline]
    fn as_mut_specified(&mut self) -> Option<&mut <Tag as FieldTag>::Field> {
        self.as_mut().and_then(T::as_mut_specified)
    }
}

impl<Tag: FieldTag> MaybeSpecifiedFor<Tag> for Unspecified {
    #[inline]
    fn specified(self) -> Option<<Tag as FieldTag>::Field> {
        None
    }

    #[inline]
    fn as_specified(&self) -> Option<&<Tag as FieldTag>::Field> {
        None
    }

    #[inline]
    fn as_mut_specified(&mut self) -> Option<&mut <Tag as FieldTag>::Field> {
        None
    }
}
