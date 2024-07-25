pub mod context;

use context::Context;

use crate::{data::DataValue, node::ASTNode, root::ASTRootItem};

/// A virtual machine that executes a program in bytecode
#[derive(Debug)]
pub struct VM {
    /// The global scope (context) of the VM
    pub global: Context,
    /// The last value returned by an expression (_)
    pub last_value: DataValue
}

impl VM {
    /// Create a new VM with a program
    pub fn new() -> Self {
        VM {
            global: Context::new(),
            last_value: DataValue::default()
        }
    }

    /// Start running the emulation
    pub fn evaluate(&mut self, program: Vec<ASTRootItem>) {
        // ip stands for instruction pointer
        let mut ip = 0;

        while ip < program.len() {
            // Tick while the program is not finished
            if let Some(ASTRootItem::Node(node)) = program.get(ip).cloned() {
                self.tick(node);
            }

            ip += 1;
        }
    }

    /// Each tick corresponds to the execution of a single instruction/node.
    pub fn tick(&mut self, node: ASTNode) {
        let mut tick = VMTick {
            vm: self,
            context: None
        };

        println!("TICK =======================================");

        if let Some(value) = node.execute(&mut tick) {
            println!("NODE: {node}");
            println!("{} => {value}", if let ASTNode::Statement(_) = node { "S" } else { "E" });
        }
    }
}

pub struct VMTick<'a> {
    pub vm: &'a mut VM,
    pub context: Option<Box<Context>>,
}

impl VMTick<'_> {
    pub fn get_context(&mut self) -> &mut Context {
        if let Some(context) = &mut self.context {
            context
        } else {
            &mut self.vm.global
        }
    }

    // pub fn tmp_context(&'a mut self) -> VMTick<'a> {
    //     VMTick {
    //         vm: self.vm,
    //         context: Some(Context::with_parent(self.context.clone()))
    //     }
    // }
}