use crate::rlox::chunk::Chunk;
use crate::rlox::disassembler::disassemble_instruction;
use crate::rlox::instruction::Instruction;
use crate::rlox::value::Value;

pub struct Vm {
    ip: usize,
    stack: Vec<Value>,
}

impl Vm {
    pub fn new() -> Vm {
        Vm {
            ip: 0,
            stack: vec![],
        }
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
                    // At this point, the stack in the book holds elements of
                    // type Value, which is (for now) an alias for C's double.
                    // Afaik, that means that values get copied when they're
                    // pushed in the stack. I assume eventually we'll handle
                    // primitives and non-primitives or something like that,
                    // but for now we'll just clone.
                    self.stack.push(value.clone());
                }
                Some(Instruction::OpNegate) => {
                    let value = self
                        .stack
                        .pop()
                        .expect("Tried to pop element off empty stack");
                    self.stack.push(-value);
                }
                Some(Instruction::OpAdd) => {
                    let b = self
                        .stack
                        .pop()
                        .expect("Tried to pop element off empty stack");
                    let a = self
                        .stack
                        .pop()
                        .expect("Tried to pop element off empty stack");
                    self.stack.push(a + b);
                }
                Some(Instruction::OpSubtract) => {
                    let b = self
                        .stack
                        .pop()
                        .expect("Tried to pop element off empty stack");
                    let a = self
                        .stack
                        .pop()
                        .expect("Tried to pop element off empty stack");
                    self.stack.push(a - b);
                }
                Some(Instruction::OpMultiply) => {
                    let b = self
                        .stack
                        .pop()
                        .expect("Tried to pop element off empty stack");
                    let a = self
                        .stack
                        .pop()
                        .expect("Tried to pop element off empty stack");
                    self.stack.push(a * b);
                }
                Some(Instruction::OpDivide) => {
                    let b = self
                        .stack
                        .pop()
                        .expect("Tried to pop element off empty stack");
                    let a = self
                        .stack
                        .pop()
                        .expect("Tried to pop element off empty stack");
                    self.stack.push(a / b);
                }
                None => return Err(InterpretError::RuntimeError),
            }
        }
    }
}

#[derive(Debug)]
pub enum InterpretError {
    CompileError,
    RuntimeError,
}
