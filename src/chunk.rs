use std::vec::Vec;
use crate::value::*;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum OpCode {
    Nop,
    Constant,
}

impl OpCode {
    pub fn inst_size(self) -> u8 {
        match self {
            OpCode::Nop => 1,
            OpCode::Constant => 3,
        }
    }
}

#[repr(C, u8)]
#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Nop,
    Constant(u16), 
}

impl Instruction {
    fn opcode<'a>(self) -> OpCode {
        unsafe { transmute_copy(&self) }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_constants() {
        let mut chunk = Chunk::new();
        let two = chunk.add_constant(2.0); 
        let six = chunk.add_constant(6.0); 

        assert_eq!(chunk.constants[two], 2.0);
        assert_eq!(chunk.constants[six], 6.0);
    }
}
