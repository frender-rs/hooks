#[macro_export]
macro_rules! tt_matches {
    ($dollar:tt [$($tt1:tt)*][$($tt2:tt)*]) => {
        macro_rules! impl_report_wrong_tt {
            ($($tt1)*) => {};
            ($dollar ($wrong_tt:tt)*) => {
                ::core::compile_error!{::core::concat!(
                    "expect `",
                    ::core::stringify!($($tt1)*),
                    "`\n",
                    "found `",
                    ::core::stringify!($dollar ($wrong_tt)*),
                    "`",
                )}
            };
        }

        macro_rules! impl_tt_matches {
            ($($tt1)*) => {};
        }

        impl_tt_matches! {$($tt2)*}
        impl_report_wrong_tt! {$($tt2)*}
    };
}

macro_rules! fn_path_matches {
    ([$fn1:path][$fn2:path]) => {
        const _: () = {
            let mut _v = $fn1;
            _v = $fn2;
        };
    };
}

#[cfg(test)]
mod tests {
    //!
    //!
    //! ```
    //! fn_path_matches! {[my_fn][my_fn]}
    //! ```

    tt_matches! {$[][]}
    tt_matches! {$[1][1]}
    tt_matches! {$[1 ""][1""]}
    tt_matches! {$[matched!()][matched ! ()]}

    fn my_fn<V>() {
        let mut v = my_fn::<u32>;
        v = my_fn;
    }

    fn_path_matches! {[my_fn::<i32>][my_fn]}
}
