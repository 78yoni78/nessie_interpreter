use std::vec::Vec;

#[repr(u8)]
#[derive(Debug)]
pub enum OpCode {
    Nop
}

pub struct Chunk{ 
    pub code: Vec<u8> 
}
