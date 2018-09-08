//! Macros defined here mainly to make other code look pretty.

macro_rules! caps {
    ($($e:expr),*) => {{
        let mut set = $crate::std::collections::HashSet::new();
        $($crate::std::collections::HashSet::insert(&mut set,
                                                    $crate::std::borrow::ToOwned::to_owned($e));)*
        set
    }};
}

macro_rules! match_coproduct {
    ($e:expr, {}) => {
        match $e {}
    };
    (
        $e:expr, { $hi:ident : $ht:ty => $hb:block $(,)* $($ti:ident : $tt:ty => $tb:block $(,)*)* }
    ) => {
        match $crate::frunk::Coproduct::uninject::<$ht, _>($e) {
            Ok($hi) => $hb,
            Err(e) => match_coproduct!(e, {$($ti : $tt => $tb),*}),
        }
    };
}
