mod api;

use warp::{self, filters::BoxedFilter, Filter, Reply};

use Context;

/// Returns the routes for the application.
pub fn routes(ctx: Context) -> BoxedFilter<(impl Reply,)> {
    let api = path!("api" / "thetis").and(api::routes(ctx));

    api.with(warp::log("thetis::web")).boxed()
}
