use crate::common::{dissasemble_error, runtime_error};
use crate::value::Value;

#[repr(u8)]
#[derive(Debug)]
pub enum OpCode {
    OpReturn = 0,
    OpConstant = 1,
    OpNegate = 2,
    OpAdd = 3,
    OpSubtract = 4,
    OpMultiply = 5,
    OpDivide = 6,
    OpNil = 7,
    OpTrue = 8,
    OpFalse = 9,
    OpNot = 10,
    OpEqual = 11,
    OpGreater = 12,
    OpLess = 13,
}

pub fn byte_to_op(byte: u8) -> Result<OpCode, String> {
    match byte {
        0 => return Ok(OpCode::OpReturn),
        1 => return Ok(OpCode::OpConstant),
        2 => return Ok(OpCode::OpNegate),
        3 => return Ok(OpCode::OpAdd),
        4 => return Ok(OpCode::OpSubtract),
        5 => return Ok(OpCode::OpMultiply),
        6 => return Ok(OpCode::OpDivide),
        7 => return Ok(OpCode::OpNil),
        8 => return Ok(OpCode::OpTrue),
        9 => return Ok(OpCode::OpFalse),
        10 => return Ok(OpCode::OpNot),
        11 => return Ok(OpCode::OpEqual),
        12 => return Ok(OpCode::OpGreater),
        13 => return Ok(OpCode::OpLess),
        _ => {
            return Err(runtime_error(format!(
                "Invalid conversion to instruction from byte: '{}'\nInstruction doesn't exist.",
                byte
            )))
        }
    };
}

#[derive(Debug, Clone)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: Vec<Value>,
    pub lines: Vec<i32>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: vec![],
            constants: vec![],
            lines: vec![],
        }
    }

    pub fn write_instruction(&mut self, instruction: OpCode, line: i32) {
        self.lines.push(line);
        self.code.push(instruction as u8);
    }

    pub fn write_byte(&mut self, byte: u8, line: i32) {
        self.lines.push(line);
        self.code.push(byte);
    }

    pub fn add_constant(&mut self, constant: Value) -> u8 {
        self.constants.push(constant);
        return self.constants.len() as u8 - 1;
    }

    pub fn dissasemble(&self, name: &str) -> Result<(), String> {
        println!("== {} ==", name);

        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.dissasemble_instruction(offset)?;
        }

        Ok(())
    }

    pub fn dissasemble_instruction(&self, offset: usize) -> Result<usize, String> {
        print!("{:04} ", offset);
        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("   | ");
        } else {
            print!("{:4} ", self.lines[offset]);
        }

        if let Some(byte) = self.code.get(offset) {
            let instruction = byte_to_op(*byte)?;

            match instruction {
                OpCode::OpReturn => {
                    return Ok(self.simple_instruction("OP_RETURN", offset));
                }
                OpCode::OpConstant => {
                    return Ok(self.constant_instruction("OP_CONSTANT", offset));
                }
                OpCode::OpNegate => {
                    return Ok(self.simple_instruction("OP_NEGATE", offset));
                }
                OpCode::OpAdd => {
                    return Ok(self.simple_instruction("OP_ADD", offset));
                }
                OpCode::OpSubtract => {
                    return Ok(self.simple_instruction("OP_SUBTRACT", offset));
                }
                OpCode::OpMultiply => {
                    return Ok(self.simple_instruction("OP_MULTIPLY", offset));
                }
                OpCode::OpDivide => {
                    return Ok(self.simple_instruction("OP_DIVIDE", offset));
                }
                OpCode::OpEqual => {
                    return Ok(self.simple_instruction("OP_EQUAL", offset));
                }
                OpCode::OpGreater => {
                    return Ok(self.simple_instruction("OP_GREATER", offset));
                }
                OpCode::OpLess => {
                    return Ok(self.simple_instruction("OP_LESS", offset));
                }
                OpCode::OpNil => return Ok(self.simple_instruction("OP_NIL", offset)),
                OpCode::OpTrue => return Ok(self.simple_instruction("OP_TRUE", offset)),
                OpCode::OpFalse => return Ok(self.simple_instruction("OP_FALSE", offset)),
                OpCode::OpNot => return Ok(self.simple_instruction("OP_NOT", offset)),
                _ => {
                    return Err(dissasemble_error(format!(
                        "Unknown instruction found: '{:?}'\nDissasembling not implemented.",
                        instruction
                    )));
                }
            }
        } else {
            return Err(dissasemble_error(format!(
                "Invalid instruction found at offset: '{}'\nOffset out of bounds.",
                offset
            )));
        }
    }

    fn simple_instruction(&self, name: &str, offset: usize) -> usize {
        println!("{}", name);
        return offset + 1;
    }

    fn constant_instruction(&self, name: &str, offset: usize) -> usize {
        let constant = self.code[offset + 1];
        print!("{:16} {:04} '", name, constant);
        self.constants[constant as usize].print();
        println!("'");
        return offset + 2;
    }
}
