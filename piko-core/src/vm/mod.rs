use std::collections::HashMap;
use std::io::{self, Write};
use crate::ast::PikoAst;
use crate::ast::expressions::Expression;
use crate::error::{VMError, VMResult};

pub mod constants;

pub struct VM {
    functions: HashMap<String, (Vec<String>, Expression)>,
    variables: HashMap<String, String>,
}

impl VM {
    pub fn new() -> Self {
        VM {
            functions: HashMap::new(),
            variables: HashMap::new(),
        }
    }
    
    pub fn execute(&mut self, ast: PikoAst) -> VMResult<()> {
        match ast {
            PikoAst::Expression(expr) => {
                let _result = self.evaluate_expression(&expr)?;
            }
            PikoAst::Statement(stmt) => {
                let _result = self.execute_statement(&stmt)?;
            }
            PikoAst::Program(nodes) => {
                for node in nodes {
                    self.execute(node)?;
                }
            }
        }
        Ok(())
    }
    
    pub fn execute_incremental(&mut self, ast: PikoAst) -> VMResult<()> {
        self.execute(ast)
    }
    
    fn execute_statement(&mut self, statement: &crate::ast::statements::Statement) -> VMResult<String> {
        match statement {
            crate::ast::statements::Statement::Expression(expr) => {
                self.evaluate_expression(expr)
            }
        }
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
                expr.apply_binary_op(&left_val, op, &right_val)
            }
            Expression::Output(expr) => {
                let value = self.evaluate_expression(expr)?;
                println!("{}", value);
                Ok(value)
            }
            Expression::Input(var) => {
                print!("");
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let input = input.trim().to_string();
                self.variables.insert(var.clone(), input.clone());
                Ok(input)
            }
            Expression::Assign(var, expr) => {
                let value = self.evaluate_expression(expr)?;
                self.variables.insert(var.clone(), value.clone());
                Ok(value)
            }
            Expression::Return(expr) => {
                self.evaluate_expression(expr)
            }
            Expression::Call(func, args) => {
                let mut arg_values = Vec::new();
                for arg in args {
                    arg_values.push(self.evaluate_expression(arg)?);
                }
                self.call_function(func, arg_values)
            }
            Expression::Function(name, params, body) => {
                self.functions.insert(name.clone(), (params.clone(), body.as_ref().clone()));
                Ok(format!("function_{}", name))
            }
            Expression::Loop(condition, body) => {
                loop {
                    if let Some(cond) = condition {
                        let cond_result = self.evaluate_expression(cond)?;
                        if cond_result == "a" {
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
            Expression::Break => Ok("break".to_string()),
            Expression::ChainedOp(ops) => {
                let mut result = String::new();
                for op in ops {
                    match op {
                        crate::ast::expressions::ChainOp::Input(var) => {
                            print!("");
                            io::stdout().flush().unwrap();
                            let mut input = String::new();
                            io::stdin().read_line(&mut input).unwrap();
                            result = input.trim().to_string();
                            self.variables.insert(var.clone(), result.clone());
                        }
                        crate::ast::expressions::ChainOp::Output(expr) => {
                            let value = self.evaluate_expression(expr)?;
                            println!("{}", value);
                            result = value;
                        }
                        crate::ast::expressions::ChainOp::Assign(var, expr) => {
                            let value = self.evaluate_expression(expr)?;
                            self.variables.insert(var.clone(), value.clone());
                            result = value;
                        }
                        crate::ast::expressions::ChainOp::Return(expr) => {
                            result = self.evaluate_expression(expr)?;
                        }
                        crate::ast::expressions::ChainOp::Call(func, args) => {
                            let mut arg_values = Vec::new();
                            for arg in args {
                                arg_values.push(self.evaluate_expression(arg)?);
                            }
                            result = self.call_function(func, arg_values)?;
                        }
                        crate::ast::expressions::ChainOp::Function(name, params, body) => {
                            self.functions.insert(name.clone(), (params.clone(), body.as_ref().clone()));
                            result = format!("function_{}", name);
                        }
                        crate::ast::expressions::ChainOp::Loop(condition, body) => {
                            loop {
                                if let Some(cond) = condition {
                                    let cond_result = self.evaluate_expression(cond)?;
                                    if cond_result == "a" {
                                        break;
                                    }
                                }
                                
                                let loop_result = self.evaluate_expression(body)?;
                                if loop_result == "break" {
                                    break;
                                }
                            }
                            result = "loop_completed".to_string();
                        }
                        crate::ast::expressions::ChainOp::Break => {
                            result = "break".to_string();
                        }
                    }
                }
                Ok(result)
            }
        }
    }
    
    fn call_function(&mut self, name: &str, args: Vec<String>) -> VMResult<String> {
        if let Some((params, body)) = self.functions.get(name).cloned() {
            let old_vars = self.variables.clone();
            
            if args.len() != params.len() {
                return Err(VMError::RuntimeError(format!(
                    "Function {} expects {} arguments, got {}",
                    name, params.len(), args.len()
                )));
            }
            
            for (param, arg) in params.iter().zip(args.iter()) {
                self.variables.insert(param.clone(), arg.clone());
            }
            
            let result = self.evaluate_expression(&body)?;
            
            self.variables = old_vars;
            Ok(result)
        } else {
            Err(VMError::RuntimeError(format!("Unknown function: {}", name)))
        }
    }
}