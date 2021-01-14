use crate::chunk::*;

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    let mut offset = 0;
    while offset < chunk.code.len() {
        disassemble_instruction(chunk, &mut offset);
    }
}

fn disassemble_instruction(chunk: &Chunk, offset: &mut usize) {
    let opcode: OpCode = unsafe { ::std::mem::transmute(chunk.code[*offset]) };
    let instruction: &Instruction = unsafe { ::std::mem::transmute(&chunk.code[*offset]) };
    *offset = *offset + opcode.inst_size();

    //  we must be looking at an instruction inside of chunk.code
    assert!(*offset < chunk.code.len());
    
    println!("{:0<4} {:?}", offset, *instruction);
}