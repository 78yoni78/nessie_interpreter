mod chunk;
mod debug;
mod value;

use std::vec;
use chunk::*;

fn main() {
    let chunk = Chunk { code: vec![OpCode::Nop as u8], constants: vec![] };
    debug::disassemble_chunk(&chunk, "test chunk");
}
