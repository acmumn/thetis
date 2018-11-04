//! Error types.

use std::fmt::{Display, Formatter, Result as FmtResult};

use diesel::{r2d2::PoolError, result::Error as DieselError};
use fall::ResolutionError;
use frunk::coproduct::{CNil, Coproduct};
use serde_json::{self, Value};
use warp::http::{Response, StatusCode};

use types::AuthError;

impl_trivial_WebError!(NoSuchUser, StatusCode::NOT_FOUND, "no_such_user");
impl_WebError_for_Serialize!(AuthError, StatusCode::FORBIDDEN);

impl WebError for ResolutionError {
    fn to_status_body(self) -> (StatusCode, Value) {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({ "type": "resolution", "msg": self.to_string() }),
        )
    }
}

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
