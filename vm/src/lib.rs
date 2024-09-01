use data::DataValue;
use instruction::Instruction;

pub mod instruction;
pub mod data;

pub struct LUVAM<const Collections: usize> {
    accumulator: DataValue,
    collections: [Vec<DataValue>; Collections],
}

impl LUVAM {
    pub fn new(reserved: [usize; Collections]) -> Self {
        Self {
            accumulator: DataValue::Int(0),
            collections: reserved.iter().map(|&size| Vec::with_capacity(size)).collect(),
        }
    }

    pub fn eval(&mut self, instructions: Vec<Instruction>) {
        for instruction in instructions {
            instruction.run(self);
        }
    }
}
