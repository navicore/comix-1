use chumsky::Parser;
use comix::grammar::ebnf_parser;

#[test]
fn validate_ebnf_parser() {
    let ebnf_input = r#"
        program ::= statement* ;
        statement ::= "let" IDENTIFIER "=" expression ";" ;
    "#;

    let grammar_parser = ebnf_parser();

    match grammar_parser.parse(ebnf_input) {
        Ok(grammar) => println!("Parsed grammar: {:#?}", grammar),
        Err(errors) => panic!("Failed to parse EBNF: {:?}", errors),
    }
}
