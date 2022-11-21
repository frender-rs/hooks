use std::pin::Pin;

use hooks_core::{Hook, HookLifetime};

#[derive(Debug)]
#[non_exhaustive]
pub struct NoUpdate;

#[derive(Debug)]
pub struct DataAndDep<Data, Dep> {
    pub data: Data,
    pub dep: Dep,
}

#[derive(Debug)]
pub struct Memo<Data, Dep>(Option<DataAndDep<Data, Dep>>);

impl<Data, Dep> Default for Memo<Data, Dep> {
    #[inline]
    fn default() -> Self {
        Self(None)
    }
}

impl<Data, Dep> Unpin for Memo<Data, Dep> {}

impl<Data, Dep> Memo<Data, Dep> {
    /// `f` is required to return the initialized mutable reference,
    /// so we can make sure use_memo is initialized in compile time.
    pub fn use_hook_with<'hook>(
        self: Pin<&'hook mut Self>,
        f: impl FnOnce(&mut Option<DataAndDep<Data, Dep>>) -> &mut DataAndDep<Data, Dep>,
    ) -> (&'hook Data, &'hook Dep) {
        let this = self.get_mut();
        f(&mut this.0); // not using the returned reference because it might be a field.

        let dd = this.0.as_mut().expect("use_memo not initialized");
        (&dd.data, &dd.dep)
    }

    pub fn use_hook_eq<'hook>(
        self: Pin<&'hook mut Self>,
        get_data: impl FnOnce(&Dep) -> Data,
        dep: Dep,
    ) -> (&'hook Data, &'hook Dep)
    where
        Dep: PartialEq,
    {
        let dd = &mut self.get_mut().0;
        let dd = if let Some(dd) = dd {
            if dd.dep != dep {
                dd.data = get_data(&dep);
            }
            dd.dep = dep;
            dd
        } else {
            let data = get_data(&dep);
            dd.insert(DataAndDep { data, dep })
        };

        (&dd.data, &dd.dep)
    }
}

crate::utils::impl_hook! {
    impl [Data, Dep] for Memo<Data, Dep> {
        #[inline]
        poll_next_update(self) {
            self.0.is_none().into()
        }
        #[inline]
        use_hook[
            F: FnOnce(&mut Option<DataAndDep<Data, Dep>>) -> &mut DataAndDep<Data, Dep>,
        ](self, f: F) -> (&'hook Data, &'hook Dep) {
            self.use_hook_with(f)
        }
    }
}

impl<'hook, Data, Dep: PartialEq, F: FnOnce(&Dep) -> Data> HookLifetime<'hook, (F, Dep)>
    for Memo<Data, Dep>
{
    type Value = (&'hook Data, &'hook Dep);
}

impl<Data, Dep: PartialEq, F: FnOnce(&Dep) -> Data> Hook<(F, Dep)> for Memo<Data, Dep> {
    #[inline]
    fn use_hook<'hook>(
        self: Pin<&'hook mut Self>,
        (get_data, dep): (F, Dep),
    ) -> <Self as hooks_core::HookLifetime<'hook, (F, Dep)>>::Value
    where
        Self: 'hook,
    {
        self.use_hook_eq(get_data, dep)
    }
}

#[inline]
pub fn use_memo<Data, Dep>() -> Memo<Data, Dep> {
    Default::default()
}

#[inline]
pub fn memo_with<Data, Dep>(
    f: impl FnOnce(&mut Option<DataAndDep<Data, Dep>>) -> &mut DataAndDep<Data, Dep>,
) -> impl FnOnce(&mut Option<DataAndDep<Data, Dep>>) -> &mut DataAndDep<Data, Dep> {
    f
}
