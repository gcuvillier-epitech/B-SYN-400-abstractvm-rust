use std::fmt;

#[allow(dead_code)]
#[derive(Copy, Clone)]
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
        write!(f, "{}", format!("{:?}", self))
    }
}

impl fmt::Debug for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpCode::Noop => write!(f, "noop"),
            OpCode::Push => write!(f, "push"),
            OpCode::Pop => write!(f, "pop"),
            OpCode::Dump => write!(f, "dump"),
            OpCode::Clear => write!(f, "clear"),
            OpCode::Dup => write!(f, "dup"),
            OpCode::Swap => write!(f, "swap"),
            OpCode::Assert => write!(f, "assert"),
            OpCode::Add => write!(f, "add"),
            OpCode::Sub => write!(f, "sub"),
            OpCode::Mul => write!(f, "mul"),
            OpCode::Div => write!(f, "div"),
            OpCode::Mod => write!(f, "mod"),
            OpCode::Load => write!(f, "load"),
            OpCode::Store => write!(f, "store"),
            OpCode::Print => write!(f, "print"),
            OpCode::Exit => write!(f, "exit"),
        }
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
