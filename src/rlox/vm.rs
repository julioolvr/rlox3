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
        self.run(&chunk)?;
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
                    let next_value = self
                        .stack
                        .last()
                        .expect("Tried to pop element of an empty stack");

                    // TODO: Print runtime error
                    match next_value {
                        Value::Number(number) => {
                            let result = -number;
                            self.stack.push(Value::Number(result));
                        }
                        _ => return Err(InterpretError::RuntimeError),
                    }
                }
                Some(Instruction::OpAdd)
                | Some(Instruction::OpSubtract)
                | Some(Instruction::OpMultiply)
                | Some(Instruction::OpDivide) => {
                    let b = self
                        .stack
                        .pop()
                        .expect("Tried to pop element off empty stack");
                    let a = self
                        .stack
                        .pop()
                        .expect("Tried to pop element off empty stack");

                    match (b, a) {
                        (Value::Number(b), Value::Number(a)) => {
                            let result = match instruction {
                                Some(Instruction::OpAdd) => a + b,
                                Some(Instruction::OpSubtract) => a - b,
                                Some(Instruction::OpDivide) => a / b,
                                Some(Instruction::OpMultiply) => a * b,
                                _ => unreachable!(),
                            };

                            self.stack.push(Value::Number(result));
                        }
                        _ => {
                            // TODO: Log runtime error
                            return Err(InterpretError::RuntimeError);
                        }
                    }
                }
                Some(Instruction::OpTrue) => self.stack.push(Value::Boolean(true)),
                Some(Instruction::OpFalse) => self.stack.push(Value::Boolean(false)),
                Some(Instruction::OpNil) => self.stack.push(Value::Nil),
                None => return Err(InterpretError::RuntimeError),
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum InterpretError {
    CompileError,
    RuntimeError,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_stack_after_binary_operation() {
        let mut vm = Vm::new();
        let mut chunk = Chunk::new();

        let constant_index = chunk.add_constant(Value::Number(2.0));
        chunk.add_instruction(Instruction::OpConstant(constant_index), 1);
        let constant_index = chunk.add_constant(Value::Number(3.0));
        chunk.add_instruction(Instruction::OpConstant(constant_index), 1);
        chunk.add_instruction(Instruction::OpAdd, 1);
        chunk.add_instruction(Instruction::OpReturn, 1);

        vm.interpret(&chunk).expect("Error running chunk");

        assert_eq!(vm.stack.len(), 0);
    }
}
