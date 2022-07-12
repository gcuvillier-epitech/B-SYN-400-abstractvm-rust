use substring::Substring;
use std::fmt::{Debug, Display, Formatter, Result};

use crate::opcode;
use crate::value;

pub struct Instruction {
    code: opcode::OpCode,
    value: value::Value,
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
            let op = opcode::parse_op_code(s.substring(0, a).trim());
            if opcode::op_code_needs_value(op) {
                let val = value::parse_value(s.substring(a, s.chars().count()).trim());
                Instruction{ code: op , value: val }
            } else {
                Instruction { code: op , value: value::Value::Int8(0) }
            }
        }
        _ => panic!("Syntax error")
    }
}
