//! Common types between the thetis client and server.

extern crate serde;
#[macro_use]
extern crate serde_derive;

mod auth;
mod newtypes;

pub use auth::*;
pub use newtypes::*;
