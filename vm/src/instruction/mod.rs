use crate::LUVAM;

pub enum Instruction {
    ColInit(usize),
    ColClear(usize),
    ColPush(usize),
    ColPop(usize),
    ColMoveLast(usize),
    ColDestroy(usize),
    ColGet(usize, usize),
    ColUpdate(usize, usize),
    ColRemove(usize, usize),
    ColMove(usize, usize),
    ColSwap(usize, usize),

    Add(usize, usize),
    Sub(usize, usize),
    Mul(usize, usize),
    Div(usize, usize),
    Quo(usize, usize),
}

impl Instruction {
    pub fn run(vm: &mut LUVAM) {
        todo!()
    }
}
