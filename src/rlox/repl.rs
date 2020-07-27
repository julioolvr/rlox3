use crate::rlox::compiler::compile;
use crate::rlox::vm::InterpretError;
use std::io;
use std::io::prelude::*;

pub fn repl() -> Result<(), InterpretError> {
    println!("Welcome to the rlox prompt");
    println!("^D to exit\n");

    loop {
        print!("> ");
        io::stdout().flush().expect("Error flushing stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error: unable to read user input");

        interpret(&input)?;
    }
}

fn interpret(code: &String) -> Result<(), InterpretError> {
    compile(code);

    // let mut chunk = Chunk::new();
    // let constant_index = chunk.add_constant(124.0);
    // chunk.add_instruction(Instruction::OpConstant(constant_index), 123);
    // chunk.add_instruction(Instruction::OpNegate, 123);
    // let second_constant_index = chunk.add_constant(13.5);
    // chunk.add_instruction(Instruction::OpConstant(second_constant_index), 123);
    // chunk.add_instruction(Instruction::OpMultiply, 123);
    // chunk.add_instruction(Instruction::OpReturn, 123);
    // disassemble(&chunk, "test chunk");

    // println!("\nRunning vm...");
    // let mut vm = Vm::new();
    // vm.interpret(&chunk).unwrap();

    Ok(())
}
