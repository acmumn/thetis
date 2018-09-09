//! API routes, which get mounted at `/api`.

use futures::prelude::*;
use serde_json;
use warp::{
    self,
    http::{Response, StatusCode},
    Filter,
};

use errors::{DatabaseError, WebError};
use types::{AuthCheckRequest, MailEnqueueRequest};
use web::{middleware, Resp};
use {auth_check, Context};

/// Returns the routes that get mounted at `/api`.
pub fn routes(ctx: Context) -> Resp {
    let get_ping = path!("ping").and(get_ping(ctx.clone()));
    let post_auth_check = path!("auth" / "check").and(post_auth_check(ctx.clone()));
    let post_mail_enqueue = path!("mail" / "enqueue").and(post_mail_enqueue(ctx));

    get_ping
        .or(post_auth_check)
        .unify()
        .or(post_mail_enqueue)
        .unify()
        .boxed()
}

/// The `POST /api/auth/check` route.
pub fn post_auth_check(ctx: Context) -> Resp {
    warp::index()
        .and(warp::post2())
        .and(warp::cookie::optional("auth"))
        .and(middleware::body())
        .and_then(move |auth, req: AuthCheckRequest| {
            middleware::capabilities(&ctx, auth, caps!["auth.check"])
                .and_then(|()| {
                    auth_check(&ctx, &req.token, req.capabilities).then(|r| match r {
                        Ok(()) => Ok((StatusCode::OK, json!({ "type": "ok" }))),
                        Err(e) => {
                            let (status, body) = e.to_status_body();
                            let status = if status == StatusCode::FORBIDDEN {
                                StatusCode::OK
                            } else {
                                status
                            };
                            Ok((status, body))
                        }
                    })
                })
                .then(|r| match r {
                    Ok((status, body)) => middleware::simple_response(status, body),
                    Err(e) => Ok(e.to_response()),
                })
        })
        .boxed()
}

/// The `POST /api/mail/enqueue` route.
pub fn post_mail_enqueue(ctx: Context) -> Resp {
    warp::index()
        .and(warp::post2())
        .and(warp::cookie::optional("auth"))
        .and(middleware::body())
        .and_then(move |auth, req: MailEnqueueRequest| {
            middleware::capabilities(&ctx, auth, caps!["mail.send"])
                .and_then(|()| {
                    //auth_check(&ctx, &req.token, req.capabilities).then(|r| {
                    //let body = match r {
                    //Ok(()) => json!({ "type": "ok" }),
                    //Err(e) => match_coproduct!(e, {
                    //err : AuthError => { serde_json::to_value(err).unwrap() }
                    //}),
                    //};
                    //Ok((StatusCode::OK, body))
                    //})
                    Ok(unimplemented!())
                })
                .then(|r: Result<(_, ()), _>| match r {
                    Ok((status, body)) => middleware::simple_response(status, body),
                    Err(e) => Ok(e.to_response()),
                })
        })
        .boxed()
}

/// The `GET /api/ping` route.
pub fn get_ping(_ctx: Context) -> Resp {
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
