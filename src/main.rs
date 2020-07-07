use rlox3::{disassemble, Chunk, Instruction, Vm};

fn main() {
    let mut chunk = Chunk::new();
    let constant_index = chunk.add_constant(124.0);
    chunk.add_instruction(Instruction::OpConstant(constant_index), 123);
    chunk.add_instruction(Instruction::OpNegate, 123);
    chunk.add_instruction(Instruction::OpReturn, 123);
    disassemble(&chunk, "test chunk");

    println!("\nRunning vm...");
    let mut vm = Vm::new();
    vm.interpret(&chunk).unwrap();
}
