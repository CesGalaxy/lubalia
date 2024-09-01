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

const STACK_DEFAULT_VALUE: DataValue = DataValue::Int(0);

impl<const CAPACITY: usize> LUVAM<CAPACITY> {
    pub fn new() -> Self {
        Self {
            accumulator: DataValue::Int(0),
            stack: [STACK_DEFAULT_VALUE; CAPACITY],
        }
    }

    pub fn eval(&mut self, instructions: Vec<Instruction>) {
        for instruction in instructions {
            instruction.run(self);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vm() {
        let mut vm = LUVAM::<2>::new();

        vm.accumulator = DataValue::Int(2);

        vm.eval(vec![
            Instruction::Set(0),
            Instruction::Set(1),
            Instruction::Add(0, 1),
        ]);

        assert_eq!(vm.accumulator, DataValue::Int(4));
    }
}
