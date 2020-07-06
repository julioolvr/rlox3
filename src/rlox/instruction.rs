#[derive(Debug)]
pub enum Instruction {
    OpReturn,
    OpConstant(usize),
}

