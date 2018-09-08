//! Common types between the thetis client and server.

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod auth;
mod errors;
mod mail;
mod newtypes;

pub use auth::*;
pub use errors::*;
pub use mail::*;
pub use newtypes::*;
