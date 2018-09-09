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

macro_rules! define_error {
    ($name:ident { $($n:ident($t:ty, $dn:ident => $d:expr)),* $(,)* }) => {
        #[derive(Debug, Fail)]
        pub enum $name {
            $($n($t)),*
        }

        impl Display for $name {
            fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
                match *self {
                    $($name::$n(ref $dn) => write!(fmt, "{}", $d)),*
                }
            }
        }

        $(impl From<$t> for $name {
            fn from(t: $t) -> $name {
                $name::$n(t)
            }
        })*
    };
}

macro_rules! impl_WebError_for_Serialize {
    ($t:ty, $code:expr) => {
        impl WebError for $t {
            fn to_status_body(self) -> (StatusCode, Value) {
                match serde_json::to_value(&self) {
                    Ok(json) => ($code, json),
                    Err(e) => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        json!({ "type": "serde", "msg": e.to_string() }),
                    ),
                }
            }
        }
    };
}
