mod capabilities;

use std::collections::HashSet;

use frunk::Coproduct;
use futures::{
    future::{err, ok, Either},
    prelude::*,
};
use jsonwebtoken::{self, errors::ErrorKind, Algorithm, Validation};

use types::{AuthError, MemberID};
use HandlerContext;

/// The `/api/auth/check` call.
pub fn auth_check<I: IntoIterator<Item = String>>(
    ctx: &HandlerContext,
    token: &str,
    caps: I,
) -> impl Future<Item = (), Error = Coprod!(AuthError)> {
    lazy_static! {
        static ref VALIDATION: Validation = Validation::new(Algorithm::HS512);
    }

    let caps_wanted = caps.into_iter().collect::<HashSet<_>>();
    let fut = match jsonwebtoken::decode::<Claims>(token, ctx.jwt_secret.as_bytes(), &VALIDATION) {
        Ok(tok) => match tok.claims.inner {
            ClaimsInner::Service { name, caps } => {
                let mut caps_missing = caps_wanted;
                for cap in &caps {
                    caps_missing.remove(cap);
                }
                if caps_missing.is_empty() {
                    Either::A(ok(()))
                } else {
                    warn!("Service {} tried to use unauthorized capabilities:", name);
                    warn!("    Have capabilities {:?}", caps);
                    warn!("    Was missing {:?}", caps_missing);
                    Either::A(err(AuthError::CapabilitiesRequired(caps_missing)))
                }
            }
            ClaimsInner::User { id } => {
                unimplemented!();
                Either::B(ok(()))
            }
        },
        Err(e) => {
            error!("Got invalid JWT: {}", e);
            Either::A(err(if let &ErrorKind::ExpiredSignature = e.kind() {
                AuthError::Expired
            } else {
                AuthError::Invalid
            }))
        }
    };
    fut.map_err(Coproduct::inject)
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
