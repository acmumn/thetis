mod capabilities;

use std::collections::HashSet;
use std::iter::FromIterator;

use futures::{
    future::{err, ok, Either},
    prelude::*,
};
use jsonwebtoken::{self, errors::ErrorKind, Algorithm, Validation};

use types::{AuthError, MemberID};
use HandlerContext;

/// The `/api/auth/check` call.
pub fn auth_check<C: AsRef<str>, I: IntoIterator<Item = C>>(
    ctx: &HandlerContext,
    token: &str,
    caps: I,
) -> impl Future<Item = (), Error = Coprod!(AuthError)> {
    lazy_static! {
        static ref VALIDATION: Validation = Validation::new(Algorithm::HS512);
    }

    let caps_wanted = caps.into_iter().map(|c| c.as_ref()).collect::<HashSet<_>>();
    match jsonwebtoken::decode::<Claims>(token, ctx.jwt_secret.as_bytes(), &VALIDATION) {
        Ok(tok) => match tok.claims.inner {
            ClaimsInner::Service { name, caps } => {
                let caps = caps.into_iter().map(|c| c.as_ref()).collect::<HashSet<_>>();
                if caps_wanted.is_subset(&caps) {
                    Either::A(ok(()))
                } else {
                    warn!("Service {} tried to use unauthorized capabilities:", name);
                    warn!("    Have capabilities {:?}", caps);
                    warn!("    Tried to use {:?}", caps_wanted);
                    Either::A(err(unimplemented!()))
                }
            }
            ClaimsInner::User { id } => {
                unimplemented!();
            }
        },
        Err(e) => {
            error!("Got invalid JWT: {}", e);
            Either::A(err(if let &ErrorKind::ExpiredSignature = e.kind() {
                unimplemented!()
            } else {
                unimplemented!()
            }))
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    exp: i64,
    iat: i64,
    #[serde(flatten)]
    inner: ClaimsInner,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum ClaimsInner {
    User { id: MemberID },
    Service { name: String, caps: Vec<String> },
}
