mod chunk;
mod debug;

use std::vec;
use chunk::*;

fn main() {
    let chunk = Chunk { code: vec![OpCode::Nop as u8] };
    debug::disassemble_chunk(&chunk, "test chunk");
}
