use std::sync::Arc;

use url::Url;

use DB;

/// The values shared by request handlers. Cheaply clonable.
#[derive(Clone)]
pub struct HandlerContext {
    /// The base URL of the site.
    pub base_url: Arc<Url>,

    /// A pool of connections to the database.
    pub db: DB,

    /// The secret to use for signing JWTs.
    pub jwt_secret: Arc<str>,
}
