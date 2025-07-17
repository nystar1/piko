use serde::{Deserialize, Serialize};

use crate::error::VMResult;
use super::traits::{Atom, Chain, Evaluable, Parseable};
use super::expressions::Expression;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Statement {
    Expression(Expression),
}

impl Chain for Statement {
    fn chain(&self, _next: Box<dyn Chain>) -> Box<dyn Chain> {
        Box::new(self.clone())
    }
}

impl Parseable for Statement {
    fn parse(input: &str) -> VMResult<Self> {
        let expr = Expression::parse(input)?;
        Ok(Statement::Expression(expr))
    }
}

impl Evaluable for Statement {
    fn evaluate(&self) -> VMResult<String> {
        match self {
            Statement::Expression(expr) => expr.evaluate(),
        }
    }
}

impl Atom for Statement {
    fn is_single_letter(&self) -> bool {
        match self {
            Statement::Expression(expr) => expr.is_single_letter(),
        }
    }
}
