mod api;

use warp::{self, filters::BoxedFilter, Filter, Reply};

use HandlerContext;

/// Returns the routes for the application.
pub fn routes(ctx: HandlerContext) -> BoxedFilter<(impl Reply,)> {
    let api = path!("api").and(api::routes(ctx));

    api.with(warp::log("thetis::web")).boxed()
}
