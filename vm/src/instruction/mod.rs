use crate::LUVAM;

pub enum Instruction {
    Push,
    Pop,
    Get(usize),
    Set(usize),
    Swap(usize, usize),

    Add(usize, usize),
    Sub(usize, usize),
    Mul(usize, usize),
    Div(usize, usize),
    Quo(usize, usize),
}

impl Instruction {
    pub fn run(&self, vm: &mut LUVAM) {
        match self {
            Instruction::Push => {
                vm.stack.push(vm.accumulator);
            },
            Instruction::Pop => {
                vm.accumulator = vm.stack.pop().unwrap();
            },
            Instruction::Get(index) => {
                vm.accumulator = vm.stack[*index];
            },
            Instruction::Set(index) => {
                vm.stack[*index] = vm.accumulator;
            },
            Instruction::Swap(index1, index2) => {
                vm.stack.swap(*index1, *index2);
            },
            Instruction::Add(index1, index2) => {
                vm.accumulator = vm.stack[*index1] + vm.stack[*index2];
            },
            Instruction::Sub(index1, index2) => {
                vm.accumulator = vm.stack[*index1] - vm.stack[*index2];
            },
            Instruction::Mul(index1, index2) => {
                vm.accumulator = vm.stack[*index1] * vm.stack[*index2];
            },
            Instruction::Div(index1, index2) => {
                vm.accumulator = vm.stack[*index1] / vm.stack[*index2];
            },
            Instruction::Quo(index1, index2) => {
                vm.accumulator = vm.stack[*index1] % vm.stack[*index2];
            },
        }
    }
}
