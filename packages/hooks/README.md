# `::hooks`

[![Crates.io](https://img.shields.io/crates/v/hooks?style=for-the-badge)](https://crates.io/crates/hooks)
[![docs.rs](https://img.shields.io/docsrs/hooks/latest?style=for-the-badge)](https://docs.rs/hooks)
[![GitHub license](https://img.shields.io/github/license/frender-rs/hooks?style=for-the-badge)](https://github.com/frender-rs/hooks/blob/main/LICENSE)
[![GitHub stars](https://img.shields.io/github/stars/frender-rs/hooks?style=for-the-badge)](https://github.com/frender-rs/hooks/stargazers)

Compile-time, async hooks in safe Rust.

## Quick Start

Run `cargo add hooks` to add `hooks` to your project.

Note that this project is still in alpha,
and it may have BREAKING CHANGES.

Please see [changelogs](https://github.com/frender-rs/hooks/blob/alpha/packages/hooks/CHANGELOG.md) before upgrading.

```rust
use hooks::hook;

#[hook]
fn use_demo() {
    let (state, updater) = hooks::use_state(0);

    let updater = updater.clone();

    hooks::use_effect(move |v: &i32| {
        println!("state = {}", *v);

        if *v < 2 {
          updater.set(*v + 1);
        }
    }, *state);
}

fn main() {
    futures_lite::future::block_on(async {
        use hooks::HookExt;

        let mut hook = use_demo();

        while let Some(()) = hook.next_value(()).await {}
    });
}
```

You will see the following logs. Then the program exits gracefully because
it knows there won't be new values.

```txt
state = 0
state = 1
state = 2
```

## What is a compile-time `Hook`?

<details>
<summary>
To understand the concepts behind this crate,
you can expand this section and read the details.
</summary>

Hooks, introduced by [React 16.8](https://reactjs.org/docs/hooks-intro.html),
is a way to bring _state_ into functional components.
Hooks can make stateless functional components stateful, and reactive.

Conventional hook implementations uses a global state to record hook calls and their states.
Thus, the order of hook calls must not change; conditional hook calls are also forbidden.
Developers must follow [`Rules of Hooks`](https://reactjs.org/docs/hooks-rules.html)
to write a valid custom hook.
[`yew.rs`](https://yew.rs/docs/concepts/function-components/custom-hooks) also uses a global
[`CURRENT_HOOK`](https://docs.rs/yew/0.19.3/src/yew/functional/hooks/mod.rs.html#36)
to implement hooks.
We can see the above implementation relies on runtime behavior of a hook fn.
The hook runner must run the hook fn once to know what is initialized.
We call this _runtime hooks_.

Rust language has powerful static type systems,
it can do a lot of things at compile time.
We can abstract hook behavior as a `Hook` trait, and then
the `fn`s that returns `impl Hook` are hook functions.
We can have a [`hook`] macro to help developers write custom hooks.
We call this kind of hook implementation as _compile-time hooks_.

This crate defines and implements _compile-time hooks_ for you.

When a type implements [`Hook<Args>`], it defines three behaviors:

1. What arguments does this hook accept?

   It accepts `Args` as argument of [`Hook::use_hook`].

2. When using this hook, what does it output?

   [`Hook::use_hook`] returns [`HookLifetime::Value`].

   This crate uses _GAT (Generic Associated Types)_ to allow the output type borrowing from the hook itself.
   To support rust versions before 1.65, this crate uses
   [_better GAT_ pattern](https://sabrinajewson.org/blog/the-better-alternative-to-lifetime-gats) introduced by _Sabrina Jewson_. Thanks to her!
   Due to this pattern, an extra trait [`HookLifetime`] is needed.

3. When should we re-use this hook?

   Hooks have states. When the state doesn't change, we don't need to re-call `use_hook` to get the new output.
   We can wait for the hook's state to change with [`HookPollNextUpdate::poll_next_update`],
   or by just awaiting [`hook.next_update()`](HookPollNextUpdateExt::next_update).

   To wait for the next value when state changes,
   you can use [`HookExt::next_value*`](HookExt::next_value) methods.

</details>

## How to write a custom hook?

Please see [`Hook#how-to-impl-hook`] trait.

### How to use hooks in a hook function?

With [`hook`] macro, you can just call `use_another_hook(arg0, arg1)` at
_top level_ (not in an inner block like `match`, `if`, or `while`).
The macro will transform the call.
You can see the [snapshots](https://github.com/frender-rs/hooks/blob/alpha/packages/hooks-test/tests/snapshots) for what this macro outputs.

### How to conditionally use hooks?

Please see [`use_lazy_pinned_hook`] and [`use_default_pinned_hook`].

## How to use the hook when not writing custom hooks?

To consume a hook, you can use it with [`HookExt::next_value*`](HookExt::next_value).

If a hook accepts `()` as `Args`, then it is an [`AsyncIterableHook`].
You can turn it into a [`IterHook`](::hooks_core::IterHook) by calling
[`hook.into_iter()`](AsyncIterableHook::into_iter) or [`hook.iter_mut()`](AsyncIterableHook::iter_mut).
An `IterHook` is actually a `LendingAsyncIterator`.
If the output type doesn't borrow from the hook itself.
It is also an [`AsyncIterator`](std::async_iter::AsyncIterator) or [`Stream`](https://docs.rs/futures-core/latest/futures_core/stream/trait.Stream.html).

```rust
# use hooks::hook;
#[hook]
fn use_demo() -> i32 {
    let (state, updater) = hooks::use_state(0);

    let updater = updater.clone();
    hooks::use_effect(move |_: &i32| {
        updater.replace_maybe_with_fn_pointer(
            |v| if *v < 2 { Some(*v + 1) } else { None }
        );
    }, *state);

    *state
}

// with HookExt::next_value
futures_lite::future::block_on(async {
    use hooks::HookExt;

    let mut hook = use_demo();
    assert_eq!(hook.next_value(()).await, Some(0));
    assert_eq!(hook.next_value(()).await, Some(1));
    assert_eq!(hook.next_value(()).await, Some(2));
    assert_eq!(hook.next_value(()).await, None);
});

// with AsyncIterableHook::into_iter
futures_lite::future::block_on(async {
    use hooks::AsyncIterableHook;

    let mut hook = use_demo().into_iter();
    assert_eq!(hook.next_value().await, Some(0));
    assert_eq!(hook.next_value().await, Some(1));
    assert_eq!(hook.next_value().await, Some(2));
    assert_eq!(hook.next_value().await, None);
});

// with AsyncIterableHook::into_iter and Stream
futures_lite::future::block_on(async {
    use hooks::AsyncIterableHook;
    use futures_lite::StreamExt;

    let mut hook = use_demo().into_iter();

    let values: Vec<_> = hook.collect().await;
    assert_eq!(values, [0, 1, 2]);
});
```
