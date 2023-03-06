use std::pin::Pin;

#[derive(Debug, Default)]
pub struct Memo<Data, Dep> {
    pub data: Data,
    pub dependency: Dep,
}

impl<Data, Dep> Unpin for Memo<Data, Dep> {}

impl<Data, Dep> Memo<Data, Dep> {
    pub fn update_if_ne_and_get(
        &mut self,
        get_data: impl FnOnce(&Dep) -> Data,
        new_dependency: Dep,
    ) -> (&Data, &Dep)
    where
        Dep: PartialEq,
    {
        let Self { data, dependency } = self;
        if *dependency != new_dependency {
            *data = get_data(&new_dependency);
            *dependency = new_dependency;
        }
        (data, dependency)
    }
}

hooks_core::impl_hook![
    type For<Data, Dep> = Memo<Data, Dep>;
    fn unmount() {}
    #[inline]
    fn poll_next_update(self) {
        std::task::Poll::Ready(false)
    }
    #[inline]
    fn use_hook(self) -> (&'hook mut Data, &'hook mut Dep) {
        let this = self.get_mut();
        (&mut this.data, &mut this.dependency)
    }
];

#[derive(Debug)]
pub struct MemoUninitialized<Data, Dep>(Option<Memo<Data, Dep>>);

impl<Data, Dep> Unpin for MemoUninitialized<Data, Dep> {}

impl<Data, Dep> Default for MemoUninitialized<Data, Dep> {
    fn default() -> Self {
        Self(None)
    }
}

hooks_core::impl_hook![
    type For<Data, Dep> = MemoUninitialized<Data, Dep>;
    fn unmount() {}
    #[inline]
    fn poll_next_update(self) {
        std::task::Poll::Ready(false)
    }
];

impl<Data, Dep> MemoUninitialized<Data, Dep> {
    /// `f` is required to return the initialized mutable reference,
    /// so we can make sure use_memo is initialized in compile time.
    pub fn update_and_get<'hook>(
        self: Pin<&'hook mut Self>,
        f: impl FnOnce(&mut Option<Memo<Data, Dep>>) -> &mut Memo<Data, Dep>,
    ) -> (&'hook Data, &'hook Dep) {
        let this = self.get_mut();
        f(&mut this.0); // not using the returned reference because it might be a field.

        let dd = this.0.as_mut().expect("use_memo not initialized");
        (&dd.data, &dd.dependency)
    }

    pub fn update_if_ne_and_get<'hook>(
        self: Pin<&'hook mut Self>,
        get_data: impl FnOnce(&Dep) -> Data,
        dep: Dep,
    ) -> (&'hook Data, &'hook Dep)
    where
        Dep: PartialEq,
    {
        let dd = &mut self.get_mut().0;
        let dd = if let Some(dd) = dd {
            if dd.dependency != dep {
                dd.data = get_data(&dep);
            }
            dd.dependency = dep;
            dd
        } else {
            let data = get_data(&dep);
            dd.insert(Memo {
                data,
                dependency: dep,
            })
        };

        (&dd.data, &dd.dependency)
    }
}

pub struct UseMemo<Data, Dep: PartialEq, F: FnOnce(&Dep) -> Data>(pub F, pub Dep);
pub use UseMemo as use_memo;

hooks_core::impl_hook![
    type For<Data, Dep: PartialEq, F> = UseMemo<Data, Dep, F>
        where __![F: FnOnce(&Dep) -> Data]: __;

    fn into_hook(self) -> Memo<Data, Dep> {
        Memo {
            data: self.0(&self.1),
            dependency: self.1,
        }
    }

    fn update_hook(self, hook: _) {
        let _ = hook.get_mut().update_if_ne_and_get(self.0, self.1);
    }

    fn h(self, hook: MemoUninitialized<Data, Dep>) {
        let Memo { data, dependency } = hook.get_mut().0.get_or_insert_with(|| Memo {
            data: self.0(&self.1),
            dependency: self.1,
        });
        (data, dependency)
    }
];
