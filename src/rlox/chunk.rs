use super::instruction::Instruction;
use super::value::Value;

pub struct Chunk {
    instructions: Vec<Instruction>,
    constants: Vec<Value>,
    lines: Vec<usize>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            instructions: vec![],
            constants: vec![],
            lines: vec![],
        }
    }

    pub fn instructions(&self) -> impl Iterator<Item = &Instruction> {
        self.instructions.iter()
    }

    pub fn instruction_at(&self, index: usize) -> Option<&Instruction> {
        self.instructions.get(index)
    }

    pub fn instructions_count(&self) -> usize {
        self.instructions.len()
    }

    pub fn add_instruction(&mut self, instruction: Instruction, line: usize) {
        self.instructions.push(instruction);
        self.lines.push(line);
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

    pub fn line_at(&self, index: usize) -> &usize {
        self.lines
            .get(index)
            .expect("Tried to get missing line number")
    }
}
