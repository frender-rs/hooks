pub mod arguments_with_lifetimes {
    pub mod elided_and_not_used {
        use hooks_core::hook_fn;

        hook_fn!(
            fn not_specify_bounds(_: &str) {}
        );

        hook_fn!(
            type Bounds = impl '_;
            fn specify_elided_bounds(_: &str) {}
        );

        hook_fn!(
            type Bounds = impl 'static;
            fn specify_any_bounds(_: &str) {}
        );

        #[test]
        fn assert_static() {
            fn assert_static(v: impl Sized + 'static) {}

            let s = String::new();
            assert_static(specify_any_bounds(&s));
        }
    }

    /// See also:
    /// - [`hooks_core::doc_tests::ArgumentsWithElidedLifetimeAndUsedMustSpecifyBounds`]
    pub mod elided_and_used {
        use hooks_core::hook_fn;

        hook_fn!(
            type Bounds = impl '_;
            fn elided_and_used_specify_bounds(v: &str) {
                println!("{}", v)
            }
        );

        #[derive(Debug)]
        struct WithReceiverArgument;
        impl WithReceiverArgument {
            hook_fn!(
                type Bounds = impl '_;
                fn specify_bounds(&self) {
                    println!("{:?}", self)
                }
            );
        }
    }

    pub mod explicit_and_not_used {
        use hooks_core::hook_fn;

        hook_fn!(
            fn not_specify_bounds<'a>(_: &'a str) {}
        );

        hook_fn!(
            type Bounds = impl '_;
            fn specify_elided_bounds<'a>(_: &'a str) {}
        );

        hook_fn!(
            type Bounds = impl 'a;
            fn specify_explicit_bounds<'a>(_: &'a str) {}
        );

        hook_fn!(
            type Bounds = impl 'static;
            fn specify_any_bounds<'a>(_: &'a str) {}
        );
    }

    /// See also:
    /// - [`hooks_core::doc_tests::ArgumentsWithExplicitLifetimeAndUsedMustSpecifyBounds`]
    pub mod explicit_and_used {
        use hooks_core::hook_fn;

        hook_fn!(
            /// `hook_fn!` output type auto captures lifetimes declared in generics.
            fn explicit_and_used_auto_bounds<'a>(v: &'a str) {
                println!("{}", v)
            }
        );

        hook_fn!(
            type Bounds = impl '_;
            fn explicit_and_used_specify_elided_bounds<'a>(v: &'a str) {
                println!("{}", v)
            }
        );

        hook_fn!(
            type Bounds = impl 'a;
            fn explicit_and_used_specify_explicit_bounds<'a>(v: &'a str) {
                println!("{}", v)
            }
        );
    }

    pub mod explicit_and_returned {
        use hooks_core::hook_fn;

        hook_fn!(
            type Bounds = impl '_;
            fn specify_elided_bounds<'a>(v: &'a str) -> &'a str {
                v
            }
        );

        hook_fn!(
            type Bounds = impl 'a;
            fn specify_explicit_bounds<'a>(v: &'a str) -> &'a str {
                v
            }
        );
    }
}

pub mod generic_arguments_with_lifetimes {
    pub mod elided_and_not_used {
        use hooks_core::hook_fn;

        hook_fn!(
            fn not_specify_bounds<T>(_: &T) {}
        );

        hook_fn!(
            type Bounds = impl '_;
            fn specify_elided_bounds<T>(_: &T) {}
        );

        hook_fn!(
            type Bounds = impl 'static;
            fn specify_any_bounds<T>(_: &T) {}
        );
    }

    /// See also:
    /// - [`hooks_core::doc_tests::GenericArgumentsWithElidedLifetimeAndUsedMustSpecifyBounds`]
    pub mod elided_and_used {
        use hooks_core::hook_fn;

        hook_fn!(
            type Bounds = impl '_;
            fn elided_and_used_specify_bounds<T: std::fmt::Display>(v: &T) {
                println!("{}", v)
            }
        );
    }

    pub mod explicit_and_not_used {
        use hooks_core::hook_fn;

        hook_fn!(
            fn not_specify_bounds<'a, T>(_: &'a T) {}
        );

        hook_fn!(
            type Bounds = impl '_;
            fn specify_elided_bounds<'a, T>(_: &'a T) {}
        );

        hook_fn!(
            type Bounds = impl 'a;
            fn specify_explicit_bounds<'a, T>(_: &'a T) {}
        );

        hook_fn!(
            type Bounds = impl 'static;
            fn specify_any_bounds<'a, T>(_: &'a T) {}
        );
    }

    /// See also:
    /// - [`hooks_core::doc_tests::GenericArgumentsWithExplicitLifetimeAndUsedMustSpecifyBounds`]
    pub mod explicit_and_used {
        use hooks_core::hook_fn;

        hook_fn!(
            fn explicit_and_used_auto_bounds<'a, T: std::fmt::Display>(v: &'a T) {
                println!("{}", v);
            }
        );

        hook_fn!(
            type Bounds = impl '_;
            fn explicit_and_used_specify_elided_bounds<'a, T: std::fmt::Display>(v: &'a T) {
                println!("{}", v)
            }
        );

        hook_fn!(
            type Bounds = impl 'a;
            fn explicit_and_used_specify_explicit_bounds<'a, T: std::fmt::Display>(v: &'a T) {
                println!("{}", v)
            }
        );
    }

    pub mod explicit_and_returned {
        use hooks_core::hook_fn;

        hook_fn!(
            fn auto_bounds<'a, T: 'a>(v: &'a T) -> &'a T {
                v
            }
        );

        hook_fn!(
            type Bounds = impl '_;
            fn specify_elided_bounds<'a, T: 'a>(v: &'a T) -> &'a T {
                v
            }
        );

        hook_fn!(
            type Bounds = impl 'a;
            fn specify_explicit_bounds<'a, T: 'a>(v: &'a T) -> &'a T {
                v
            }
        );
    }
}

pub mod outer_lifetimes {
    pub mod with_self_receiver {
        use hooks_core::hook_fn;

        pub struct Data<'a>(&'a str);

        impl<'outer> Data<'outer> {
            hook_fn!(
                fn use_argument(v: &'outer str) {
                    println!("{}", v);
                }
            );

            hook_fn!(
                fn use_with_self_receiver(self) {
                    println!("{}", self.0);
                }
            );

            hook_fn!(
                fn use_with_self_type(this: Self) {
                    println!("{}", this.0);
                }
            );

            hook_fn!(
                fn use_with_self_receiver_rc(self: std::rc::Rc<Self>) {
                    println!("{}", self.0);
                }
            );

            hook_fn!(
                fn use_with_self_type_rc(this: std::rc::Rc<Self>) {
                    println!("{}", this.0);
                }
            );
        }
    }
}
