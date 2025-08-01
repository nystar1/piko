use std::collections::HashMap;
use std::io::{BufRead, Write};
use crate::ast::PikoAst;
use crate::ast::expressions::{Expression, BinaryOp};
use crate::utils::error::{VMError, VMResult};
use crate::utils::base_26;

pub mod constants;

pub struct VM<W: Write, R: BufRead> {
    functions: HashMap<String, (Vec<String>, Expression)>,
    variables: HashMap<String, String>,
    output: W,
    input: R,
}

impl<W: Write, R: BufRead> VM<W, R> {
    pub fn new(output: W, input: R) -> Self {
        VM {
            functions: HashMap::new(),
            variables: HashMap::new(),
            output,
            input,
        }
    }
    
    pub fn get_output(&mut self) -> &mut W {
        &mut self.output
    }
    
    pub fn execute(&mut self, ast: PikoAst) -> VMResult<()> {
        match ast {
            PikoAst::Expression(expr) => {
                self.evaluate_expression(&expr)?;
            }
            PikoAst::Program(nodes) => {
                for node in nodes {
                    self.execute(node)?;
                }
            }
        }
        Ok(())
    }
    
    fn evaluate_expression(&mut self, expr: &Expression) -> VMResult<String> {
        match expr {
            Expression::Variable(name) => {
                Ok(self.variables.get(name).cloned().unwrap_or_else(|| name.clone()))
            }
            Expression::Literal(value) => Ok(value.clone()),
            Expression::BinaryOp(left, op, right) => {
                let left_val = self.evaluate_expression(left)?;
                let right_val = self.evaluate_expression(right)?;
                self.apply_binary_op(&left_val, op, &right_val)
            }
            Expression::Output(expr) => {
                let value = self.evaluate_expression(expr)?;
                writeln!(self.output, "{}", value)
                    .map_err(|e| VMError::ExecutionError(e.to_string()))?;
                Ok(value)
            }
            Expression::Input(var) => {
                let mut input = String::new();
                self.input.read_line(&mut input)
                    .map_err(|e| VMError::ExecutionError(e.to_string()))?;
                let input = input.trim().to_string();
                self.variables.insert(var.clone(), input.clone());
                Ok(input)
            }
            Expression::Assign(var, expr) => {
                let value = self.evaluate_expression(expr)?;
                self.variables.insert(var.clone(), value.clone());
                Ok(value)
            }
            Expression::Return(expr) => self.evaluate_expression(expr),
            Expression::Call(func, args) => {
                let arg_values = args.iter()
                    .map(|arg| self.evaluate_expression(arg))
                    .collect::<VMResult<Vec<_>>>()?;
                self.call_function(func, arg_values)
            }
            Expression::Function(name, params, body) => {
                self.functions.insert(name.clone(), (params.clone(), body.as_ref().clone()));
                Ok(format!("function_{}", name))
            }
            Expression::Loop(condition, body) => {
                self.execute_loop(condition.as_deref(), body)
            }
            Expression::Break => Ok("break".to_string()),
            Expression::ChainedOp(ops) => {
                let mut result = String::new();
                for op in ops {
                    result = self.execute_chain_op(op, result)?;
                }
                Ok(result)
            }
            Expression::Block(exprs) => {
                let mut result = String::new();
                for expr in exprs {
                    result = self.evaluate_expression(expr)?;
                }
                Ok(result)
            }
        }
    }
    
    fn apply_binary_op(&self, left: &str, op: &BinaryOp, right: &str) -> VMResult<String> {
        let result = match op {
            BinaryOp::Add => base_26::add(left, right),
            BinaryOp::Sub => base_26::sub(left, right),
            BinaryOp::Mul => base_26::mul(left, right),
            BinaryOp::Div => base_26::div(left, right),
            BinaryOp::Lt => self.bool_to_string(base_26::compare_lt(left, right)),
            BinaryOp::Gt => self.bool_to_string(base_26::compare_gt(left, right)),
            BinaryOp::Le => self.bool_to_string(base_26::compare_le(left, right)),
            BinaryOp::Ge => self.bool_to_string(base_26::compare_ge(left, right)),
            BinaryOp::Eq => self.bool_to_string(base_26::compare_eq(left, right)),
            BinaryOp::Ne => self.bool_to_string(base_26::compare_ne(left, right)),
        };
        Ok(result)
    }
    
    fn bool_to_string(&self, value: bool) -> String {
        if value { "b" } else { "a" }.to_string()
    }
    
    fn execute_loop(&mut self, condition: Option<&Expression>, body: &Expression) -> VMResult<String> {
        loop {
            if let Some(cond) = condition {
                let cond_result = self.evaluate_expression(cond)?;
                if self.is_false(&cond_result) {
                    break;
                }
            }
            
            let result = self.evaluate_expression(body)?;
            if result == "break" {
                break;
            }
        }
        Ok("loop_completed".to_string())
    }
    
    fn is_false(&self, value: &str) -> bool {
        value == "a"
    }
    
    fn execute_chain_op(&mut self, op: &crate::ast::expressions::ChainOp, current_result: String) -> VMResult<String> {
        match op {
            crate::ast::expressions::ChainOp::Input(var) => {
                let mut input = String::new();
                self.input.read_line(&mut input)
                    .map_err(|e| VMError::ExecutionError(e.to_string()))?;
                let input = input.trim().to_string();
                self.variables.insert(var.clone(), input.clone());
                Ok(input)
            }
            crate::ast::expressions::ChainOp::Output => {
                writeln!(self.output, "{}", current_result)
                    .map_err(|e| VMError::ExecutionError(e.to_string()))?;
                Ok(current_result)
            }
            crate::ast::expressions::ChainOp::Assign(var, expr) => {
                let value = self.evaluate_expression(expr)?;
                self.variables.insert(var.clone(), value.clone());
                Ok(value)
            }
            crate::ast::expressions::ChainOp::Return(expr) => {
                self.evaluate_expression(expr)
            }
            crate::ast::expressions::ChainOp::Call(func, args) => {
                let arg_values = args.iter()
                    .map(|arg| self.evaluate_expression(arg))
                    .collect::<VMResult<Vec<_>>>()?;
                self.call_function(func, arg_values)
            }
            crate::ast::expressions::ChainOp::Function(name, params, body) => {
                self.functions.insert(name.clone(), (params.clone(), body.as_ref().clone()));
                Ok(format!("function_{}", name))
            }
            crate::ast::expressions::ChainOp::Loop(condition, body) => {
                self.execute_loop(condition.as_deref(), body)
            }
            crate::ast::expressions::ChainOp::Break => Ok("break".to_string()),
        }
    }
    
    fn call_function(&mut self, name: &str, args: Vec<String>) -> VMResult<String> {
        let (params, body) = self.functions.get(name).cloned()
            .ok_or_else(|| VMError::RuntimeError(format!("Unknown function: {}", name)))?;
        
        if args.len() != params.len() {
            return Err(VMError::RuntimeError(format!(
                "Function {} expects {} arguments, got {}",
                name, params.len(), args.len()
            )));
        }
        
        let old_vars = self.variables.clone();
        
        for (param, arg) in params.iter().zip(args.iter()) {
            self.variables.insert(param.clone(), arg.clone());
        }
        
        let result = self.evaluate_expression(&body);
        
        self.variables = old_vars;
        result
    }
}