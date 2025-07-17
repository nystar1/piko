use lexpr::Value;
use crate::error::{VMError, VMResult};
use super::expressions::{Expression, BinaryOp, ChainOp};

pub struct Parser;

impl Parser {
    pub fn parse_expression(input: &str) -> VMResult<Expression> {
        let cleaned = Self::strip_comments(input);
        let trimmed = cleaned.trim();
        
        if trimmed.is_empty() {
            return Err(VMError::ParseError("Empty input".to_string()));
        }
        
        if trimmed.starts_with('"') && trimmed.ends_with('"') && trimmed.len() >= 2 {
            let content = &trimmed[1..trimmed.len()-1];
            return Ok(Expression::Literal(content.to_string()));
        }
        
        if Self::is_variable(trimmed) {
            return Ok(Expression::Variable(trimmed.to_string()));
        }
        
        if trimmed.starts_with('(') && trimmed.ends_with(')') {
            let parsed = lexpr::from_str(trimmed)
                .map_err(|e| VMError::ParseError(format!("Invalid S-expression: {}", e)))?;
            return Self::parse_sexpr(&parsed);
        }
        
        Ok(Expression::Literal(trimmed.to_string()))
    }
    
    fn parse_sexpr(value: &Value) -> VMResult<Expression> {
        match value {
            Value::String(s) => {
                if Self::is_variable(s) {
                    Ok(Expression::Variable(s.to_string()))
                } else {
                    Ok(Expression::Literal(s.to_string()))
                }
            }
            Value::Symbol(s) => {
                if Self::is_variable(s) {
                    Ok(Expression::Variable(s.to_string()))
                } else {
                    Ok(Expression::Literal(s.to_string()))
                }
            }
            Value::Cons(cons) => {
                let list = Self::cons_to_vec(cons)?;
                if list.is_empty() {
                    return Err(VMError::ParseError("Empty list".to_string()));
                }
                
                let op = match &list[0] {
                    Value::Symbol(s) => s.to_string(),
                    _ => return Err(VMError::ParseError("First element must be an operator".to_string())),
                };
                
                match op.as_str() {
                    "o" => {
                        if list.len() == 2 {
                            let arg = Self::parse_sexpr(&list[1])?;
                            Ok(Expression::Output(Box::new(arg)))
                        } else {
                            Err(VMError::ParseError("o expects 1 argument".to_string()))
                        }
                    }
                    "i" => {
                        if list.len() == 2 {
                            let var = match &list[1] {
                                Value::Symbol(s) => s.to_string(),
                                _ => return Err(VMError::ParseError("i expects a variable name".to_string())),
                            };
                            Ok(Expression::Input(var))
                        } else {
                            Err(VMError::ParseError("i expects 1 argument".to_string()))
                        }
                    }
                    "a" => {
                        if list.len() == 3 {
                            let var = match &list[1] {
                                Value::Symbol(s) => s.to_string(),
                                _ => return Err(VMError::ParseError("a expects a variable name".to_string())),
                            };
                            let expr = Self::parse_sexpr(&list[2])?;
                            Ok(Expression::Assign(var, Box::new(expr)))
                        } else {
                            Err(VMError::ParseError("a expects 2 arguments".to_string()))
                        }
                    }
                    "r" => {
                        if list.len() == 2 {
                            let expr = Self::parse_sexpr(&list[1])?;
                            Ok(Expression::Return(Box::new(expr)))
                        } else {
                            Err(VMError::ParseError("r expects 1 argument".to_string()))
                        }
                    }
                    "c" => {
                        if list.len() >= 2 {
                            let func_name = match &list[1] {
                                Value::Symbol(s) => s.to_string(),
                                _ => return Err(VMError::ParseError("c expects a function name".to_string())),
                            };
                            let mut args = Vec::new();
                            for i in 2..list.len() {
                                args.push(Self::parse_sexpr(&list[i])?);
                            }
                            Ok(Expression::Call(func_name, args))
                        } else {
                            Err(VMError::ParseError("c expects at least 1 argument".to_string()))
                        }
                    }
                    "f" => {
                        if list.len() >= 4 {
                            let func_name = match &list[1] {
                                Value::Symbol(s) => s.to_string(),
                                _ => return Err(VMError::ParseError("f expects a function name".to_string())),
                            };
                            let mut params = Vec::new();
                            for i in 2..list.len()-1 {
                                match &list[i] {
                                    Value::Symbol(s) => params.push(s.to_string()),
                                    _ => return Err(VMError::ParseError("f expects parameter names".to_string())),
                                }
                            }
                            let body = Self::parse_sexpr(&list[list.len()-1])?;
                            Ok(Expression::Function(func_name, params, Box::new(body)))
                        } else {
                            Err(VMError::ParseError("f expects at least 3 arguments".to_string()))
                        }
                    }
                    "l" => {
                        if list.len() == 2 {
                            let body = Self::parse_sexpr(&list[1])?;
                            Ok(Expression::Loop(None, Box::new(body)))
                        } else if list.len() == 3 {
                            let condition = Self::parse_sexpr(&list[1])?;
                            let body = Self::parse_sexpr(&list[2])?;
                            Ok(Expression::Loop(Some(Box::new(condition)), Box::new(body)))
                        } else {
                            Err(VMError::ParseError("l expects 1 or 2 arguments".to_string()))
                        }
                    }
                    "b" => {
                        if list.len() == 1 {
                            Ok(Expression::Break)
                        } else {
                            Err(VMError::ParseError("b expects no arguments".to_string()))
                        }
                    }
                    "+" => Self::parse_binary_op(&list, BinaryOp::Add),
                    "-" => Self::parse_binary_op(&list, BinaryOp::Sub),
                    "*" => Self::parse_binary_op(&list, BinaryOp::Mul),
                    "/" => Self::parse_binary_op(&list, BinaryOp::Div),
                    "<" => Self::parse_binary_op(&list, BinaryOp::Lt),
                    ">" => Self::parse_binary_op(&list, BinaryOp::Gt),
                    "<=" => Self::parse_binary_op(&list, BinaryOp::Le),
                    ">=" => Self::parse_binary_op(&list, BinaryOp::Ge),
                    "==" => Self::parse_binary_op(&list, BinaryOp::Eq),
                    "!=" => Self::parse_binary_op(&list, BinaryOp::Ne),
                    _ => {
                        if Self::is_chain_op(&op) {
                            Self::parse_chain_op(&list)
                        } else {
                            Err(VMError::ParseError(format!("Unknown operator: {}", op)))
                        }
                    }
                }
            }
            _ => Err(VMError::ParseError("Invalid expression".to_string())),
        }
    }
    
    fn parse_binary_op(list: &[Value], op: BinaryOp) -> VMResult<Expression> {
        if list.len() != 3 {
            return Err(VMError::ParseError("Binary operator expects 2 arguments".to_string()));
        }
        let left = Self::parse_sexpr(&list[1])?;
        let right = Self::parse_sexpr(&list[2])?;
        Ok(Expression::BinaryOp(Box::new(left), op, Box::new(right)))
    }
    
    fn is_chain_op(op: &str) -> bool {
        op.chars().all(|c| matches!(c, 'o' | 'i' | 'a' | 'r' | 'c' | 'f' | 'l' | 'b')) && op.len() > 1
    }
    
    fn parse_chain_op(list: &[Value]) -> VMResult<Expression> {
        let chain_str = match &list[0] {
            Value::Symbol(s) => s.to_string(),
            _ => return Err(VMError::ParseError("Chain operator must be a symbol".to_string())),
        };
        
        let mut ops = Vec::new();
        let mut arg_index = 1;
        
        for c in chain_str.chars() {
            match c {
                'o' => {
                    if arg_index < list.len() {
                        let expr = Self::parse_sexpr(&list[arg_index])?;
                        ops.push(ChainOp::Output(Box::new(expr)));
                        arg_index += 1;
                    } else {
                        return Err(VMError::ParseError("Missing argument for o in chain".to_string()));
                    }
                }
                'i' => {
                    if arg_index < list.len() {
                        let var = match &list[arg_index] {
                            Value::Symbol(s) => s.to_string(),
                            _ => return Err(VMError::ParseError("i expects a variable name".to_string())),
                        };
                        ops.push(ChainOp::Input(var));
                        arg_index += 1;
                    } else {
                        return Err(VMError::ParseError("Missing argument for i in chain".to_string()));
                    }
                }
                'a' => {
                    if arg_index + 1 < list.len() {
                        let var = match &list[arg_index] {
                            Value::Symbol(s) => s.to_string(),
                            _ => return Err(VMError::ParseError("a expects a variable name".to_string())),
                        };
                        let expr = Self::parse_sexpr(&list[arg_index + 1])?;
                        ops.push(ChainOp::Assign(var, Box::new(expr)));
                        arg_index += 2;
                    } else {
                        return Err(VMError::ParseError("Missing arguments for a in chain".to_string()));
                    }
                }
                'r' => {
                    if arg_index < list.len() {
                        let expr = Self::parse_sexpr(&list[arg_index])?;
                        ops.push(ChainOp::Return(Box::new(expr)));
                        arg_index += 1;
                    } else {
                        return Err(VMError::ParseError("Missing argument for r in chain".to_string()));
                    }
                }
                'b' => {
                    ops.push(ChainOp::Break);
                }
                _ => return Err(VMError::ParseError(format!("Invalid chain operator: {}", c))),
            }
        }
        
        Ok(Expression::ChainedOp(ops))
    }
    
    fn cons_to_vec(cons: &lexpr::Cons) -> VMResult<Vec<Value>> {
        let mut result = Vec::new();
        let mut current = cons;
        
        loop {
            result.push(current.car().clone());
            match current.cdr() {
                Value::Cons(next_cons) => current = next_cons,
                Value::Nil | Value::Null => break,
                _ => return Err(VMError::ParseError("Improper list".to_string())),
            }
        }
        
        Ok(result)
    }
    
    fn is_variable(s: &str) -> bool {
        s.chars().all(|c| c.is_ascii_lowercase()) && !s.is_empty()
    }
    
    fn strip_comments(input: &str) -> String {
        let mut result = String::new();
        let mut in_string = false;
        let mut escape_next = false;
        let chars: Vec<char> = input.chars().collect();
        let mut i = 0;
        
        while i < chars.len() {
            let ch = chars[i];
            
            if escape_next {
                result.push(ch);
                escape_next = false;
                i += 1;
                continue;
            }
            
            match ch {
                '\\' if in_string => {
                    escape_next = true;
                    result.push(ch);
                }
                '"' => {
                    in_string = !in_string;
                    result.push(ch);
                }
                '#' if !in_string => {
                    break;
                }
                _ => {
                    result.push(ch);
                }
            }
            
            i += 1;
        }
        
        result
    }
}