//! Authentication functions.

pub mod capabilities;

use std::collections::HashSet;

use fall::ResolutionError;
use frunk::Coproduct;
use futures::{
    future::{err, ok, Either},
    prelude::*,
};
use jsonwebtoken::{self, errors::ErrorKind, Algorithm, Validation};

use errors::DatabaseError;
use types::{AuthError, MemberID};
use Context;

/// Checks if an authentication token has the given capabilities.
pub fn check<I: IntoIterator<Item = String>>(
    ctx: &Context,
    token: &str,
    caps: I,
) -> impl Future<Item = (), Error = Coprod!(AuthError, DatabaseError, ResolutionError)> {
    lazy_static! {
        static ref VALIDATION: Validation = Validation::new(Algorithm::HS512);
    }

    match jsonwebtoken::decode::<ClaimsOuter>(token, ctx.jwt_secret.as_bytes(), &VALIDATION) {
        Ok(tok) => Either::A(tok.claims.inner.check(ctx, caps)),
        Err(e) => {
            error!("Got invalid JWT: {}", e);
            Either::B(err(Coproduct::inject(
                if let &ErrorKind::ExpiredSignature = e.kind() {
                    AuthError::Expired
                } else {
                    AuthError::Invalid
                },
            )))
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct ClaimsOuter {
    exp: i64,
    iat: i64,
    #[serde(flatten)]
    inner: Claims,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Claims {
    User { id: MemberID },
    Service { name: String, caps: Vec<String> },
}

impl Claims {
    /// Checks if a `Claims` value has the given capabilities.
    pub fn check<I: IntoIterator<Item = String>>(
        &self,
        ctx: &Context,
        caps: I,
    ) -> impl Future<Item = (), Error = Coprod!(AuthError, DatabaseError, ResolutionError)> {
        let caps_wanted = caps.into_iter().collect::<HashSet<_>>();
        match *self {
            Claims::Service { name, caps } => {
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
                    Either::A(err(Coproduct::inject(AuthError::CapabilitiesRequired(
                        caps_missing,
                    ))))
                }
            }
            Claims::User { id } => {
                let caps = caps_wanted.iter().cloned().collect();
                Either::B(
                    capabilities::check(ctx.clone(), id, caps).then(|r| match r {
                        Ok(true) => Ok(()),
                        Ok(false) => Err(Coproduct::inject(AuthError::CapabilitiesRequired(
                            caps_wanted,
                        ))),
                        Err(e) => Err(Coproduct::embed(e)),
                    }),
                )
            }
        }
    }
}
