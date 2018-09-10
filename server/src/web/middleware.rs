use std::collections::HashSet;
use std::fmt::Display;

use frunk::Coproduct;
use futures::{
    future::{err, Either},
    prelude::*,
};
use mime;
use serde::{de::DeserializeOwned, Serialize};
use serde_json;
use warp::{
    self,
    http::{header, Response, StatusCode},
    Filter, Rejection,
};

use errors::WebError;
use types::AuthError;
use {auth_check, Context};

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

/// A helper to check for capabilities in an `auth` cookie, failing if the cookie is not present.
pub fn capabilities<C: AsRef<str>, I: IntoIterator<Item = String>>(
    ctx: &Context,
    auth_cookie: Option<C>,
    caps: I,
) -> impl Future<Item = (), Error = impl WebError> + Send {
    if let Some(auth_cookie) = auth_cookie {
        let caps = caps.into_iter().collect::<HashSet<_>>();
        Either::A(auth_check(ctx, auth_cookie.as_ref(), caps.clone()))
    } else {
        Either::B(err(Coproduct::inject(AuthError::AuthTokenRequired)))
    }
}

/// A helper for passing to `.map_err` that prints the error and rejects with a server error.
pub fn log_server_error<T: Display>(t: T) -> Rejection {
    error!("{}", t);
    warp::reject::server_error()
}

/// A helper that serializes a response code and value to a real response.
pub fn simple_response<T: Serialize>(
    status: StatusCode,
    body: T,
) -> Result<Response<String>, Rejection> {
    // TODO: In theory, this should check on the Accept header, and use that to make decisions.

    serde_json::to_string(&body)
        .map_err(log_server_error)
        .and_then(|body| {
            Response::builder()
                .status(status)
                .body(body)
                .map_err(log_server_error)
        })
}
