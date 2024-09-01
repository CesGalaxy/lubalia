use lubalang::syntax::node::const_declaration::ConstDeclarationNode;
use luvam::{data::DataValue, instruction::Instruction};

pub fn compile_save_value(node: ConstDeclarationNode) -> Vec<Instruction> {
    vec![
        Instruction::Load(DataValue::Int(node.value)),
        Instruction::Set(0)
    ]
}