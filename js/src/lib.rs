pub mod utils;

use lubengine::{lang::{lexer::lexer, parser::parser}, vm::VM};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn is_lubalia_in_da_house() -> bool {
    true
}

#[wasm_bindgen]
pub fn luval(code: String) -> Option<String> {
    // Get the tokens from the source code
    let tokens = lexer(code).ok()?;

    // Parse the tokens into an AST
    let tree = parser(tokens).ok()?;

    // Get all the root-nodes from the AST
    let program: Vec<_> = tree.units().into_iter().cloned().collect();

    let mut vm = VM::new();

    let result = vm.evaluate(program);

    result.map(|value| value.to_string())
}
