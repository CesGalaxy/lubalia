pub mod save_value;
pub mod scope;

use lubalang::syntax::node::Node;
use luvam::instruction::Instruction;
use save_value::compile_save_value;
use scope::Scope;

pub fn compile(ast: Vec<Node>) -> Vec<Instruction> {
    let mut program = Vec::new();

    let mut global = Scope::new(0);

    // for node in ast {
    //     match node {
    //         ASTRootItem::Const(node) => {
    //             program.extend(compile_save_value(node, &mut global));
    //         },
    //         _ => todo!(),
    //     }
    // }

    program
}
