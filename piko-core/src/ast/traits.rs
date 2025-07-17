use crate::error::VMResult;

pub trait Parseable {
    fn parse(input: &str) -> VMResult<Self> where Self: Sized;
}

pub trait Evaluable {
    fn evaluate(&self) -> VMResult<String>;
}

pub trait Atom {
    fn is_single_letter(&self) -> bool;
}

pub trait Chain {
    fn chain(&self, next: Box<dyn Chain>) -> Box<dyn Chain>;
}
