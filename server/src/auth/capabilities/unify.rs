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
            substs.push((l, r.clone()));
            Some(())
        }
        (_, &Term::Var(r)) => {
            substs.push((r, l.clone()));
            Some(())
        }
        (Term::Lit(l), Term::Lit(r)) => {
            if l.functor() == r.functor() {
                for (l, r) in l.1.iter().zip(r.1.iter()) {
                    unify_helper(l.clone(), r.clone(), substs)?;
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
        _ => None,
    }
}
