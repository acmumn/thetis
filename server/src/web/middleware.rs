use futures::{future::ok, prelude::*};
use serde::de::DeserializeOwned;
use warp::{self, Filter, Rejection};

use web::HandlerContext;

pub fn body<T: DeserializeOwned + Send>() -> impl Filter<Extract = (T,), Error = Rejection> + Copy {
    warp::body::json()
}

/// A middleware to check for capabilities in an `auth` cookie, failing if the cookie is not
/// present.
pub fn capabilities<C, I>(
    ctx: &HandlerContext,
    caps: I,
) -> impl Filter<Extract = ((),), Error = Rejection> + Copy
where
    C: AsRef<str>,
    I: IntoIterator<Item = C>,
{
    warp::cookie("auth")
        .and_then(|auth| -> Result<_, warp::Rejection> {
            //middleware::capabilities(ctx.db.clone(), auth, ["capabilities.check"])
            Ok(unimplemented!())
        })
        .map(|()| ())
}
