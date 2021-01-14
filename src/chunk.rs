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
    /// Adds a new constant to the chunk and returns a reference to it
    pub fn add_constant(&mut self, value: Value) -> &Value {
        self.constants.push(value);
        &self.constants.last().unwrap()
    }
}
