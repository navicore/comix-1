use chumsky::prelude::*;
use chumsky::Parser;

// Structured representation of the grammar
#[derive(Debug)]
pub struct Grammar {
    pub rules: Vec<Rule>,
}

#[derive(Debug)]
pub struct Rule {
    pub name: String,
    pub productions: Vec<Production>,
}

#[derive(Debug)]
pub enum Production {
    Sequence(Vec<String>),
    Choice(Vec<String>),
    Repetition(String),
    Optional(String),
    Terminal(String),
    NonTerminal(String),
}

// Parser for EBNF
pub fn ebnf_parser() -> impl Parser<char, Grammar, Error = Simple<char>> {
    // Parse rule names (non-terminals)
    let ident = text::ident();

    // Parse terminal strings (e.g., "let")
    let terminal = just('"')
        .ignore_then(filter(|c| *c != '"').repeated().collect::<String>())
        .then_ignore(just('"'))
        .map(Production::Terminal);

    // Parse non-terminal references (e.g., IDENTIFIER)
    let non_terminal = ident.map(Production::NonTerminal);

    // Parse sequences (ordered lists of terminals/non-terminals)
    let sequence = non_terminal.or(terminal).repeated().at_least(1).map(|seq| {
        Production::Sequence(
            seq.into_iter()
                .map(|prod| match prod {
                    Production::NonTerminal(name) => name,
                    Production::Terminal(value) => value,
                    _ => unreachable!(),
                })
                .collect(),
        )
    });

    // Parse choices (alternatives separated by "|")
    let choice = sequence.separated_by(just('|')).map(|choices| {
        let flattened: Vec<String> = choices
            .into_iter()
            .flat_map(|prod| match prod {
                Production::Sequence(seq) => seq,
                _ => unreachable!(),
            })
            .collect();
        Production::Choice(flattened)
    });

    // Combine sequence and choice parsers
    let production = choice.or(sequence);

    // Parse a single rule
    let rule = ident
        .then_ignore(just("::="))
        .then(production)
        .map(|(name, prod)| Rule {
            name,
            productions: vec![prod],
        });

    // Parse the entire grammar (list of rules)
    rule.repeated().map(|rules| Grammar { rules })
}
