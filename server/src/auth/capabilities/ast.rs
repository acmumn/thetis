use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;

use auth::capabilities::cst;
use errors::CapabilitiesLoadError;
use util::gensym;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Rules(pub Vec<Clause>);

impl Rules {
    /// Loads from a file.
    pub fn load_from(path: impl AsRef<Path>) -> Result<Rules, CapabilitiesLoadError> {
        let src = {
            let mut f = File::open(path)?;
            let mut buf = String::new();
            f.read_to_string(&mut buf)?;
            buf
        };
        src.parse().map_err(CapabilitiesLoadError::from)
    }
}

impl FromStr for Rules {
    type Err = <cst::Rules as FromStr>::Err;
    fn from_str(src: &str) -> Result<Rules, Self::Err> {
        let cst = src.parse::<cst::Rules>()?;
        let mut atoms = HashSet::new();
        Ok(cst.to_ast(&mut atoms))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Clause(pub Lit, pub Vec<Lit>);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Term {
    Lit(Lit),
    Num(u32),
    Var(usize),
}

impl Term {
    /// Returns a unique `Var`.
    pub fn gensym() -> Arc<Term> {
        Arc::new(Term::Var(gensym()))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Lit(pub Arc<str>, pub Vec<Arc<Term>>);

impl Lit {
    /// Returns the name and arity of the literal.
    pub fn functor(&self) -> (Arc<str>, usize) {
        (self.0.clone(), self.1.len())
    }

    /// Returns the name and arity of the literal.
    pub fn functor_b(&self) -> (&str, usize) {
        (&*self.0, self.1.len())
    }
}
