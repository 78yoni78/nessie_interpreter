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

impl Chunk {
    /// Creates a new empty chunk with no constants 
    pub fn new() -> Self {
        Chunk { 
            code: vec![],
            constants: vec![],
        }
    }
    
    /// Adds a new constant to the chunk and returns its index
    pub fn add_constant(& mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }
}