use crate::parser::{Expr, Statement};
use std::collections::HashMap;

// Represents the values in the symbol table
#[derive(Debug, Clone)]
pub enum Value {
    Number(i64),
    Text(String),
}

// The symbol table for variable bindings
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

    // Add a new variable to the symbol table
    pub fn define(&mut self, name: &str, value: Value) -> Result<(), String> {
        if self.variables.contains_key(name) {
            Err(format!("Variable '{}' is already defined", name))
        } else {
            self.variables.insert(name.to_string(), value);
            Ok(())
        }
    }

    // Retrieve a variable's value
    pub fn get(&self, name: &str) -> Option<&Value> {
        self.variables.get(name)
    }
}

// Evaluates a program (list of statements)
pub fn evaluate_program(
    statements: Vec<Statement>,
    symbols: &mut SymbolTable,
) -> Result<(), String> {
    for statement in statements {
        match statement {
            Statement::Let(name, expr) => {
                // Evaluate the expression
                let value = evaluate_expression(expr, symbols)?;
                // Define the variable (enforces immutability)
                symbols.define(&name, value)?;
            }
        }
    }
    Ok(())
}

// Evaluates an expression
pub fn evaluate_expression(expr: Expr, symbols: &SymbolTable) -> Result<Value, String> {
    match expr {
        Expr::Number(num) => Ok(Value::Number(num)),
        Expr::Text(text) => Ok(Value::Text(text)),
        Expr::Variable(name) => symbols
            .get(&name)
            .cloned()
            .ok_or_else(|| format!("Undefined variable '{}'", name)),
        Expr::Add(lhs, rhs) => {
            let lhs = evaluate_expression(*lhs, symbols)?;
            let rhs = evaluate_expression(*rhs, symbols)?;
            match (lhs, rhs) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),
                _ => Err("Addition is only supported for numbers".to_string()),
            }
        }
    }
}
