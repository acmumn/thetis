//! The HTTP-serving parts of the application.

mod middleware;
mod routes;

use std::net::SocketAddr;

use futures::{
    future::{loop_fn, ok, Loop},
    prelude::*,
};
use warp::{self, filters::BoxedFilter, http::Response};

pub use web::routes::routes;
use {HandlerContext, DB};

/// A convenient alias.
type Resp = BoxedFilter<(Response<String>,)>;

/// Starts a server at the given address. The polymorphism in the return type indicates that the
/// future will never resolve, since it can be trivially used as
/// `impl Future<Item = Void, Error = Void>`.
pub fn serve_on<T, E>(addr: SocketAddr, ctx: HandlerContext) -> impl Future<Item = T, Error = E> {
    loop_fn((), move |()| {
        warp::serve(routes(ctx.clone())).bind(addr).then(|r| {
            let status = match r {
                Ok(()) => "success",
                Err(()) => "failure",
            };
            warn!("HTTP server exited with {}; restarting...", status);
            ok(Loop::Continue(()))
        })
    })
}
