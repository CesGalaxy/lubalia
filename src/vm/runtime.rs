use crate::lang::parser::node::{expression::ExpressionNode, TreeNode};

use super::VM;

impl VM {
    pub fn run(&mut self) {
        while self.ip < self.program.len() {
            self.tick()
        }
    }

    pub fn tick(&mut self) {
        let node = self.program.get(self.ip);

        if let Some(TreeNode::Expression(expr)) = node {
            println!("#{} - {:?} - {:?}", self.ip, expr, expr.evaluate(&self.global));
        }

        self.ip += 1;
    }
}