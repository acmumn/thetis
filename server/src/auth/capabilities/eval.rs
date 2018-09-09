use std::collections::HashMap;
use std::sync::Arc;

use futures::{
    prelude::*,
    stream::{empty, iter_ok, once, Empty},
};

use auth::capabilities::{
    unify::{apply_subst_to_lit, unify, Subst},
    Clause, Lit, Rules, Term,
};
use util::box_stream;

/// An execution environment.
pub struct Env<F> {
    // rules is indexed by functor to make lookup faster.
    rules: Arc<HashMap<(Arc<str>, usize), Vec<Clause>>>,

    // An external function for resolving predicates.
    external: Arc<F>,
}

// The unit type argument is just a placeholder.
impl Env<()> {
    /// Creates an env without an external resolver.
    pub fn new_self_contained<E: 'static + Send>(
        rules: &Rules,
    ) -> Env<for<'a> fn(&'a Lit) -> Empty<Subst, E>> {
        Env::new(rules, |_| empty())
    }
}

impl<E, F, S> Env<F>
where
    E: 'static + Send,
    F: 'static + for<'a> Fn(&'a Lit) -> S + Send + Sync,
    S: 'static + Stream<Item = Subst, Error = E> + Send,
{
    /// Create an Env.
    pub fn new(rules: &Rules, external: F) -> Env<F> {
        let mut internal = HashMap::<_, Vec<_>>::new();
        for rule in &rules.0 {
            internal
                .entry(rule.0.functor())
                .or_default()
                .push(rule.clone());
        }

        Env {
            rules: Arc::new(internal),
            external: Arc::new(external),
        }
    }

    /// Tries to solve for the given literal.
    pub fn solve(&self, lit: Lit) -> impl Stream<Item = Subst, Error = E> + Send {
        eprintln!("solve {:?}", lit);
        (self.external)(&lit).chain(self.solve_internal(lit))
    }

    /// Tries to solve for multiple literals.
    pub fn solve_all(&self, mut lits: Vec<Lit>) -> impl Stream<Item = Subst, Error = E> + Send {
        if lits.is_empty() {
            box_stream(once(Ok(Subst::new())))
        } else {
            let hd = lits.remove(0);
            let env = self.clone(); // TODO: There's gotta be a more elegant approach...
            box_stream(
                self.solve(hd)
                    .map(move |s| {
                        let tl = lits.iter().map(|c| apply_subst_to_lit(c, &s)).collect();
                        env.solve_all(tl).map(move |s2| {
                            let mut s = s.clone();
                            s.extend(s2);
                            s
                        })
                    })
                    .flatten(),
            )
        }
    }

    fn solve_internal(&self, lit: Lit) -> impl Stream<Item = Subst, Error = E> + Send {
        let rules = self.rules.get(&lit.functor()).cloned().unwrap_or_default();
        let term = Arc::new(Term::Lit(lit));
        let env = self.clone(); // TODO: There's gotta be a more elegant approach...
        iter_ok(rules)
            .filter_map(move |Clause(h, b)| {
                let h = Arc::new(Term::Lit(h.clone()));
                unify(term.clone(), h).map(|s| {
                    let b = b.iter().map(|c| apply_subst_to_lit(c, &s)).collect();
                    (s, b)
                })
            })
            .map(move |(s, b)| {
                env.solve_all(b).map(move |s2| {
                    let mut s = s.clone();
                    s.extend(s2);
                    s
                })
            })
            .flatten()
    }
}

impl<F> Clone for Env<F> {
    fn clone(&self) -> Env<F> {
        Env {
            rules: self.rules.clone(),
            external: self.external.clone(),
        }
    }
}
