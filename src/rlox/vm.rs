use crate::rlox::chunk::Chunk;
use crate::rlox::disassemble_instruction;
use crate::rlox::instruction::Instruction;

pub struct Vm {
    ip: usize,
}

impl Vm {
    pub fn new() -> Vm {
        Vm { ip: 0 }
    }

    pub fn interpret(&mut self, chunk: &Chunk) -> Result<(), InterpretError> {
        self.ip = 0;
        self.run(chunk)?;
        Ok(())
    }

    fn run(&mut self, chunk: &Chunk) -> Result<(), InterpretError> {
        loop {
            let instruction = chunk.instruction_at(self.ip);
            self.ip += 1;

            if cfg!(debug_assertions) {
                disassemble_instruction(chunk, self.ip - 1);
            }

            match instruction {
                Some(Instruction::OpReturn) => {
                    return Ok(());
                }
                Some(Instruction::OpConstant(index)) => {
                    let value = chunk.constant_at(*index);
                    println!("Debug OpConstant: {}", value);
                }
                _ => return Err(InterpretError::RuntimeError),
            }
        }
    }
}

#[derive(Debug)]
pub enum InterpretError {
    CompileError,
    RuntimeError,
}
