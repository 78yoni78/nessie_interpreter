use std::vec::Vec;
use std::mem::{transmute_copy, transmute};

use crate::value::*;


#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum OpCode {
    Nop,
    Constant,
}

#[repr(C, u8)]
#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Nop,
    Constant(u16), 
}

pub struct Chunk{ 
    pub code: Vec<u8>, 
    pub constants: Vec<Value>,
}


impl OpCode {
    pub fn inst_size(self) -> usize {
        match self {
            OpCode::Nop => 1,
            OpCode::Constant => 3,
        }
    }
}

impl Instruction {
    fn opcode<'a>(self) -> OpCode {
        unsafe { transmute_copy(&self) }
    }
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

    /// Adds a new instruction to the bytecode
    pub fn add_instruction(&mut self, inst: Instruction) {
        let size = inst.opcode().inst_size();
        unsafe {
            let inst: &u8 = transmute(&inst);
            let ptr: *const u8 = inst as *const u8; 
            let slice = std::slice::from_raw_parts(ptr, size as usize);
            self.code.extend_from_slice(slice);
        }
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

    #[test]
    fn adds_instructions() {
        let mut chunk = Chunk::new();

        let i = chunk.add_constant(12.0) as u16;
        chunk.add_instruction(Instruction::Nop); 
        chunk.add_instruction(Instruction::Constant(i)); 

        assert_eq!(chunk.code.len(), 4);
        assert_eq!(chunk.code[0], OpCode::Nop as u8);
        assert_eq!(chunk.code[1], OpCode::Constant as u8);
        assert_eq!(
            unsafe { *(&chunk.code[2] as *const u8 as *const u16) },
            i
        );
    }
}
