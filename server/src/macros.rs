//! Macros defined here mainly to make other code look pretty.

/// Does a foldr1 with `Filter::or` over the provided expressions.
macro_rules! foldr1_filter_or {
    ($e:expr) => { $e };
    ($h:expr $(,$t:expr)* $(,)*) => {
        $crate::warp::Filter::or($h, foldr1_filter_or!($($t),*))
    };
}

/// Composes several routes with `Filter::or`.
macro_rules! routes {
    (in $m:ident; with $ctx:ident; $e:ident $(,)*) => {
        $crate::web::routes::$m::$e($ctx.clone())
    };
    (in $m:ident; with $ctx:ident; $h:ident $(, $t:ident)* $(,)*) => {
        $crate::warp::Filter::or(routes!(in $m; with $ctx; $h), routes!(in $m; with $ctx; $($t),*))
    };
}

/// Creates a path-based router using `warp::path!`.
macro_rules! router {
    (with $ctx:ident; $($base:tt => $m:ident::[$($n:ident $(,)*)*])*) => {
        foldr1_filter_or!($(routes! { in $m; with $ctx; $($n),* }),*)
    };
}
