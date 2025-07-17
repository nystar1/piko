use wasm_bindgen::prelude::*;
use piko_core::ast::traits::Parseable;
use piko_core::ast::PikoAst;
use piko_core::vm::VM;

const EXAMPLES: &[(&str, &str)] = &[
    ("hello", include_str!("../../examples/hello.pyx")),
    ("variables", include_str!("../../examples/variables.pyx")),
    ("math", include_str!("../../examples/math.pyx")),
    ("functions", include_str!("../../examples/functions.pyx")),
    ("loops", include_str!("../../examples/loops.pyx")),
    ("input", include_str!("../../examples/input.pyx")),
    ("chains", include_str!("../../examples/chains.pyx")),
];

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn prompt(msg: &str) -> Option<String>;
}

pub struct WebInput {
    buffer: String,
}

impl WebInput {
    pub fn new() -> Self {
        WebInput {
            buffer: String::new(),
        }
    }
}

impl std::io::BufRead for WebInput {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        Ok(self.buffer.as_bytes())
    }
    
    fn consume(&mut self, amt: usize) {
        self.buffer.drain(..amt);
    }
    
    fn read_line(&mut self, buf: &mut String) -> std::io::Result<usize> {
        let input = prompt("Enter input:").unwrap_or_default();
        buf.push_str(&input);
        buf.push('\n');
        Ok(input.len() + 1)
    }
}

impl std::io::Read for WebInput {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let input = prompt("Enter input:").unwrap_or_default();
        let input_bytes = input.as_bytes();
        let len = input_bytes.len().min(buf.len());
        buf[..len].copy_from_slice(&input_bytes[..len]);
        Ok(len)
    }
}

#[wasm_bindgen]
pub struct PikoVM {
    vm: VM<Vec<u8>, WebInput>,
}

#[wasm_bindgen]
impl PikoVM {
    #[wasm_bindgen(constructor)]
    pub fn new() -> PikoVM {
        let output = Vec::new();
        let input = WebInput::new();
        PikoVM {
            vm: VM::new(output, input),
        }
    }
    
    #[wasm_bindgen]
    pub fn execute(&mut self, code: &str) -> Result<(), JsValue> {
        let statements = self.split_statements(code);
        
        for statement in statements {
            let statement = statement.trim();
            if statement.is_empty() || statement.starts_with('#') {
                continue;
            }
            
            match PikoAst::parse(statement) {
                Ok(ast) => {
                    self.vm.execute(ast).map_err(|e| JsValue::from_str(&e.to_string()))?;
                }
                Err(e) => return Err(JsValue::from_str(&e.to_string())),
            }
        }
        Ok(())
    }
    
    fn split_statements(&self, code: &str) -> Vec<String> {
        let mut statements = Vec::new();
        let mut current_statement = String::new();
        let mut paren_count = 0;
        let mut in_string = false;
        let mut escape_next = false;
        
        for ch in code.chars() {
            if escape_next {
                current_statement.push(ch);
                escape_next = false;
                continue;
            }
            
            match ch {
                '\\' if in_string => {
                    escape_next = true;
                    current_statement.push(ch);
                }
                '"' => {
                    in_string = !in_string;
                    current_statement.push(ch);
                }
                '(' if !in_string => {
                    paren_count += 1;
                    current_statement.push(ch);
                }
                ')' if !in_string => {
                    paren_count -= 1;
                    current_statement.push(ch);
                    
                    if paren_count == 0 && !current_statement.trim().is_empty() {
                        statements.push(current_statement.trim().to_string());
                        current_statement.clear();
                    }
                }
                '\n' if paren_count == 0 => {
                    if !current_statement.trim().is_empty() {
                        statements.push(current_statement.trim().to_string());
                        current_statement.clear();
                    }
                }
                _ => {
                    current_statement.push(ch);
                }
            }
        }
        
        if !current_statement.trim().is_empty() {
            statements.push(current_statement.trim().to_string());
        }
        
        statements
    }
    
    #[wasm_bindgen]
    pub fn get_output(&mut self) -> String {
        let output = self.vm.get_output();
        let result = String::from_utf8_lossy(output).to_string();
        output.clear();
        result
    }
}

#[wasm_bindgen]
pub fn get_example(name: &str) -> Option<String> {
    EXAMPLES.iter()
        .find(|(example_name, _)| *example_name == name)
        .map(|(_, content)| content.to_string())
}

#[wasm_bindgen]
pub fn get_example_names() -> Vec<String> {
    EXAMPLES.iter()
        .map(|(name, _)| name.to_string())
        .collect()
}

#[wasm_bindgen(start)]
pub fn main() {
    web_sys::console::log_1(&"Piko WASM loaded".into());
}