use data::DataValue;
use instruction::Instruction;

pub mod instruction;
pub mod data;

pub struct LUVAM<const CAPACITY: usize> {
    /// The accumulator of the VM
    accumulator: DataValue,

    /// The stack of the VM
    stack: [DataValue; CAPACITY],
}

impl LUVAM {
    pub fn new(stack_size: usize) -> Self {
        Self {
            accumulator: DataValue::Int(0),
            // TODO: Is this even secure?
            stack: Vec::with_capacity(stack_size),
        }
    }

    pub fn eval(&mut self, instructions: Vec<Instruction>) {
        for instruction in instructions {
            instruction.run(self);
        }
    }
}
