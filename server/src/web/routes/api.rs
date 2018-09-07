//! API routes, which get mounted at `/api`.

use futures::{future::ok, prelude::*};
use warp::{
    self,
    http::{Response, StatusCode},
    Filter, Rejection,
};

use api;
use types::AuthCheckRequest;
use web::{middleware, Resp};
use HandlerContext;

/// Returns the routes that get mounted at `/api`.
pub fn routes(ctx: HandlerContext) -> Resp {
    let auth_check = path!("auth" / "check").and(auth_check(ctx.clone()));
    let ping = path!("ping").and(ping(ctx));

    auth_check.or(ping).unify().boxed()
}

/// The `POST /api/auth/check` route.
pub fn auth_check(ctx: HandlerContext) -> Resp {
    warp::index()
        .and(warp::post2())
        .and(middleware::capabilities(&ctx, &["capabilities.check"]))
        .and(middleware::body())
        .and_then(move |auth, req: AuthCheckRequest| {
            api::auth_check(&ctx, &req.token, &req.capabilities).then(|r| -> Result<_, Rejection> {
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
pub fn ping(_ctx: HandlerContext) -> Resp {
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
