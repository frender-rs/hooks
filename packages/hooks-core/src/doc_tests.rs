/// ```compile_fail
/// use hooks_core::hook_fn;
/// hook_fn!(
///     fn use_hook(v: &str) {
///         println!("{}", v);
///     }
/// );
/// ```
///
/// `let _ = v` or `_ = v` are not using `v`.
///
/// ```
/// use hooks_core::hook_fn;
/// hook_fn!(
///     fn use_hook_1(v: &str) {
///         let _ = v;
///     }
/// );
///
/// hook_fn!(
///     fn use_hook_2(v: &str) {
///         _ = v
///     }
/// );
/// ```
pub enum ArgumentsWithElidedLifetimeAndUsedMustSpecifyBounds {}

/// ```compile_fail
/// use hooks_core::hook_fn;
/// hook_fn!(
///     fn use_hook(_: &()) -> &str {
///         ""
///     }
/// );
/// ```
///
/// ```
/// use hooks_core::hook_fn;
/// hook_fn!(
///     fn use_hook<'a>(_: &'a ()) -> &'a str {
///         ""
///     }
/// );
/// ```
pub enum ReturnValueMustNotHaveElidedLifetimes {}

/// ```compile_fail
/// use hooks_core::hook_fn;
/// hook_fn!(
///     fn use_hook<T: std::fmt::Display>(v: &T) {
///         println!("{}", v);
///     }
/// );
/// ```
pub enum GenericArgumentsWithElidedLifetimeAndUsedMustSpecifyBounds {}

/// ```compile_fail
/// use hooks_core::hook_fn;
///
/// struct Data<'a>(&'a ());
///
/// impl<'a> Data<'a> {
///     hook_fn!(
///         fn use_hook(v: &'a ()) {
///             _ = (v,)
///         }
///     );
/// }
/// ```
///
/// ```
/// use hooks_core::hook_fn;
///
/// struct Data<'a>(&'a ());
///
/// impl<'a> Data<'a> {
///     hook_fn!(
///         type Bounds = impl 'a;
///         fn use_hook(v: &'a ()) {
///             _ = (v,)
///         }
///     );
/// }
/// ```
pub enum ArgumentsWithOuterLifetimeAndUsedMustSpecifyBounds {}
