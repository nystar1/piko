use lexpr::Value;
use crate::utils::error::{VMError, VMResult};
use super::expressions::{Expression, BinaryOp, ChainOp};

pub struct Parser;

impl Parser {
    pub fn parse_expression(input: &str) -> VMResult<Expression> {
        let cleaned = Self::strip_comments(input);
        let trimmed = cleaned.trim();
        
        if trimmed.is_empty() {
            return Err(VMError::ParseError("Empty input".to_string()));
        }
        
        if Self::is_string_literal(trimmed) {
            let content = &trimmed[1..trimmed.len()-1];
            return Ok(Expression::Literal(content.to_string()));
        }
        
        if Self::is_variable(trimmed) {
            return Ok(Expression::Variable(trimmed.to_string()));
        }
        
        if Self::is_s_expression(trimmed) {
            let parsed = lexpr::from_str(trimmed)
                .map_err(|e| VMError::ParseError(format!("Invalid S-expression: {}", e)))?;
            return Self::parse_sexpr(&parsed);
        }
        
        Ok(Expression::Literal(trimmed.to_string()))
    }
    
    fn is_string_literal(s: &str) -> bool {
        s.starts_with('"') && s.ends_with('"') && s.len() >= 2
    }
    
    fn is_s_expression(s: &str) -> bool {
        s.starts_with('(') && s.ends_with(')')
    }
    
    fn parse_sexpr(value: &Value) -> VMResult<Expression> {
        match value {
            Value::String(s) | Value::Symbol(s) => {
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
                
                let op = Self::extract_operator(&list[0])?;
                Self::parse_operation(&op, &list)
            }
            _ => Err(VMError::ParseError("Invalid expression".to_string())),
        }
    }
    
    fn extract_operator(value: &Value) -> VMResult<String> {
        match value {
            Value::Symbol(s) => Ok(s.to_string()),
            _ => Err(VMError::ParseError("First element must be an operator".to_string())),
        }
    }
    
    fn parse_operation(op: &str, list: &[Value]) -> VMResult<Expression> {
        match op {
            "o" => Self::parse_output(list),
            "i" => Self::parse_input(list),
            "a" => Self::parse_assign(list),
            "r" => Self::parse_return(list),
            "c" => Self::parse_call(list),
            "f" => Self::parse_function(list),
            "l" => Self::parse_loop(list),
            "b" => Self::parse_break(list),
            "+" => Self::parse_binary_op(list, BinaryOp::Add),
            "-" => Self::parse_binary_op(list, BinaryOp::Sub),
            "*" => Self::parse_binary_op(list, BinaryOp::Mul),
            "/" => Self::parse_binary_op(list, BinaryOp::Div),
            "<" => Self::parse_binary_op(list, BinaryOp::Lt),
            ">" => Self::parse_binary_op(list, BinaryOp::Gt),
            "<=" => Self::parse_binary_op(list, BinaryOp::Le),
            ">=" => Self::parse_binary_op(list, BinaryOp::Ge),
            "==" => Self::parse_binary_op(list, BinaryOp::Eq),
            "!=" => Self::parse_binary_op(list, BinaryOp::Ne),
            _ => {
                if Self::is_chain_op(op) {
                    Self::parse_chain_op(list)
                } else {
                    Err(VMError::ParseError(format!("Unknown operator: {}", op)))
                }
            }
        }
    }
    
    fn parse_output(list: &[Value]) -> VMResult<Expression> {
        if list.len() != 2 {
            return Err(VMError::ParseError("o expects 1 argument".to_string()));
        }
        let arg = Self::parse_sexpr(&list[1])?;
        Ok(Expression::Output(Box::new(arg)))
    }
    
    fn parse_input(list: &[Value]) -> VMResult<Expression> {
        if list.len() != 2 {
            return Err(VMError::ParseError("i expects 1 argument".to_string()));
        }
        let var = Self::extract_symbol(&list[1], "i expects a variable name")?;
        Ok(Expression::Input(var))
    }
    
    fn parse_assign(list: &[Value]) -> VMResult<Expression> {
        if list.len() != 3 {
            return Err(VMError::ParseError("a expects 2 arguments".to_string()));
        }
        let var = Self::extract_symbol(&list[1], "a expects a variable name")?;
        let expr = Self::parse_sexpr(&list[2])?;
        Ok(Expression::Assign(var, Box::new(expr)))
    }
    
    fn parse_return(list: &[Value]) -> VMResult<Expression> {
        if list.len() != 2 {
            return Err(VMError::ParseError("r expects 1 argument".to_string()));
        }
        let expr = Self::parse_sexpr(&list[1])?;
        Ok(Expression::Return(Box::new(expr)))
    }
    
    fn parse_call(list: &[Value]) -> VMResult<Expression> {
        if list.len() < 2 {
            return Err(VMError::ParseError("c expects at least 1 argument".to_string()));
        }
        let func_name = Self::extract_symbol(&list[1], "c expects a function name")?;
        let mut args = Vec::new();
        for i in 2..list.len() {
            args.push(Self::parse_sexpr(&list[i])?);
        }
        Ok(Expression::Call(func_name, args))
    }
    
    fn parse_function(list: &[Value]) -> VMResult<Expression> {
        if list.len() < 4 {
            return Err(VMError::ParseError("f expects at least 3 arguments".to_string()));
        }
        let func_name = Self::extract_symbol(&list[1], "f expects a function name")?;
        let mut params = Vec::new();
        for i in 2..list.len()-1 {
            let param = Self::extract_symbol(&list[i], "f expects parameter names")?;
            params.push(param);
        }
        let body = Self::parse_sexpr(&list[list.len()-1])?;
        Ok(Expression::Function(func_name, params, Box::new(body)))
    }
    
    fn parse_loop(list: &[Value]) -> VMResult<Expression> {
        if list.len() < 2 {
            return Err(VMError::ParseError("l expects at least 1 argument".to_string()));
        }
        if list.len() == 2 {
            let body = Self::parse_sexpr(&list[1])?;
            Ok(Expression::Loop(None, Box::new(body)))
        } else {
            let condition = Self::parse_sexpr(&list[1])?;
            let mut body_exprs = Vec::new();
            for i in 2..list.len() {
                body_exprs.push(Self::parse_sexpr(&list[i])?);
            }
            let body = if body_exprs.len() == 1 {
                body_exprs.into_iter().next().unwrap()
            } else {
                Expression::Block(body_exprs)
            };
            Ok(Expression::Loop(Some(Box::new(condition)), Box::new(body)))
        }
    }
    
    fn parse_break(list: &[Value]) -> VMResult<Expression> {
        if list.len() != 1 {
            return Err(VMError::ParseError("b expects no arguments".to_string()));
        }
        Ok(Expression::Break)
    }
    
    fn extract_symbol(value: &Value, error_msg: &str) -> VMResult<String> {
        match value {
            Value::Symbol(s) => Ok(s.to_string()),
            _ => Err(VMError::ParseError(error_msg.to_string())),
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
        op.len() > 1 && op.chars().all(|c| matches!(c, 'o' | 'i' | 'a' | 'r' | 'c' | 'f' | 'l' | 'b'))
    }
    
    fn parse_chain_op(list: &[Value]) -> VMResult<Expression> {
        let chain_str = Self::extract_operator(&list[0])?;
        let mut ops = Vec::new();
        let mut arg_index = 1;
        
        for c in chain_str.chars() {
            match c {
                'o' => ops.push(ChainOp::Output),
                'i' => {
                    if arg_index >= list.len() {
                        return Err(VMError::ParseError("Missing argument for i in chain".to_string()));
                    }
                    let var = Self::extract_symbol(&list[arg_index], "i expects a variable name")?;
                    ops.push(ChainOp::Input(var));
                    arg_index += 1;
                }
                'a' => {
                    if arg_index + 1 >= list.len() {
                        return Err(VMError::ParseError("Missing arguments for a in chain".to_string()));
                    }
                    let var = Self::extract_symbol(&list[arg_index], "a expects a variable name")?;
                    let expr = Self::parse_sexpr(&list[arg_index + 1])?;
                    ops.push(ChainOp::Assign(var, Box::new(expr)));
                    arg_index += 2;
                }
                'r' => {
                    if arg_index >= list.len() {
                        return Err(VMError::ParseError("Missing argument for r in chain".to_string()));
                    }
                    let expr = Self::parse_sexpr(&list[arg_index])?;
                    ops.push(ChainOp::Return(Box::new(expr)));
                    arg_index += 1;
                }
                'b' => ops.push(ChainOp::Break),
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
        !s.is_empty() && s.chars().all(|c| c.is_ascii_lowercase())
    }
    
    fn strip_comments(input: &str) -> String {
        let mut result = String::new();
        let mut in_string = false;
        let mut escape_next = false;
        
        for ch in input.chars() {
            if escape_next {
                result.push(ch);
                escape_next = false;
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
                '#' if !in_string => break,
                _ => result.push(ch),
            }
        }
        
        result
    }
}