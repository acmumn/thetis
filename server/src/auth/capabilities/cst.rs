use std::str::FromStr;

use lalrpop_util::ParseError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Rules(pub Vec<Clause>);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Clause(pub Lit, pub Vec<Lit>);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Term {
    Any,
    Lit(Lit),
    Num(isize),
    Var(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Lit(pub String, pub Vec<Term>);

macro_rules! generate_fromstr {
    ($name:ident $parser:ident) => {
        impl FromStr for $name {
            type Err = ParseError<usize, (usize, String), &'static str>;

            fn from_str(s: &str) -> Result<$name, Self::Err> {
                use auth::capabilities::grammar::Token;
                ::auth::capabilities::grammar::$parser::new()
                    .parse(s).map_err(|e| e.map_token(|Token(n, s)| (n, s.to_string())))
            }
        }
    };
    ($(($name:ident, $parser:ident)),*) => { $(generate_fromstr!($name $parser);)* };
}

generate_fromstr![
    (Clause, ClauseParser),
    (Lit, LitParser),
    (Rules, RulesParser),
    (Term, TermParser)
];
