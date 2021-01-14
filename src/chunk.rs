use std::{usize, vec::Vec};
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

struct Line { first_byte_offset: usize }

pub struct Chunk{ 
    code: Vec<u8>, 
    constants: Vec<Value>,
    lines: Vec<Line>,
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
        let first_line = Line { first_byte_offset: 0 };
        Chunk { 
            code: Vec::new(),
            constants: Vec::new(),
            lines: vec![first_line],
        }
    }

    pub fn code(&self) -> &Vec<u8> { &self.code }
    pub fn code_mut(&mut self) -> &mut Vec<u8> { &mut self.code }
    
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

    /// Returns the number of the line the instruction at the offset was in in the original source code (starting at 1)
    pub fn get_line_number(&self, instruction_offset: usize) -> usize {
        let mut line_number = 0;
        while line_number < self.lines.len() && self.lines[line_number].first_byte_offset <= instruction_offset {
            line_number = line_number + 1;
        }
        line_number
    }

    /// Call once a new line is reached
    pub fn new_line(&mut self) {
        self.lines.push(Line { first_byte_offset: self.code.len() })
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
