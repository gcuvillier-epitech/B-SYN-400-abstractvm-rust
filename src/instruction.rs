use std::fmt::{Debug, Display, Formatter, Result};

use crate::value::Value;

pub enum Instruction {
    Noop,
    Push(Value),
    Pop,
    Dump,
    Clear,
    Dup,
    Swap,
    Assert(Value),
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Load(usize),
    Store(usize),
    Print,
    Exit,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", format!("{:?}", self))
    }
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Instruction::Noop => write!(f, "noop"),
            Instruction::Push(v) => write!(f, "push {}", v),
            Instruction::Pop => write!(f, "pop"),
            Instruction::Dump => write!(f, "dump"),
            Instruction::Clear => write!(f, "clear"),
            Instruction::Dup => write!(f, "dup"),
            Instruction::Swap => write!(f, "swap"),
            Instruction::Assert(v) => write!(f, "assert{}", v),
            Instruction::Add => write!(f, "add"),
            Instruction::Sub => write!(f, "sub"),
            Instruction::Mul => write!(f, "mul"),
            Instruction::Div => write!(f, "div"),
            Instruction::Mod => write!(f, "mod"),
            Instruction::Load(v) => write!(f, "load {}", v),
            Instruction::Store(v) => write!(f, "store {}", v),
            Instruction::Print => write!(f, "print"),
            Instruction::Exit => write!(f, "exit"),
        }
    }
}

impl Instruction
{
    pub fn parse(s: &str) -> Instruction {
        let first_offset = s.find(' ');
        let (opcode, value) = match first_offset {
            Some(v) => (&s[..v], &s[v + 1..]),
            None => (s, s),
        };
        match opcode {
            "noop" => Instruction::Noop,
            "push" => Instruction::Push(Value::parse(value.trim())),
            "pop" => Instruction::Pop,
            "dump" => Instruction::Dump,
            "clear" => Instruction::Clear,
            "dup" => Instruction::Dup,
            "swap" => Instruction::Swap,
            "assert" => Instruction::Assert(Value::parse(value.trim())),
            "add" => Instruction::Add,
            "sub" => Instruction::Sub,
            "mul" => Instruction::Mul,
            "div" => Instruction::Div,
            "mod" => Instruction::Mod,
            "load" => Instruction::Load(parse_reg(value.trim())),
            "store" => Instruction::Store(parse_reg(value.trim())),
            "print" => Instruction::Print,
            "exit" => Instruction::Exit,
            _ => panic!("syntax error: unknown instruction: {}", s)
        }
    }
}

fn parse_reg(s: &str) -> usize {
    match Value::parse(s) {
        Value::Int8(v) => {
            if v < 0 || v > 15 {
                panic!("invalid register {}", s)
            }
            v as usize
        }
        other => panic!("invalid value for register: {}", other)
    }
}
