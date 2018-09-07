//! The "publicish" functions. These are directly exposed as routes on `/api`, and making these
//! more generic should be preferred to reimplementing their functionality elsewhere. See
//! `doc/api.md` for details.

use futures::{future::ok, prelude::*};

use HandlerContext;

/// The `/api/auth/check` call.
pub fn auth_check<C>(
    ctx: &HandlerContext,
    token: &str,
    caps: &[C],
) -> impl Future<Item = (), Error = ()>
where
    C: AsRef<str>,
{
    ok(unimplemented!())
}

/// The `/api/auth/login` call.
pub fn auth_login() {
    unimplemented!()
}

/// The `/api/mail/enqueue` call.
pub fn mail_enqueue() {
    unimplemented!()
}
