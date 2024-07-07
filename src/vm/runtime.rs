use crate::lang::parser::node::{expression::ExpressionNode, TreeNode};

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
        let node = self.program.get(self.ip);

        if let Some(TreeNode::Expression(expr)) = node {
            println!("#{} - {:?} - {:?}", self.ip, expr, expr.evaluate(&self.global));
        }

        self.ip += 1;
    }
}