use crate::utils::UninitializedHook;

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

pub struct UseMemo<Data, Dep: PartialEq, F: FnOnce(&Dep) -> Data>(pub F, pub Dep);
pub use UseMemo as use_memo;

hooks_core::impl_hook![
    type For<Data, Dep: PartialEq, F: FnOnce(&Dep) -> Data> = UseMemo<Data, Dep, F>;

    fn into_hook(self) -> Memo<Data, Dep> {
        Memo {
            data: self.0(&self.1),
            dependency: self.1,
        }
    }

    fn update_hook(self, hook: _) {
        let _ = hook.get_mut().update_if_ne_and_get(self.0, self.1);
    }

    fn h(self, hook: UninitializedHook<Memo<Data, Dep>>) {
        hook.get_mut().use_into_or_update_hook(self)
    }
];
