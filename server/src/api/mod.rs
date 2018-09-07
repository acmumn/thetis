//! The "publicish" functions. These are directly exposed as routes on `/api`, and making these
//! more generic should be preferred to reimplementing their functionality elsewhere. See
//! `doc/api.md` for details.

/// The `/api/auth/check` route.
pub fn auth_check() {
    unimplemented!()
}

/// The `/api/auth/login` route.
pub fn auth_login() {
    unimplemented!()
}

/// The `/api/mail/enqueue` route.
pub fn mail_enqueue() {
    unimplemented!()
}
