use chumsky::Stream;
use comix::grammar::ebnf_parser;

#[test]
fn validate_ebnf_parser() {
    let ebnf_input = r#"
        program ::= statement* ;
        statement ::= "let" IDENTIFIER "=" expression ";" ;
        expression ::= term ("+" term)* ;
        term ::= atom | text | number ;
        atom ::= IDENTIFIER ;
        text ::= '"' .* '"' ;
        number ::= [0-9]+ ;
    "#;

    // Create a stream with positional data for parsing
    let stream = Stream::from_iter(
        ebnf_input.len()..ebnf_input.len() + 1,
        ebnf_input.chars().enumerate().map(|(i, c)| (c, i..i + 1)),
    );

    let grammar_parser = ebnf_parser();

    // Parse the EBNF grammar
    match grammar_parser.parse(stream) {
        Ok(grammar) => {
            println!("Parsed grammar: {:#?}", grammar);
            // Add assertions here to validate the parsed grammar
        }
        Err(errors) => {
            for error in errors {
                eprintln!("Parse error: {}", error);
            }
            panic!("EBNF parser validation failed");
        }
    }
}
