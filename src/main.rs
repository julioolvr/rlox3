use rlox3::{disassemble, Chunk, Instruction};

fn main() {
    let mut chunk = Chunk::new();
    chunk.add_instruction(Instruction::OpReturn);
    disassemble(&chunk, "test chunk");
}
