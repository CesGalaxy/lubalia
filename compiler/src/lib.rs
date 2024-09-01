pub mod save_value;

use lubalang::syntax::root::ASTRootItem;
use luvam::instruction::Instruction;
use save_value::compile_save_value;

pub fn compile(ast: Vec<ASTRootItem>) -> Vec<Instruction> {
    let mut program = Vec::new();

    for node in ast {
        match node {
            ASTRootItem::Const(node) => {
                program.extend(compile_save_value(node));
            },
            _ => todo!(),
        }
    }

    program
}
