use std::fmt::{Debug, Display, Formatter, Result};

use crate::opcode::{op_code_needs_value, OpCode, parse_op_code};
use crate::value::{parse_value, Value};

pub struct Instruction {
    pub code: OpCode,
    pub value: Option<Value>,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?} {:?}", self.code, self.value)
    }
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self)
    }
}

pub fn parse_instruction(s: &str) -> Instruction {
    match s.find(' ') {
        Some(a) => {
            let op = parse_op_code(s[..a].trim());
            if op_code_needs_value(op) {
                let val = parse_value(s[a + 1..].trim());
                Instruction { code: op, value: Some(val) }
            } else {
                Instruction { code: op, value: None }
            }
        }
        _ => Instruction { code: parse_op_code(s.trim()), value: None }
    }
}
