use crate::chunk::*;

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    let mut offset = 0;
    while offset < chunk.code.len() {
        offset = disassemble_instruction(chunk, offset);
    }
}

fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    let opcode: OpCode = unsafe { ::std::mem::transmute(chunk.code[offset]) };
    println!("{:0<4} {:?}", offset, opcode);
    offset + 1
}