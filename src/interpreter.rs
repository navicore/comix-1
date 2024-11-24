use crate::parser::{Expr, Statement};
use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Value {
    Number(i64),
    Text(String),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Number(num) => write!(f, "{}", num),
            Value::Text(text) => write!(f, "\"{}\"", text),
        }
    }
}

#[derive(Debug)]
pub struct SymbolTable {
    variables: HashMap<String, Value>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            variables: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: &str, value: Value) -> Result<(), String> {
        if self.variables.contains_key(name) {
            Err(format!("Variable '{}' is already defined", name))
        } else {
            self.variables.insert(name.to_string(), value);
            Ok(())
        }
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        self.variables.get(name)
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        SymbolTable::new()
    }
}

pub fn evaluate_program(
    statements: Vec<Statement>,
    symbols: &mut SymbolTable,
) -> Result<(), String> {
    for statement in statements {
        match statement {
            Statement::Let(name, expr) => {
                let value = evaluate_expression(expr, symbols)?;
                symbols.define(&name, value)?;
            }
        }
    }
    Ok(())
}

pub fn evaluate_expression(expr: Expr, symbols: &SymbolTable) -> Result<Value, String> {
    match expr {
        Expr::Number(num) => {
            println!("Evaluating Number: {}", num);
            Ok(Value::Number(num))
        }
        Expr::Variable(name) => {
            println!("Resolving Variable: {}", name);
            symbols
                .get(&name)
                .cloned()
                .ok_or_else(|| format!("Undefined variable '{}'", name))
        }
        Expr::Add(lhs, rhs) => {
            println!("Evaluating Add: {:#?} + {:#?}", lhs, rhs);
            let lhs = evaluate_expression(*lhs, symbols)?;
            let rhs = evaluate_expression(*rhs, symbols)?;
            match (lhs, rhs) {
                (Value::Number(l), Value::Number(r)) => {
                    println!("Add Result: {} + {} = {}", l, r, l + r);
                    Ok(Value::Number(l + r))
                }
                _ => Err("Addition is only supported for numbers".to_string()),
            }
        }
        _ => unimplemented!(),
    }
}
