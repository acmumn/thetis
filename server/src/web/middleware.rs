use std::sync::Arc;

use futures::{future::ok, prelude::*};
use mime::{self, Mime};
use serde::{de::DeserializeOwned, Serialize};
use serde_json;
use warp::{
    self,
    http::{header, Response, StatusCode},
    Filter, Rejection,
};

use api::auth_check;
use web::Resp;
use HandlerContext;

/// A filter that parses a JSON or form body.
pub fn body<T: DeserializeOwned + Send>() -> impl Filter<Extract = (T,), Error = Rejection> + Copy {
    // TODO: This is technically not the semantics we want; this will accept a JSON body with a
    // content-type of application/x-www-form-urlencoded. (But not vice versa, at least.)

    warp::header::exact(
        header::CONTENT_TYPE.as_ref(),
        mime::APPLICATION_WWW_FORM_URLENCODED.as_ref(),
    ).and(warp::body::form())
        .or(warp::body::json())
        .unify()
}

/// A filter to check for capabilities in an `auth` cookie, failing if the cookie is not present.
pub fn capabilities<C, I>(
    ctx: &HandlerContext,
    caps: I,
) -> impl Filter<Extract = ((),), Error = Rejection> + Clone
where
    C: AsRef<str> + Send + Sync,
    I: IntoIterator<Item = C>,
{
    let caps = Arc::from(caps.into_iter().collect::<Vec<C>>());
    let ctx = ctx.clone();

    warp::cookie("auth").and_then(move |auth: String| {
        auth_check(&ctx, &auth, &*caps).map_err(|e| -> Rejection { unimplemented!("{:?}", e) })
    })
}

/// A function that serializes a response code and value to a real response.
pub fn serialize<T: Serialize>(
    (code, body): (StatusCode, T),
) -> impl Future<Item = Response<String>, Error = Rejection> {
    // TODO: In theory, this should check on the Accept header, and use that to make decisions.

    serde_json::to_string(&body)
        .map_err(|e| unimplemented!("{:?}", e))
        .and_then(|body| {
            Response::builder()
                .status(code)
                .body(body)
                .map_err(|e| unimplemented!("{:?}", e))
        })
        .into_future()
}
