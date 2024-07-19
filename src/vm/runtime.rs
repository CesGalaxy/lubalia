use crate::lang::parser::root::ASTRootItem;

use super::VM;

impl VM {
    /// Start running the emulation
    pub fn run(&mut self) {
        while self.ip < self.program.len() {
            // Tick while the program is not finished
            self.tick();
        }
    }

    /// Each tick corresponds to the execution of a single instruction (represented by a root-node)
    pub fn tick(&mut self) {
        if let Some(ASTRootItem::Node(node)) = self.program.get(self.ip) {
            println!("{:?}", node.execute(&mut self.global));
        }

        self.ip += 1;
    }
}