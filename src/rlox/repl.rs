use crate::rlox::chunk::Chunk;
use crate::rlox::compiler::compile;
use crate::rlox::vm::InterpretError;
use crate::rlox::vm::Vm;
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
    let mut chunk = Chunk::new();
    compile(code, &mut chunk)?;

    // TODO: Persist the VM across the REPL session
    let mut vm = Vm::new();
    vm.interpret(&chunk).unwrap();

    Ok(())
}
