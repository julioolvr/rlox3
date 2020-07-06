use super::instruction::Instruction;
use super::value::Value;

pub struct Chunk {
    instructions: Vec<Instruction>,
    constants: Vec<Value>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            instructions: vec![],
            constants: vec![],
        }
    }

    pub fn instructions(&self) -> impl Iterator<Item = &Instruction> {
        self.instructions.iter()
    }

    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    pub fn add_constant(&mut self, constant: Value) -> usize {
        self.constants.push(constant);
        self.constants.len() - 1
    }

    pub fn constant_at(&self, index: usize) -> &Value {
        self.constants
            .get(index)
            .expect("Tried to get missing constant")
    }
}
