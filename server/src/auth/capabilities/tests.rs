use std::collections::HashSet;

use super::*;
use util::gensym;

#[test]
fn execution() {
    let rules = r#"
        taught(socrates, plato).
        taught(plato, aristotle).

        path(X, X).
        path(X, Z) :- taught(X, Y), path(Y, Z).
    "#;
    let rules = rules.parse::<Rules>().unwrap();

    let ans_var = gensym();
    let mut scope = vec![("X".to_string(), ans_var)].into_iter().collect();
    let query = "path(plato, X)"
        .parse::<cst::Lit>()
        .unwrap()
        .to_ast(&mut HashSet::new(), &mut scope);

    let env = Env::new_self_contained::<()>(&rules);
    let results = env
        .solve(query)
        .map(|s| {
            s.into_iter()
                .find(|&(k, _)| k == ans_var)
                .map(|(_, v)| v)
                .unwrap()
        })
        .collect()
        .wait();
    unimplemented!("{:?}", results)
}

#[test]
fn parse_terms() {
    assert_eq!(
        "foo".parse::<cst::Term>().unwrap(),
        cst::Term::Lit(cst::Lit("foo".to_string(), vec![]))
    );
    assert_eq!(
        "Bar".parse::<cst::Term>().unwrap(),
        cst::Term::Var("Bar".to_string())
    );
    assert_eq!("_".parse::<cst::Term>().unwrap(), cst::Term::Any);
    assert_eq!(
        "baz(1, 'Quux')".parse::<cst::Term>().unwrap(),
        cst::Term::Lit(cst::Lit(
            "baz".to_string(),
            vec![
                cst::Term::Num(1),
                cst::Term::Lit(cst::Lit("Quux".to_string(), vec![])),
            ]
        ))
    );
}
