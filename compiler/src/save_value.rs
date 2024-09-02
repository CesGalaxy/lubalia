use lubalang::syntax::node::const_declaration::ConstDeclaration;
use luvam::{data::DataValue, instruction::Instruction};

use crate::scope::Scope;

pub fn compile_save_value(node: ConstDeclaration, scope: &mut Scope) -> Vec<Instruction> {
    let instructions = vec![
        Instruction::Load(DataValue::Int(node.value)),
        Instruction::Set(scope.indent)
    ];

    scope.indent += 1;

    instructions
}