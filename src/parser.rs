use chumsky::prelude::*;
use chumsky::Parser;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Expr {
    Number(i64),
    Text(String),
    Variable(String),
    Add(Box<Expr>, Box<Expr>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Statement {
    Let(String, Expr),
}

pub fn language_parser() -> impl Parser<char, Vec<Statement>, Error = Simple<char>> {
    // Parse identifiers (variable names)
    let ident = text::ident().padded();

    // Parse numbers
    let number = text::int(10)
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
    let variable = ident.clone().map(Expr::Variable);

    // Parse terms (numbers, text, or variables)
    let term = number.or(text).or(variable);

    // Parse addition expressions
    let expression = term
        .clone()
        .then(just('+').padded().ignore_then(term).repeated())
        .foldl(|lhs, rhs| Expr::Add(Box::new(lhs), Box::new(rhs)));

    // Parse `let` statements
    let let_statement = just("let")
        .ignore_then(ident)
        .then_ignore(just('='))
        .then(expression)
        .then_ignore(just(';').padded())
        .map(|(name, expr)| Statement::Let(name, expr));

    // Ensure separation of multiple statements
    let_statement
        .padded()
        .separated_by(just('\n').or(just(';')).repeated())
        .allow_trailing() // Allow trailing semicolons or newlines
        .padded()
}
