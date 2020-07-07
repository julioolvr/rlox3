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
    for i in 0..chunk.instructions_count() {
        disassemble_instruction(chunk, i);
    }
}

pub fn disassemble_instruction(chunk: &Chunk, index: usize) {
    let instruction = chunk
        .instruction_at(index)
        .expect("Tried to get instruction at wrong index");

    let last_line = if index >= 1 {
        Some(chunk.line_at(index - 1))
    } else {
        None
    };

    let line = chunk.line_at(index);

    print!("{:0>4} ", index);

    if last_line.map_or(false, |actual_last_line| actual_last_line == line) {
        print!("   | ");
    } else {
        print!("{: >4} ", line);
    }

    match instruction {
        Instruction::OpReturn => println!("OpReturn"),
        Instruction::OpConstant(index) => {
            print!("{: <16}", "OpConstant");
            print!("{: >4}", index);
            println!(" {:?}", chunk.constant_at(*index));
        }
    }
}
