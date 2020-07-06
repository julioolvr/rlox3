use rlox3::{disassemble, Chunk, Instruction};

fn main() {
    let mut chunk = Chunk::new();
    let constant_index = chunk.add_constant(124);
    chunk.add_instruction(Instruction::OpConstant(constant_index));
    chunk.add_instruction(Instruction::OpReturn);
    disassemble(&chunk, "test chunk");
}
