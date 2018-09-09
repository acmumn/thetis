use std::sync::Arc;

use auth::capabilities::ast::{Lit, Term};

/// A substitution.
pub type Subst = Vec<(usize, Arc<Term>)>;

/// Applies a substitution to a term.
pub fn apply_subst<'a, 'b: 'a>(mut term: &'a Term, subst: &'b Subst) -> Arc<Term> {
    loop {
        match *term {
            Term::Lit(ref l) => break Arc::new(Term::Lit(apply_subst_to_lit(l, subst))),
            Term::Num(n) => break Arc::new(Term::Num(n)),
            Term::Var(n) => {
                for &(k, ref v) in subst.iter() {
                    if k == n {
                        term = &*v;
                    }
                }
                break Arc::new(Term::Var(n));
            }
        }
    }
}

/// Applies a substitution to a lit.
pub fn apply_subst_to_lit(lit: &Lit, subst: &Subst) -> Lit {
    let Lit(ref n, ref a) = *lit;
    let a = a.iter().map(|t| apply_subst(&*t, subst)).collect();
    Lit(n.clone(), a)
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
