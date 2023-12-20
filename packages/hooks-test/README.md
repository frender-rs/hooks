This is the testing package for `::hooks`.

## borrowing arguments in used hooks

Currently borrowing arguments in used hooks only compiles with rust 1.74.0 and later versions.
See issue #103532 <https://github.com/rust-lang/rust/issues/103532> for more information.

```rust
# use hooks::{use_effect, hook};
/// Print debug on `value` change.
#[hook(bounds = "'a")]
fn use_debug<'a, T: std::fmt::Debug + PartialEq + 'a>(value: &'a T) {
    use_effect(|v: &_| {
        println!("{v:?}");
    }, value);
}
```

> error[E0658]: `impl Trait` return type cannot contain a projection or `Self` that references lifetimes from a parent scope
>
> note: see issue #103532 <https://github.com/rust-lang/rust/issues/103532> for more information

A workaround is to remove the lifetime and accept anything that `Deref<Target = T>`, including `&T`.

```rust
# use hooks::{use_effect, hook};
/// Print debug on `value` change.
#[hook]
fn use_debug<
    T: std::fmt::Debug + PartialEq,
    R: std::ops::Deref<Target = T> + PartialEq + Copy,
>(value: R) {
    use_effect(|v: &R| {
        let v: &T = &**v;
        println!("{v:?}");
    }, value);
}
```

## `impl Trait + 'hook`

Currently returning impl Trait with `'hook` lifetime fails to compile.

```rust,compile_fail
# use hooks::hook;
#[hook]
fn use_hook_lt() -> &'hook (impl ToString + 'hook) {
    static VALUE: i32 = 2;
    &VALUE
}
```

> error[E0261]: use of undeclared lifetime name `'hook`
>
> error: higher kinded lifetime bounds on nested opaque types are not supported yet
