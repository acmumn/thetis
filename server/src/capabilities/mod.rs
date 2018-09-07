//! The capabilities system. This is an interpreter for a small subset of Prolog; if you find
//! yourself confused as to why this is the case (and possibly feel the urge to rewrite it), ask
//! Prof. Gini or Prof. Nadathur for Prolog/logic programming resources and convince yourself that
//! this approach makes sense.
//!
//! If you want to understand more about this implementation, see
//! [How to replace failure by a list of successes](http://dl.acm.org/citation.cfm?id=5280.5288)
//! by Philip Wadler. This code does not follow that paper exactly (since we can encounter database
//! errors during resolution, and we want to memoize as much as possible), but the general approach
//! is the same.
//!
//! NOTE(remexre): I'm unconvinced that this implementation won't have stack overflows with
//! sufficiently poorly written rules; the [stacker](https://crates.io/crates/stacker/) crate may
//! help with this, though exactly how is nontrivial. (Create a custom stream adaptor?)

use futures::{future::ok, prelude::*};

use {types::MemberID, DB};

/// Checks if a member currently has a set of capabilities.
pub fn check(db: DB, member: MemberID) -> impl Future<Item = bool, Error = ()> {
    ok(unimplemented!())
}
