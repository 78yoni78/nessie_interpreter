use std::vec::Vec;
use crate::value::*;

#[repr(u8)]
#[derive(Debug)]
pub enum OpCode {
    Nop
}

pub struct Chunk{ 
    pub code: Vec<u8>, 
    pub constants: Vec<Value>,
}
