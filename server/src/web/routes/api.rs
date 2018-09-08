//! API routes, which get mounted at `/api`.

use futures::{future::ok, prelude::*};
use serde_json;
use warp::{
    self,
    http::{Response, StatusCode},
    Filter, Rejection,
};

use types::{AuthCheckRequest, AuthError};
use web::{middleware, Resp};
use {auth_check, HandlerContext};

/// Returns the routes that get mounted at `/api`.
pub fn routes(ctx: HandlerContext) -> Resp {
    let get_ping = path!("ping").and(get_ping(ctx.clone()));
    let post_auth_check = path!("auth" / "check").and(post_auth_check(ctx));

    get_ping.or(post_auth_check).unify().boxed()
}

/// The `POST /api/auth/check` route.
pub fn post_auth_check(ctx: HandlerContext) -> Resp {
    warp::index()
        .and(warp::post2())
        .and(warp::cookie::optional("auth"))
        .and(middleware::body())
        .and_then(move |auth, req: AuthCheckRequest| {
            middleware::capabilities(&ctx, auth, caps!["capabilities.check"])
                .and_then(|()| {
                    auth_check(&ctx, &req.token, req.capabilities).then(|r| {
                        let body = match r {
                            Ok(()) => json!({ "type": "ok" }),
                            Err(e) => match_coproduct!(e, {
                                err : AuthError => { serde_json::to_value(err).unwrap() }
                            }),
                        };
                        Ok((StatusCode::OK, body))
                    })
                })
                .then(|r| match r {
                    Ok((status, body)) => middleware::simple_response(status, body),
                    Err(e) => Ok(e),
                })
        })
        .boxed()
}

/// The `GET /api/ping` route.
pub fn get_ping(_ctx: HandlerContext) -> Resp {
    warp::index()
        .and(warp::get2())
        .map(|| {
            Response::builder()
                .status(StatusCode::NO_CONTENT)
                .body(String::new())
                .unwrap()
        })
        .boxed()
}
