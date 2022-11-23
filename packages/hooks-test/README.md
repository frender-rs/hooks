This is the testing package for `::hooks`.

Currently returning references with `'hook` lifetime fails to compile

```compile_fail
# use hooks::hook;
#[hook]
fn use_hook_lt() -> &'hook impl Display {
    static VALUE: i32 = 2;
    &VALUE
}
```

```compile_fail
# use hooks::hook;
#[hook]
fn use_hook_lt() -> &'hook (impl Display + 'hook) {
    static VALUE: i32 = 2;
    &VALUE
}
```
