#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Rules(pub Vec<Clause>);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Clause(pub Lit, pub Vec<Lit>);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Name(pub String);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Term {
    Any,
    Lit(Lit),
    Num(isize),
    Var(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Lit(pub Name, pub Vec<Term>);
