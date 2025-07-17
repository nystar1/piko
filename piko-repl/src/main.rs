use piko_core::{VMError, VMResult, PikoAst};
use piko_core::vm::VM;
use piko_core::ast::traits::Parseable;
use std::fs;
use std::env;
use std::io::{self, Write, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 {
        let filename = &args[1];
        
        if let Err(e) = run_file(filename) {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    } else {
        if let Err(e) = run_repl() {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn run_file(filename: &str) -> VMResult<()> {
    let content = fs::read_to_string(filename)
        .map_err(|e| VMError::ExecutionError(format!("Error reading file '{}': {}", filename, e)))?;
    
    let mut vm = VM::new(io::stdout(), BufReader::new(io::stdin()));
    
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        let ast = PikoAst::parse(line)?;
        vm.execute(ast)?;
    }
    
    Ok(())
}

fn run_repl() -> VMResult<()> {
    println!("Welcome to Piko REPL!");
    println!("By Parth");
    println!("Type 'quit' or 'q' to exit");
    println!("You can also pass a .pyx file: cargo run filename.pyx");
    
    let mut vm = VM::new(io::stdout(), BufReader::new(io::stdin()));
    
    loop {
        print!("piko> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        if input == "quit" || input == "q" {
            break;
        }
        
        if input.is_empty() {
            continue;
        }
        
        match PikoAst::parse(input) {
            Ok(ast) => {
                if let Err(e) = vm.execute(ast) {
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

