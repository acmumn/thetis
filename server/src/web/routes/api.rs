//! API routes, which get mounted at `/api`.

use warp::{
    self,
    http::{Response, StatusCode},
    Filter,
};

use types::AuthCheckRequest;
use web::{middleware, HandlerContext, Resp};

/// Returns the `/api` routes.
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
        .map(|auth, req: AuthCheckRequest| {
            //middleware::capabilities(ctx.db.clone(), req.token, req.capabilities)
            unimplemented!()
        })
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
