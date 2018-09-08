use super::cst;

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
