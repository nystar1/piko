use std::fmt;

#[derive(Debug, Clone)]
pub enum VMError {
    ParseError(String),
    CompileError(String),
    ExecutionError(String),
    RuntimeError(String),
    StackUnderflow,
    UnknownFunction(String),
    InvalidOperation(String),
}

impl fmt::Display for VMError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VMError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            VMError::CompileError(msg) => write!(f, "Compile error: {}", msg),
            VMError::ExecutionError(msg) => write!(f, "Execution error: {}", msg),
            VMError::RuntimeError(msg) => write!(f, "Runtime error: {}", msg),
            VMError::StackUnderflow => write!(f, "Stack underflow"),
            VMError::UnknownFunction(name) => write!(f, "Unknown function: {}", name),
            VMError::InvalidOperation(msg) => write!(f, "Invalid operation: {}", msg),
        }
    }
}

impl std::error::Error for VMError {}

pub type VMResult<T> = Result<T, VMError>;
