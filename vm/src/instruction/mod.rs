use crate::LUVAM;

#[derive(Debug)]
pub enum Instruction {
    Get(usize),
    Set(usize),
    Swap(usize, usize),

    Add(usize, usize),
    // Sub(usize, usize),
    // Mul(usize, usize),
    // Div(usize, usize),
    // Quo(usize, usize),
}

impl Instruction {
    pub fn run<const CAPACITY: usize>(&self, vm: &mut LUVAM<CAPACITY>) {
        match self {
            Instruction::Get(index) => {
                vm.accumulator = vm.stack[*index].clone();
            },
            Instruction::Set(index) => {
                vm.stack[*index] = vm.accumulator.clone();
            },
            Instruction::Swap(index1, index2) => {
                let temp = vm.stack[*index1].clone();
                vm.stack[*index1] = vm.stack[*index2].clone();
                vm.stack[*index2] = temp;
            },
            Instruction::Add(index1, index2) => {
                vm.accumulator = &vm.stack[*index1] + &vm.stack[*index2];
            },
            // Instruction::Sub(index1, index2) => {
            //     vm.accumulator = vm.stack[*index1] - vm.stack[*index2];
            // },
            // Instruction::Mul(index1, index2) => {
            //     vm.accumulator = vm.stack[*index1] * vm.stack[*index2];
            // },
            // Instruction::Div(index1, index2) => {
            //     vm.accumulator = vm.stack[*index1] / vm.stack[*index2];
            // },
            // Instruction::Quo(index1, index2) => {
            //     vm.accumulator = vm.stack[*index1] % vm.stack[*index2];
            // },
        }
    }
}
