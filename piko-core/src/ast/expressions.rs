use serde::{Deserialize, Serialize};

use crate::error::VMResult;
use crate::utils::base_26;
use super::traits::{Atom, Chain, Evaluable, Parseable};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Lt,
    Gt,
    Le,
    Ge,
    Eq,
    Ne,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Expression {
    Variable(String),
    Literal(String),
    BinaryOp(Box<Expression>, BinaryOp, Box<Expression>),
    Output(Box<Expression>),
    Input(String),
    Assign(String, Box<Expression>),
    Return(Box<Expression>),
    Call(String, Vec<Expression>),
    Function(String, Vec<String>, Box<Expression>),
    Loop(Option<Box<Expression>>, Box<Expression>),
    Break,
    ChainedOp(Vec<ChainOp>),
    Block(Vec<Expression>),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChainOp {
    Input(String),
    Output,
    Assign(String, Box<Expression>),
    Return(Box<Expression>),
    Call(String, Vec<Expression>),
    Function(String, Vec<String>, Box<Expression>),
    Loop(Option<Box<Expression>>, Box<Expression>),
    Break,
}

impl Parseable for Expression {
    fn parse(input: &str) -> VMResult<Self> {
        super::parser::Parser::parse_expression(input)
    }
}

impl Evaluable for Expression {
    fn evaluate(&self) -> VMResult<String> {
        match self {
            Expression::Variable(name) => Ok(name.clone()),
            Expression::Literal(value) => Ok(value.clone()),
            Expression::BinaryOp(left, op, right) => {
                let left_val = left.evaluate()?;
                let right_val = right.evaluate()?;
                self.apply_binary_op(&left_val, op, &right_val)
            }
            Expression::Output(expr) => {
                let value = expr.evaluate()?;
                println!("{}", value);
                Ok(value)
            }
            Expression::Input(_var) => {
                use std::io::{self, Write};
                print!("");
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                Ok(input.trim().to_string())
            }
            Expression::Assign(_var, expr) => {
                let value = expr.evaluate()?;
                Ok(value)
            }
            Expression::Return(expr) => {
                expr.evaluate()
            }
            Expression::Call(func, args) => {
                let mut arg_values = Vec::new();
                for arg in args {
                    arg_values.push(arg.evaluate()?);
                }
                Ok(format!("call_{}_{}", func, arg_values.join("_")))
            }
            Expression::Function(name, _params, _body) => {
                Ok(format!("function_{}", name))
            }
            Expression::Loop(_condition, _body) => {
                Ok("loop_result".to_string())
            }
            Expression::Break => Ok("break".to_string()),
            Expression::ChainedOp(ops) => {
                let mut result = String::new();
                for op in ops {
                    match op {
                        ChainOp::Input(_var) => {
                            use std::io::{self, Write};
                            print!("");
                            io::stdout().flush().unwrap();
                            let mut input = String::new();
                            io::stdin().read_line(&mut input).unwrap();
                            result = input.trim().to_string();
                        }
                        ChainOp::Output => {
                            println!("{}", result);
                        }
                        ChainOp::Assign(_var, expr) => {
                            let value = expr.evaluate()?;
                            result = value;
                        }
                        ChainOp::Return(expr) => {
                            result = expr.evaluate()?;
                        }
                        ChainOp::Call(func, args) => {
                            let mut arg_values = Vec::new();
                            for arg in args {
                                arg_values.push(arg.evaluate()?);
                            }
                            result = format!("call_{}_{}", func, arg_values.join("_"));
                        }
                        ChainOp::Function(name, _params, _body) => {
                            result = format!("function_{}", name);
                        }
                        ChainOp::Loop(_condition, _body) => {
                            result = "loop_result".to_string();
                        }
                        ChainOp::Break => {
                            result = "break".to_string();
                        }
                    }
                }
                Ok(result)
            }
            Expression::Block(exprs) => {
                let mut result = String::new();
                for expr in exprs {
                    result = expr.evaluate()?;
                }
                Ok(result)
            }
        }
    }
}

impl Expression {
    pub fn apply_binary_op(&self, left: &str, op: &BinaryOp, right: &str) -> VMResult<String> {
        match op {
            BinaryOp::Add => Ok(base_26::add(left, right)),
            BinaryOp::Sub => Ok(base_26::sub(left, right)),
            BinaryOp::Mul => Ok(base_26::mul(left, right)),
            BinaryOp::Div => Ok(base_26::div(left, right)),
            BinaryOp::Lt => Ok(if base_26::compare_lt(left, right) { "b" } else { "a" }.to_string()),
            BinaryOp::Gt => Ok(if base_26::compare_gt(left, right) { "b" } else { "a" }.to_string()),
            BinaryOp::Le => Ok(if base_26::compare_le(left, right) { "b" } else { "a" }.to_string()),
            BinaryOp::Ge => Ok(if base_26::compare_ge(left, right) { "b" } else { "a" }.to_string()),
            BinaryOp::Eq => Ok(if base_26::compare_eq(left, right) { "b" } else { "a" }.to_string()),
            BinaryOp::Ne => Ok(if base_26::compare_ne(left, right) { "b" } else { "a" }.to_string()),
        }
    }
}

impl Atom for Expression {
    fn is_single_letter(&self) -> bool {
        match self {
            Expression::Variable(name) => name.len() == 1,
            Expression::Literal(val) => val.len() == 1,
            _ => false,
        }
    }
}

impl Chain for Expression {
    fn chain(&self, _next: Box<dyn Chain>) -> Box<dyn Chain> {
        Box::new(self.clone())
    }
}
