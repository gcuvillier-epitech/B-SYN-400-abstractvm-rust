use std::fmt::{Debug, Display, Formatter, Result};

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

impl Display for OpCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", format!("{:?}", self))
    }
}

impl Debug for OpCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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

pub fn parse_op_code(s: &str) -> OpCode {
    match s {
        "noop" => OpCode::Noop,
        "push" => OpCode::Push,
        "pop" => OpCode::Pop,
        "dump" => OpCode::Dump,
        "clear" => OpCode::Clear,
        "dup" => OpCode::Dup,
        "swap" => OpCode::Swap,
        "assert" => OpCode::Assert,
        "add" => OpCode::Add,
        "sub" => OpCode::Sub,
        "mul" => OpCode::Mul,
        "div" => OpCode::Div,
        "mod" => OpCode::Mod,
        "load" => OpCode::Load,
        "store" => OpCode::Store,
        "print" => OpCode::Print,
        "exit" => OpCode::Exit,
        _ => panic!("unknown opcode")
    }
}
