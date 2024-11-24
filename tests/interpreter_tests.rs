use chumsky::Parser;
use comix::interpreter::{evaluate_program, SymbolTable, Value};
use comix::parser::{language_parser, Statement};

#[test]
fn test_addition() {
    let program_input = r#"
        let x = 42;
        let y = x + 3;
    "#;

    let parser = language_parser();

    let parsed_program = parser
        .parse(program_input)
        .expect("Failed to parse program");
    println!("Parsed Program: {:#?}", parsed_program);

    let parsed_program = parser
        .parse(program_input)
        .expect("Failed to parse program");

    let mut symbols = SymbolTable::new();
    evaluate_program(parsed_program, &mut symbols).expect("Failed to evaluate program");

    assert_eq!(symbols.get("x"), Some(&Value::Number(42)));
    assert_eq!(symbols.get("y"), Some(&Value::Number(45)));
}
