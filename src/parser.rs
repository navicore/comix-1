use chumsky::prelude::*;
use chumsky::Parser;

// Define the Abstract Syntax Tree (AST) for your custom language
#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    Text(String),
    Variable(String),
    Add(Box<Expr>, Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Statement {
    Let(String, Expr),
}

// The parser for the custom language
pub fn language_parser() -> impl Parser<char, Vec<Statement>, Error = Simple<char>> {
    // Parse identifiers (variable names)
    let ident = text::ident().padded();

    // Parse numbers
    let number = text::digits(10)
        .from_str()
        .unwrapped()
        .padded()
        .map(Expr::Number);

    // Parse quoted strings
    let text = just('"')
        .ignore_then(filter(|c| *c != '"').repeated().collect::<String>())
        .then_ignore(just('"'))
        .padded()
        .map(Expr::Text);

    // Parse variable references
    let variable = ident.map(Expr::Variable);

    // Parse terms (numbers, text, or variables)
    let term = number.or(text).or(variable);

    // Parse addition expressions
    let expression = term
        .then(just('+').padded().ignore_then(term).repeated())
        .foldl(|lhs, rhs| Expr::Add(Box::new(lhs), Box::new(rhs)));

    // Parse `let` statements
    let let_statement = just("let")
        .ignore_then(ident)
        .then_ignore(just('='))
        .then(expression)
        .then_ignore(just(';'))
        .map(|(name, expr)| Statement::Let(name, expr));

    // Parse multiple statements
    let_statement.repeated().padded()
}
