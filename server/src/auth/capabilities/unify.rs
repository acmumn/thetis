use std::sync::Arc;

use auth::capabilities::ast::{Lit, Term};

/// A substitution.
type Subst = Vec<(usize, Arc<Term>)>;

/// Applies a substitution to a term.
pub fn apply_subst(term: Arc<Term>, subst: &Subst) -> Arc<Term> {
    unimplemented!()
}

/// Unifies two terms, returning a list of substitutions from variable IDs to terms.
pub fn unify(l: Arc<Term>, r: Arc<Term>) -> Option<Subst> {
    let mut substs = Vec::new();
    unify_helper(l, r, &mut substs)?;
    Some(substs)
}

fn unify_helper(l: Arc<Term>, r: Arc<Term>, substs: &mut Subst) -> Option<()> {
    match (&*l, &*r) {
        (&Term::Var(l), _) => {
            substs.push((l, r));
            Some(())
        }
        (_, &Term::Var(r)) => {
            substs.push((r, l));
            Some(())
        }
        (&Term::Lit(l @ Lit(_, la)), &Term::Lit(r @ Lit(_, ra))) => {
            if l.functor() == r.functor() {
                for (l, r) in la.iter().zip(ra.iter()) {
                    unify_helper(l, r, substs)?;
                }
                Some(())
            } else {
                None
            }
        }
        (&Term::Num(l), &Term::Num(r)) => if l == r {
            Some(())
        } else {
            None
        },
    }
}
