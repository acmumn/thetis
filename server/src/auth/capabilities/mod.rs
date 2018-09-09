//! The capabilities system. This is an interpreter for a small subset of Prolog; if you find
//! yourself confused as to why this is the case (and possibly feel the urge to rewrite it), ask
//! Prof. Gini or Prof. Nadathur for Prolog/logic programming resources and convince yourself that
//! this approach makes sense.
//!
//! If you want to understand more about this implementation, see
//! [How to replace failure by a list of successes](http://dl.acm.org/citation.cfm?id=5280.5288)
//! by Philip Wadler. This code does not follow that paper exactly (since we can encounter database
//! errors during resolution), but the general approach is the same.
//!
//! NOTE(remexre): I'm unconvinced that this implementation won't have stack overflows with
//! sufficiently poorly written rules; the [stacker](https://crates.io/crates/stacker/) crate may
//! help with this, though exactly how is nontrivial. (Create a custom stream adaptor?) More
//! importantly, don't write crappy rules.

mod ast;
pub(crate) mod cst;
mod eval;
mod unify;

#[cfg(test)]
mod tests;

pub(crate) mod grammar {
    lalrpop_mod!(grammar);
    pub use self::grammar::*;
}

use std::sync::Arc;

use frunk::Coproduct;
use futures::{
    prelude::*,
    stream::{once, poll_fn},
};

pub use auth::capabilities::{ast::*, eval::Env, unify::Subst};
use errors::{CapsEvalError, DatabaseError};
use types::{MemberID, Tag};
use util::box_stream;
use Context;

/// Checks if a member currently has a set of capabilities.
pub fn check<C: AsRef<str>>(
    ctx: Context,
    member: MemberID,
    caps: Vec<C>,
) -> impl Future<Item = bool, Error = Coprod!(CapsEvalError, DatabaseError)> + Send {
    lazy_static! {
        static ref CAP: Arc<str> = Arc::from("cap".to_string());
    }

    let env = {
        let ctx2 = ctx.clone();
        let caps = ctx2.capabilities.read().unwrap();
        Env::new(&caps, move |lit| ext_resolver(&ctx, lit))
    };

    let lits = caps
        .into_iter()
        .map(|cap| {
            let cap = Arc::<str>::from(cap.as_ref().to_string());
            let args = vec![
                Arc::new(Term::Num(member.0)),
                Arc::new(Term::Lit(Lit(cap, vec![]))),
            ];
            Lit(CAP.clone(), args)
        })
        .collect();
    env.solve_all(lits).into_future().then(|r| match r {
        Ok((o, _)) => Ok(o.is_some()),
        Err((e, _)) => Err(e),
    })
}

/// The external resolver we're actaully using.
fn ext_resolver(
    ctx: &Context,
    lit: &Lit,
) -> Box<dyn Stream<Item = Subst, Error = Coprod!(CapsEvalError, DatabaseError)> + Send> {
    // TODO: This could use some macro magic to make it much more readable...
    match lit.functor_b() {
        ("notBanned", 1) => match *lit.1[0] {
            Term::Num(n) => box_stream(
                success_adaptor(ctx.db.is_banned(MemberID(n))).map_err(Coproduct::inject),
            ),
            ref term => {
                let kind = match term {
                    Term::Var(_) => CapsEvalError::InsufficientlyInstantiatedArgs,
                    _ => CapsEvalError::TypeError,
                };
                box_stream(once(Err(Coproduct::inject(kind("notBanned", 1)))))
            }
        },
        ("paid", 1) => match *lit.1[0] {
            Term::Num(n) => {
                box_stream(success_adaptor(ctx.db.is_paid(MemberID(n))).map_err(Coproduct::inject))
            }
            ref term => {
                let kind = match term {
                    Term::Var(_) => CapsEvalError::InsufficientlyInstantiatedArgs,
                    _ => CapsEvalError::TypeError,
                };
                box_stream(once(Err(Coproduct::inject(kind("notBanned", 1)))))
            }
        },
        ("role", 2) => match (&*lit.1[0], &*lit.1[1]) {
            (&Term::Num(n), &Term::Lit(Lit(ref t, ref a))) if a.is_empty() => box_stream(
                success_adaptor(ctx.db.has_tag(MemberID(n), Tag(t.to_string())))
                    .map_err(Coproduct::inject),
            ),
            (&Term::Num(n), &Term::Var(v)) => {
                box_stream(/* TODO */ ::futures::stream::empty())
            }
            (term, _) => {
                let kind = match term {
                    &Term::Var(_) => CapsEvalError::InsufficientlyInstantiatedArgs,
                    _ => CapsEvalError::TypeError,
                };
                box_stream(once(Err(Coproduct::inject(kind("notBanned", 1)))))
            }
        },
        _ => box_stream(::futures::stream::empty()),
    }
}

fn success_adaptor<E, F>(mut fut: F) -> impl Stream<Item = Subst, Error = E>
where
    F: Future<Item = bool, Error = E> + Send + 'static,
{
    let mut done = false;
    box_stream(poll_fn(move || {
        if done {
            Ok(Async::Ready(None))
        } else {
            match fut.poll() {
                Ok(Async::Ready(b)) => {
                    done = true;
                    Ok(Async::Ready(if b { Some(Subst::new()) } else { None }))
                }
                Ok(Async::NotReady) => Ok(Async::NotReady),
                Err(e) => Err(e),
            }
        }
    }))
}
