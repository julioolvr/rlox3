use super::instruction::Instruction;

pub struct Chunk {
    instructions: Vec<Instruction>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            instructions: vec![],
        }
    }

    pub fn instructions(&self) -> impl Iterator<Item = &Instruction> {
        self.instructions.iter()
    }

    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }
}
