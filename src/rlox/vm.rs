use crate::rlox::chunk::Chunk;
use crate::rlox::disassemble_instruction;
use crate::rlox::instruction::Instruction;
use crate::rlox::value::Value;

pub struct Vm<'a> {
    ip: usize,
    stack: Vec<&'a Value>,
}

impl<'a> Vm<'a> {
    pub fn new() -> Vm<'a> {
        Vm {
            ip: 0,
            stack: vec![],
        }
    }

    pub fn interpret(&mut self, chunk: &'a Chunk) -> Result<(), InterpretError> {
        self.ip = 0;
        self.run(chunk)?;
        Ok(())
    }

    fn run(&mut self, chunk: &'a Chunk) -> Result<(), InterpretError> {
        loop {
            let instruction = chunk.instruction_at(self.ip);
            self.ip += 1;

            if cfg!(debug_assertions) {
                print!("          ");

                for value in self.stack.iter() {
                    print!("[ {:?} ]", value);
                }

                println!("");

                disassemble_instruction(chunk, self.ip - 1);
            }

            match instruction {
                Some(Instruction::OpReturn) => {
                    let value = self
                        .stack
                        .pop()
                        .expect("Tried to pop element off empty stack");
                    println!("Returning {:?}", value);
                    return Ok(());
                }
                Some(Instruction::OpConstant(index)) => {
                    let value = chunk.constant_at(*index);
                    self.stack.push(value);
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
