//! API routes, which get mounted at `/api`.

use futures::{future::ok, prelude::*};
use warp::{
    self,
    http::{Response, StatusCode},
    Filter, Rejection,
};

use types::AuthCheckRequest;
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
        .and(middleware::capabilities(&ctx, &["capabilities.check"]))
        .and(middleware::body())
        .and_then(move |(), req: AuthCheckRequest| {
            auth_check(&ctx, &req.token, req.capabilities).then(|r| -> Result<_, Rejection> {
                Ok(match r {
                    Ok(()) => (StatusCode::OK, json!({ "type": "ok" })),
                    Err(e) => unimplemented!("{:?}", e),
                })
            })
        })
        .and_then(middleware::serialize)
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
