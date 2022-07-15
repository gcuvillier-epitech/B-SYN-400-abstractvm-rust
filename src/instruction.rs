use std::fmt::{Debug, Display, Formatter, Result};
use std::result;

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
    pub fn parse(s: &str) -> result::Result<Instruction, String> {
        let first_offset = s.find(' ');
        let (opcode, value) = match first_offset {
            Some(v) => (&s[..v], &s[v + 1..]),
            None => (s, s),
        };
        match opcode {
            "noop" => Ok(Instruction::Noop),
            "push" => match Value::parse(value.trim()) {
                Ok(v) => Ok(Instruction::Push(v)),
                Err(e) => Err(e)
            },
            "pop" => Ok(Instruction::Pop),
            "dump" => Ok(Instruction::Dump),
            "clear" => Ok(Instruction::Clear),
            "dup" => Ok(Instruction::Dup),
            "swap" => Ok(Instruction::Swap),
            "assert" => match Value::parse(value.trim()) {
                Ok(v) => Ok(Instruction::Assert(v)),
                Err(e) => Err(e)
            },
            "add" => Ok(Instruction::Add),
            "sub" => Ok(Instruction::Sub),
            "mul" => Ok(Instruction::Mul),
            "div" => Ok(Instruction::Div),
            "mod" => Ok(Instruction::Mod),
            "load" => match parse_reg(value.trim()) {
                Ok(v) => Ok(Instruction::Load(v)),
                Err(e) => Err(e)
            },
            "store" => match parse_reg(value.trim()) {
                Ok(v) => Ok(Instruction::Store(v)),
                Err(e) => Err(e)
            },
            "print" => Ok(Instruction::Print),
            "exit" => Ok(Instruction::Exit),
            _ => Err(format!("syntax error: unknown instruction: {}", s))
        }
    }
}

fn parse_reg(s: &str) -> result::Result<usize, String> {
    match Value::parse(s) {
        Ok(v) => match v {
            Value::Int8(v) => {
                if v < 0 || v > 15 {
                    return Err(format!("invalid register {}", s));
                }
                Ok(v as usize)
            }
            other => Err(format!("invalid value for register: {}", other))
        }
        Err(e) => Err(e)
    }
}
