//! The capabilities system. This is an interpreter for a small subset of Prolog; if you find
//! yourself confused as to why this is the case (and possibly feel the urge to rewrite it), ask
//! Prof. Gini or Prof. Nadathur for Prolog/logic programming resources and convince yourself that
//! this approach makes sense.
//!
//! NOTE(remexre): I'm unconvinced that this implementation won't have stack overflows with
//! sufficiently poorly written rules; the [stacker](https://crates.io/crates/stacker/) crate may
//! help with this, though exactly how is nontrivial. (Create a custom stream adaptor?)

mod ast;
pub(crate) mod cst;
mod unify;

#[cfg(test)]
mod tests;

pub(crate) mod grammar {
    lalrpop_mod!(grammar);
    pub use self::grammar::*;
}

use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use futures::{future::ok, prelude::*};

pub use auth::capabilities::ast::*;
use {errors::CapabilitiesLoadError, types::MemberID, Context};

/// Checks if a member currently has a set of capabilities.
pub fn check<C: AsRef<str>>(
    ctx: &Context,
    member: MemberID,
    caps: Vec<C>,
) -> impl Future<Item = bool, Error = ()> {
    ok(unimplemented!())
}

/// Loads a capabilities AST from a file.
pub fn load_capabilities_from(path: impl AsRef<Path>) -> Result<Rules, CapabilitiesLoadError> {
    let src = {
        let mut f = File::open(path)?;
        let mut buf = String::new();
        f.read_to_string(&mut buf)?;
        buf
    };

    let cst = src.parse::<cst::Rules>()?;

    let mut atoms = HashSet::new();
    Ok(cst.to_ast(&mut atoms))
}
