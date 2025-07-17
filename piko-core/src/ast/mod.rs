pub mod traits;
pub mod expressions;
pub mod statements;
pub mod parser;

use serde::{Deserialize, Serialize};

use crate::error::{VMError, VMResult};
use self::expressions::Expression;
use self::statements::Statement;
use self::traits::{Atom, Chain, Evaluable, Parseable};

pub use expressions::{Expression as PikoExpression, BinaryOp};
pub use statements::Statement as PikoStatement;
pub use traits::{Parseable as PikoParseable, Evaluable as PikoEvaluable, Atom as PikoAtom, Chain as PikoChain};
pub use parser::Parser;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PikoAst {
    Expression(Expression),
    Statement(Statement),
    Program(Vec<PikoAst>),
}

impl Chain for PikoAst {
    fn chain(&self, _next: Box<dyn Chain>) -> Box<dyn Chain> {
        Box::new(self.clone())
    }
}

impl Parseable for PikoAst {
    fn parse(input: &str) -> VMResult<Self> {
        let trimmed = input.trim();
        
        if trimmed.is_empty() {
            return Err(VMError::ParseError("Empty input".to_string()));
        }
        
        if trimmed.starts_with('(') && trimmed.ends_with(')') {
            match Expression::parse(trimmed) {
                Ok(expr) => return Ok(PikoAst::Expression(expr)),
                Err(_) => {
                    match Statement::parse(trimmed) {
                        Ok(stmt) => return Ok(PikoAst::Statement(stmt)),
                        Err(stmt_err) => return Err(stmt_err),
                    }
                }
            }
        }
        
        if let Ok(expr) = Expression::parse(trimmed) {
            return Ok(PikoAst::Expression(expr));
        }
        
        let lines: Vec<&str> = trimmed.lines().collect();
        if lines.len() > 1 {
            let mut program = Vec::new();
            for line in lines {
                if !line.trim().is_empty() {
                    program.push(PikoAst::parse(line)?);
                }
            }
            return Ok(PikoAst::Program(program));
        }
        
        Err(VMError::ParseError("Failed to parse AST: unknown syntax".to_string()))
    }
}

impl Evaluable for PikoAst {
    fn evaluate(&self) -> VMResult<String> {
        match self {
            PikoAst::Expression(expr) => expr.evaluate(),
            PikoAst::Statement(stmt) => stmt.evaluate(),
            PikoAst::Program(nodes) => {
                let mut results = Vec::new();
                for node in nodes {
                    results.push(node.evaluate()?);
                }
                Ok(results.join("\n"))
            }
        }
    }
}

impl Atom for PikoAst {
    fn is_single_letter(&self) -> bool {
        match self {
            PikoAst::Expression(expr) => expr.is_single_letter(),
            PikoAst::Statement(stmt) => stmt.is_single_letter(),
            PikoAst::Program(_) => false,
        }
    }
}
