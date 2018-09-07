//! Thetis is the ACM UMN website and nerve center.

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate failure;
extern crate futures;
extern crate lettre;
extern crate lettre_email;
#[macro_use]
extern crate log;
extern crate mime;
extern crate pulldown_cmark;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate tera;
extern crate tokio;
extern crate tokio_threadpool;
extern crate url;
#[macro_use]
extern crate warp;

pub extern crate thetis_common as types;

#[macro_use]
mod macros;

pub mod api;
mod capabilities;
mod db;
pub mod util;
pub mod web;

use std::sync::Arc;

use url::Url;

pub use db::DB;

/// The values shared by request handlers. Cheaply clonable.
#[derive(Clone)]
pub struct HandlerContext {
    /// The base URL of the site.
    pub base_url: Arc<Url>,

    /// A pool of connections to the database.
    pub db: DB,
}
