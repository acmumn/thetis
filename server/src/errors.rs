use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::Error as IoError;

pub use auth::capabilities::cst::ParseError;

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

define_error!(CapabilitiesLoadError {
    Io(IoError, err => err),
    Parse(ParseError, err => err.clone().map_token(|(n,t)| t)),
});
