#[derive(Debug, PartialEq)]
pub enum Instruction {
    OpReturn,
    OpConstant(usize),
    OpNegate,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
}
