use super::{context::Context, VM};

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
        if let Some(context) = &mut self.context {
            context
        } else {
            &mut self.vm.global
        }
    }
}