use bg::{builder, valid, Maybe, Valid};

mod html {
    use bg::builder;

    builder! {
        #![inheritable]
        pub struct ElementProps {
            id[borrow? str],
        }
    }
}

builder! {
    #![inheritable]
    /// The outer docs go to `mod` and `fn` [`MyCompProps`].
    struct MyCompProps {
        /// This field is required because there is not a default value.
        /// All required fields are generic,
        /// because they need to store different types.
        /// For this field, it initially stores zero-sized `Unspecified`,
        /// if the builder method is called, then *it* stores `Vec<u8>`.
        ///
        /// Thus, the field name is also an associated type in trait [`Types`].
        ///
        /// `TypeDefsInitial::required_field` is [`Unspecified`].
        /// `TypeDefsValid::required_field`   is `Vec<u8>`.
        ///
        /// The builder method just changes the type and value.
        required_field: Vec<u8>,

        /// This field is optional because there is a default value `"".into()`.
        ///
        /// This field is not generic because it only stores `std::borrow::Cow<'static,str>`.
        ///
        /// Thus, this field will NOT be in `Types`.
        ///
        /// ```
        /// type ShouldFail = TypeDefsInitial::field_with_default;
        /// ```
        field_with_default: std::borrow::Cow<'static,str> = "".into(),

        /// This field is required because there is not a default value.
        ///
        /// `TypeDefsInitial::required_field_with_generic_input` is [`Unspecified`].
        /// `TypeDefsValid::required_field_with_generic_input`   is `Option<i32>`.
        ///
        /// The builder method changes the type to `Option<i32>`
        /// and run `value.into()` to get the value.
        /// The builder method takes generic input.
        required_field_with_generic_input[V: Into<Option<i32>>](input: V) -> Option<i32> {
            input.into()
        },

        /// This field is optional because there is a default value `Some(1)`.
        ///
        /// This field is not generic because it only stores `Option<i32>`.
        ///
        /// ```
        /// type ShouldFail = TypeDefsInitial::optional_field_with_generic_input;
        /// ```
        ///
        /// However, the builder method takes generic inputs.
        optional_field_with_generic_input[V: Into<Option<i32>>](value: V) -> Option<i32> = Some(1) => {
            value.into()
        },

        /// This field is required because there is not a default value.
        /// This field is generic because it stores `Unspecified` initially,
        /// then stores any type that `impl Into<i32>` after calling the builder method.
        ///
        /// `TypeDefsInitial::required_field_with_generic_input` is [`Unspecified`].
        /// `TypeDefsValid::required_field_with_generic_input`   is still generic.
        ///
        /// The builder is like:
        ///
        /// ```
        /// # struct Builder<TypeDefs: ?Sized + >(Data<TypeDefs>);
        /// # impl<TypeDefs: ?Sized + Types> Builder<TypeDefs> {
        /// fn generic_field<V: Into<i32>>(self, new_value: V) -> MyCompProps::Data<Types![..TypeDefs, generic_field = V]> {
        ///     todo!()
        /// }
        /// # }
        /// ```
        generic_field[impl Into<i32>],

        /// The type will be [`String`] at start,
        /// and the value will be `"".into()` at start.
        /// The valid type is `TypeDefs::GenericField2 where TypeDefs::GenericField2: MaybeSpecified<i32>`.
        /// The builder method just *changes* this type and value.
        generic_field_2[impl Into<String>]: &'static str = "hello",

        /// The type and value will be [`Unspecified`] at start,
        /// The valid type is `TypeDefs::generic_field_maybe_specified where TypeDefs::generic_field_maybe_specified: MaybeSpecifiedFor<generic_field_maybe_specified>`.
        /// The builder method just *changes* this type and value.
        generic_field_maybe_specified[? String],

        /// The initial type is [`Option<String>`].
        /// The initial value is `None`.
        /// The valid type is `TypeDefs::generic_field_maybe_specified_2 where TypeDefs::generic_field_maybe_specified_2: MaybeSpecifiedFor<generic_field_maybe_specified_2>`.
        /// The builder method just *changes* this type and value.
        generic_field_maybe_specified_2[? String]: Option<String> = None,
        element_props[inherit html::ElementProps],
    }
}

#[test]
fn builder() {
    let b = {
        use MyCompProps::prelude::*;
        MyCompProps()
            .required_field(vec![])
            .required_field_with_generic_input(None)
            .id("")
    }
    .0;

    let b = valid!(MyCompProps {
        generic_field: 2,
        generic_field_maybe_specified: String::from("world"),
        id: "my-id",
        ..b
    });

    fn test_valid_props(props: Valid![MyCompProps]) {
        let field_with_default: std::borrow::Cow<'static, str> = props.field_with_default;
        assert!(matches!(field_with_default, std::borrow::Cow::Borrowed("")));

        let required_field: Vec<u8> = props.required_field;
        assert!(required_field.is_empty());

        let required_field_with_generic_input: Option<i32> =
            props.required_field_with_generic_input;
        assert_eq!(required_field_with_generic_input, None);

        let optional_field_with_generic_input: Option<i32> =
            props.optional_field_with_generic_input;
        assert_eq!(optional_field_with_generic_input, Some(1));

        assert_eq!(props.generic_field.into(), 2);

        assert_eq!(props.generic_field_2.into(), "hello");

        assert_eq!(
            props.generic_field_maybe_specified.some(),
            Some("world".to_string())
        );

        assert_eq!(props.generic_field_maybe_specified_2.as_some(), None);
    }

    test_valid_props(b);
}
