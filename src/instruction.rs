use std::fmt::{Debug, Display, Formatter, Result};

use crate::opcode::OpCode;
use crate::value::Value;

// Unfortunately Value can't be Copy-able because BigDecimal is not.
// Let's enable Clone anyway
#[derive(Clone)]
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

impl Instruction {
    pub fn parse(s: &str) -> Instruction {
        match s.find(' ') {
            Some(a) => {
                let op = OpCode::parse(s[..a].trim());
                if op.needs_value() {
                    let val = Value::parse(s[a + 1..].trim());
                    Instruction { code: op, value: Some(val) }
                } else {
                    Instruction { code: op, value: None }
                }
            }
            _ => {
                let op = OpCode::parse(s.trim());
                if op.needs_value() {
                    panic!("syntax error: opcode needs a value: {}", s)
                }
                Instruction { code: op, value: None }
            }
        }
    }
}
