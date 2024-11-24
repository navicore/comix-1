use chumsky::Parser;
use comix::interpreter::{evaluate_program, SymbolTable};
use comix::parser::language_parser;

fn main() {
    let program_input = r#"
        let x = 42;
        let y = x + 3;
    "#;

    let parser = language_parser();
    let parsed_program = match parser.parse(program_input) {
        Ok(program) => program,
        Err(errors) => {
            eprintln!("Parse errors:");
            for error in errors {
                eprintln!("{}", error);
            }
            return;
        }
    };

    let mut symbols = SymbolTable::new();
    match evaluate_program(parsed_program, &mut symbols) {
        Ok(_) => {
            println!("Program executed successfully!");
            println!("Symbol Table: {:#?}", symbols);
        }
        Err(err) => {
            eprintln!("Execution error: {}", err);
        }
    }
}
