pub mod expressions;
pub mod parser;

use serde::{Deserialize, Serialize};

use crate::utils::error::{VMError, VMResult};
use self::expressions::{Expression, Atom, Parseable};

pub use expressions::{Expression as PikoExpression, BinaryOp, Parseable as PikoParseable, Atom as PikoAtom};
pub use parser::Parser;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PikoAst {
    Expression(Expression),
    Program(Vec<PikoAst>),
}

impl Parseable for PikoAst {
    fn parse(input: &str) -> VMResult<Self> {
        let trimmed = input.trim();
        
        if trimmed.is_empty() {
            return Err(VMError::ParseError("Empty input".to_string()));
        }
        
        let lines: Vec<&str> = trimmed.lines().collect();
        if lines.len() > 1 {
            let mut program = Vec::new();
            for line in lines {
                if !line.trim().is_empty() {
                    let expr = Expression::parse(line)?;
                    program.push(PikoAst::Expression(expr));
                }
            }
            return Ok(PikoAst::Program(program));
        }
        
        let expr = Expression::parse(trimmed)?;
        Ok(PikoAst::Expression(expr))
    }
}

impl Atom for PikoAst {
    fn is_single_letter(&self) -> bool {
        match self {
            PikoAst::Expression(expr) => expr.is_single_letter(),
            PikoAst::Program(_) => false,
        }
    }
}
