use std::sync::Arc;

use util::gensym;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Rules(pub Vec<Clause>);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Clause(pub Lit, pub Vec<Lit>);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Name(pub Arc<str>);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Term {
    Lit(Lit),
    Num(isize),
    Var(usize),
}

impl Term {
    /// Returns a unique `Var`.
    pub fn gensym() -> Arc<Term> {
        Arc::new(Term::Var(gensym()))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Lit(pub Name, pub Vec<Arc<Term>>);

impl Lit {
    /// Returns the name and arity of the literal.
    pub fn functor(&self) -> (&str, usize) {
        (&*self.0, self.1.len())
    }
}
