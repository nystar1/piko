pub mod ast;
pub mod error;
pub mod ir;
pub mod utils;
pub mod vm;

pub use ast::PikoAst;
pub use ast::traits::Parseable;
pub use error::{VMError, VMResult};

use std::io::{self, Write};

pub fn execute_repl() -> VMResult<()> {
    let mut vm = vm::VM::new();
    
    loop {
        print!("piko> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        if input == "q" || input == "quit" {
            break;
        }
        
        if input.is_empty() {
            continue;
        }
        
        match ast::PikoAst::parse(input) {
            Ok(ast) => {
                if let Err(e) = vm.execute_incremental(ast) {
                    eprintln!("Error: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Parse error: {}", e);
            }
        }
    }
    
    Ok(())
}