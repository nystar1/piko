use serde::{Deserialize, Serialize};

use crate::utils::error::VMResult;

pub trait Parseable {
    fn parse(input: &str) -> VMResult<Self> where Self: Sized;
}

pub trait Atom {
    fn is_single_letter(&self) -> bool;
}

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

impl Atom for Expression {
    fn is_single_letter(&self) -> bool {
        match self {
            Expression::Variable(name) => name.len() == 1,
            Expression::Literal(val) => val.len() == 1,
            _ => false,
        }
    }
}
