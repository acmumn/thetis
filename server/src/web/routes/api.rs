//! API routes, which get mounted at `/api`.
//!
//! This should all be rewritten the moment `#[async]`/`await!()` becomes stable.

use fall::ResolutionError;
use frunk::Coproduct;
use futures::prelude::*;
use warp::{
    self,
    http::{Response, StatusCode},
    Filter,
};

use auth::{self, Claims};
use errors::{DatabaseError, NoSuchUser, WebError};
use types::{AuthCheckRequest, AuthError, AuthLoginRequest, MailEnqueueRequest};
use web::{middleware, Resp};
use Context;

/// Returns the routes that get mounted at `/api`.
pub fn routes(ctx: Context) -> Resp {
    let get_ping = path!("ping").and(get_ping(ctx.clone()));
    let post_auth_check = path!("auth" / "check").and(post_auth_check(ctx.clone()));
    let post_auth_login = path!("auth" / "login").and(post_auth_login(ctx.clone()));
    let post_mail_enqueue = path!("mail" / "enqueue").and(post_mail_enqueue(ctx));

    get_ping
        .or(post_auth_check)
        .unify()
        .or(post_mail_enqueue)
        .unify()
        .boxed()
}

/// The `POST /api/thetis/auth/check` route.
pub fn post_auth_check(ctx: Context) -> Resp {
    warp::index()
        .and(warp::post2())
        .and(warp::cookie::optional("auth"))
        .and(middleware::body())
        .and_then(move |auth, req: AuthCheckRequest| {
            middleware::capabilities(&ctx, auth, caps!["auth.check"])
                .and_then(|()| {
                    auth::check(&ctx, &req.token, req.capabilities).then(|r| match r {
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
                }).then(|r| match r {
                    Ok((status, body)) => middleware::simple_response(status, body),
                    Err(e) => Ok(e.to_response()),
                })
        }).boxed()
}

/// The `POST /api/thetis/auth/login` route.
pub fn post_auth_login(ctx: Context) -> Resp {
    warp::index()
        .and(warp::post2())
        .and(middleware::body())
        .and_then(
            move |req: AuthLoginRequest| -> impl Future<Error = ::warp::Rejection> {
                ctx.db
                    .get_memberid_from_x500(req.x500)
                    .then(|r| match r {
                        Ok(Some(id)) => Ok(id),
                        Ok(None) => Err(Coproduct::inject(NoSuchUser)),
                        Err(e) => Err(Coproduct::inject(e)),
                    }).and_then(|id| {
                        let claims = Claims::User { id };
                        claims
                            .check(&ctx, caps!["auth.login"])
                            .map(|()| id)
                            .map_err(|e| e.embed())
                    }).and_then(|id| {
                        ctx.db.add_escrow(id).map_err(Coproduct::inject).and_then(
                            |uuid| -> ::futures::future::FutureResult<
                                (),
                                Coprod!(AuthError, DatabaseError, NoSuchUser, ResolutionError),
                            > {
                                // TODO
                                ::futures::future::ok(())
                            },
                        )
                    }).then(
                        |r: Result<
                            (),
                            Coprod!(AuthError, DatabaseError, NoSuchUser, ResolutionError),
                        >| match r {
                            Ok(()) => Ok(Response::builder()
                                .status(StatusCode::NO_CONTENT)
                                .body(String::new())
                                .unwrap()),
                            Err(e) => Ok(e.to_response()),
                        },
                    )
            },
        ).boxed()
}

/// The `POST /api/thetis/mail/enqueue` route.
pub fn post_mail_enqueue(ctx: Context) -> Resp {
    warp::index()
        .and(warp::post2())
        .and(warp::cookie::optional("auth"))
        .and(middleware::body())
        .and_then(move |auth, req: MailEnqueueRequest| {
            middleware::capabilities(&ctx, auth, caps!["mail.send"])
                .and_then(|()| {
                    //auth::check(&ctx, &req.token, req.capabilities).then(|r| {
                    //let body = match r {
                    //Ok(()) => json!({ "type": "ok" }),
                    //Err(e) => match_coproduct!(e, {
                    //err : AuthError => { serde_json::to_value(err).unwrap() }
                    //}),
                    //};
                    //Ok((StatusCode::OK, body))
                    //})
                    Ok(unimplemented!())
                }).then(|r: Result<(_, ()), _>| match r {
                    Ok((status, body)) => middleware::simple_response(status, body),
                    Err(e) => Ok(e.to_response()),
                })
        }).boxed()
}

/// The `GET /api/thetis/ping` route.
pub fn get_ping(_ctx: Context) -> Resp {
    warp::index()
        .and(warp::get2())
        .map(|| {
            Response::builder()
                .status(StatusCode::NO_CONTENT)
                .body(String::new())
                .unwrap()
        }).boxed()
}
