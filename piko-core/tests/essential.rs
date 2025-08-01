
use std::io::Cursor;

use piko_core::ast::expressions::Parseable;
use piko_core::ast::PikoAst;
use piko_core::vm::VM;

#[test]
fn test_basic_functionality() {
    let output = Vec::new();
    let input = Cursor::new(String::new());
    let mut vm = VM::new(output, input);
    
    let ast = PikoAst::parse("(o \"hello\")").unwrap();
    assert!(vm.execute(ast).is_ok());
    
    let ast = PikoAst::parse("(a x \"world\")").unwrap();
    assert!(vm.execute(ast).is_ok());
    
    let ast = PikoAst::parse("(o x)").unwrap();
    assert!(vm.execute(ast).is_ok());
    
    let ast = PikoAst::parse("(+ \"a\" \"b\")").unwrap();
    assert!(vm.execute(ast).is_ok());
}

#[test]
fn test_functions() {
    let output = Vec::new();
    let input = Cursor::new(String::new());
    let mut vm = VM::new(output, input);
    
    let ast = PikoAst::parse("(f test x (r x))").unwrap();
    assert!(vm.execute(ast).is_ok());
    
    let ast = PikoAst::parse("(c test \"hello\")").unwrap();
    assert!(vm.execute(ast).is_ok());
}

#[test]
fn test_chain_operations() {
    let output = Vec::new();
    let input = Cursor::new(String::new());
    let mut vm = VM::new(output, input);
    
    let ast = PikoAst::parse("(ao x \"test\")").unwrap();
    assert!(vm.execute(ast).is_ok());
}