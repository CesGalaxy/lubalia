use super::{context::Context, VM};

/// A tick is the execution of a single instruction/node in the VM
pub struct VMTick<'a> {
    /// The VM running the tick
    pub vm: &'a mut VM,

    /// The smallest on which the tick is run
    pub context: Option<Box<Context>>,
}

impl VMTick<'_> {
    /// Gets the current context used ing the VM,
    /// if there's no custom context it returns the global
    pub fn get_context(&mut self) -> &mut Context {
        self.context.as_mut().map(|c| c.as_mut()).unwrap_or(&mut self.vm.global)
    }
}