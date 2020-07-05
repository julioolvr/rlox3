use super::chunk::Chunk;
use super::instruction::Instruction;

pub fn disassemble(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    // For now, the chunk holds a vec of instructions. Instructions are an
    // enum, meaning that the size of each value is the size of the largest
    // enum variant. This isn't as optimal as the representation in the book,
    // but it should be possible to eventually change the representation and
    // implement iteration in the chunk in a way that returns instructions.
    // It also has the side effect that for now there's no real "offset" other
    // than the index, so the disaseembly here will be a bit of a lie.
    for (i, instruction) in chunk.instructions().enumerate() {
        print!("{:0>4} ", i);

        match instruction {
            Instruction::OpReturn => println!("OpReturn"),
            _ => println!("Unknown opcode {:?}", instruction),
        }
    }
}
