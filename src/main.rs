mod chunk;
mod debug;
mod value;

use chunk::*;

fn main() {
    let mut chunk = Chunk::new();

    let six = chunk.add_constant(6.0);
    chunk.add_instruction(Instruction::Constant(six as u16));
    chunk.add_instruction(Instruction::Nop);
    
    debug::disassemble_chunk(&chunk, "test chunk");
}
