//! Error types.

use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::Error as IoError;

use diesel::{r2d2::PoolError, result::Error as DieselError};
use frunk::coproduct::{CNil, Coproduct};
use serde_json::{self, Value};
use warp::http::{Response, StatusCode};

pub use auth::capabilities::cst::ParseError;
use types::AuthError;

impl_WebError_for_Serialize!(AuthError, StatusCode::FORBIDDEN);

/// An error evaluating capabilities.
#[derive(Debug, Fail)]
pub enum CapsEvalError {
    /// A variable was passed where a literal was required. Reordering goals may fix this.
    #[fail(display = "Insufficiently instantiated arguments to {}/{}", _0, _1)]
    InsufficientlyInstantiatedArgs(&'static str, usize),

    /// A value of the wrong type was passed.
    #[fail(display = "Type error in arguments to {}/{}", _0, _1)]
    TypeError(&'static str, usize),
}

impl WebError for CapsEvalError {
    fn to_status_body(self) -> (StatusCode, Value) {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({ "type": "capabilities", "msg": self.to_string() }),
        )
    }
}

define_error!(CapabilitiesLoadError {
    Io(IoError, err => err),
    Parse(ParseError, err => err.clone().map_token(|(n, t)| t)),
});

define_error!(DatabaseError {
    Diesel(DieselError, err => err),
    R2D2(PoolError, err => err),
});

impl WebError for DatabaseError {
    fn to_status_body(self) -> (StatusCode, Value) {
        let msg = match self {
            DatabaseError::Diesel(err) => err.to_string(),
            DatabaseError::R2D2(err) => err.to_string(),
        };
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({ "type": "database", "msg": msg }),
        )
    }
}

/// A trait for returning errors to a client more easily.
pub trait WebError: Sized {
    fn to_status_body(self) -> (StatusCode, Value);

    fn to_response(self) -> Response<String> {
        let (status, body) = self.to_status_body();
        Response::builder()
            .status(status)
            .body(body.to_string())
            .unwrap()
    }
}

impl WebError for CNil {
    fn to_status_body(self) -> (StatusCode, Value) {
        match self {}
    }
}

impl<H: WebError, T: WebError> WebError for Coproduct<H, T> {
    fn to_status_body(self) -> (StatusCode, Value) {
        match self {
            Coproduct::Inl(h) => h.to_status_body(),
            Coproduct::Inr(t) => t.to_status_body(),
        }
    }
}
