//! The capabilities system. This is an interpreter for a small subset of Prolog; if you find
//! yourself confused as to why this is the case (and possibly feel the urge to rewrite it), ask
//! Prof. Gini or Prof. Nadathur for Prolog/logic programming resources and convince yourself that
//! this approach makes sense.
//!
//! NOTE(remexre): I'm unconvinced that this implementation won't have stack overflows with
//! sufficiently poorly written rules; the [stacker](https://crates.io/crates/stacker/) crate may
//! help with this, though exactly how is nontrivial. (Create a custom stream adaptor?)

mod ast;
mod cst;
mod unify;

lalrpop_mod!(grammar);

use futures::{future::ok, prelude::*};

use {types::MemberID, HandlerContext};

/// Checks if a member currently has a set of capabilities.
pub fn check<C: AsRef<str>>(
    ctx: &HandlerContext,
    member: MemberID,
    caps: Vec<C>,
) -> impl Future<Item = bool, Error = ()> {
    ok(unimplemented!())
}
