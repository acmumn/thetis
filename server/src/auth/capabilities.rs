//! The capabilities system. This uses [fall](https://crates.io/crates/fall), an interpreter for a
//! small subset of Prolog. If you find yourself confused as to why this is the case (and possibly
//! feel the urge to rewrite it), ask Prof. Gini or Prof. Nadathur for Prolog/logic programming
//! resources and convince yourself that this approach makes sense.

use std::sync::Arc;

use frunk::Coproduct;
use futures::{
    prelude::*,
    stream::{iter_ok, once, poll_fn},
};

use errors::DatabaseError;
use fall::{Env, Lit, ResolutionError, Subst, Term};
use types::{MemberID, Tag};
use util::box_stream;
use Context;

/// Checks if a member currently has a set of capabilities.
pub fn check<C: AsRef<str>>(
    ctx: Context,
    member: MemberID,
    caps: Vec<C>,
) -> impl Future<Item = bool, Error = Coprod!(ResolutionError, DatabaseError)> + Send {
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
    env.solve_all(lits, 10).into_future().then(|r| match r {
        Ok((o, _)) => Ok(o.is_some()),
        Err((e, _)) => Err(e),
    })
}

/// The external resolver we're actaully using.
fn ext_resolver(
    ctx: &Context,
    lit: &Lit,
) -> Box<Stream<Item = Subst, Error = Coprod!(ResolutionError, DatabaseError)> + Send> {
    // TODO: This could use some macro magic to make it much more readable...
    match lit.functor_b() {
        ("debug", _) => {
            info!("{}", lit);
            box_stream(once(Ok(Subst::new())))
        }
        ("notBanned", 1) => match *lit.1[0] {
            Term::Num(n) => box_stream(
                success_adaptor(ctx.db.is_banned(MemberID(n)).map(|b| !b))
                    .map_err(Coproduct::inject),
            ),
            ref term => {
                let kind = match term {
                    Term::Var(_) => ResolutionError::InsufficientlyInstantiatedArgs,
                    _ => ResolutionError::TypeError,
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
                    Term::Var(_) => ResolutionError::InsufficientlyInstantiatedArgs,
                    _ => ResolutionError::TypeError,
                };
                box_stream(once(Err(Coproduct::inject(kind("notBanned", 1)))))
            }
        },
        ("role", 2) => match (&*lit.1[0], &*lit.1[1]) {
            (&Term::Num(n), &Term::Lit(Lit(ref t, ref a))) if a.is_empty() => box_stream(
                success_adaptor(ctx.db.has_tag(MemberID(n), Tag(t.to_string())))
                    .map_err(Coproduct::inject),
            ),
            (&Term::Num(n), &Term::Var(v)) => box_stream(
                ctx.db
                    .get_tags(MemberID(n))
                    .map(move |tags| {
                        iter_ok(tags).map(move |tag| {
                            let tag = Lit(tag.0.into(), vec![]);
                            let tag = Arc::new(Term::Lit(tag));

                            let mut s = Subst::new();
                            s.push(v, tag);
                            s
                        })
                    })
                    .flatten_stream()
                    .map_err(Coproduct::inject),
            ),
            (term, _) => {
                let kind = match term {
                    &Term::Var(_) => ResolutionError::InsufficientlyInstantiatedArgs,
                    _ => ResolutionError::TypeError,
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
