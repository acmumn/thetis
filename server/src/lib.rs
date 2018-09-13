//! Thetis is the ACM UMN website and nerve center.

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate failure;
extern crate fall;
extern crate futures;
#[macro_use]
extern crate frunk;
extern crate jsonwebtoken;
#[macro_use]
extern crate lazy_static;
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
mod auth;
mod context;
mod db;
pub mod errors;
pub mod util;
pub mod web;

pub use auth::auth_check;
pub use context::Context;
pub use db::DB;
