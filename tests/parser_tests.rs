use chumsky::Parser;
use comix::parser::{language_parser, Expr, Statement};

#[test]
fn parse_multiple_statements() {
    let program_input = r#"
        let x = 42;
        let y = x + 3;
    "#;

    let parser = language_parser();
    let parsed_program = parser
        .parse(program_input)
        .expect("Failed to parse program");

    assert_eq!(
        parsed_program,
        vec![
            Statement::Let("x".to_string(), Expr::Number(42)),
            Statement::Let(
                "y".to_string(),
                Expr::Add(
                    Box::new(Expr::Variable("x".to_string())),
                    Box::new(Expr::Number(3))
                )
            )
        ]
    );
}
