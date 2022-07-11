use std::fmt;

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub enum OpCode {
    Noop,
    Push,
    Pop,
    Dump,
    Clear,
    Dup,
    Swap,
    Assert,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Load,
    Store,
    Print,
    Exit,
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

pub fn op_code_needs_value(op: OpCode) -> bool {
    match op {
        OpCode::Push => true,
        OpCode::Assert => true,
        OpCode::Load => true,
        OpCode::Store => true,
        _ => false
    }
}
