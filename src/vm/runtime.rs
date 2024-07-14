use crate::lang::old_parser::node::{expression::ExpressionNode, statement::StatementNode, TreeNode};

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

        if let Some(node) = node {
            let node = node.clone();

            if let TreeNode::Expression(expr) = node {
                println!("#{} - {expr} -> {}", self.ip, expr.evaluate(&self.global));
            } else if let TreeNode::Statement(statement) = node {
                statement.run(&mut self.global);
            }
        }

        self.ip += 1;
    }
}