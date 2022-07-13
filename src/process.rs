use std::cmp::min;

use crate::instruction::Instruction;
use crate::opcode::OpCode;
use crate::program::Program;
use crate::stack::Stack;
use crate::value::Value;

pub struct Process {
    program: Program,
    stack: Stack,
    registers: [Option<Value>; 16],
    ip: usize,
    exited: bool,
}

impl Process {
    pub fn new(p: Program) -> Process {
        Process {
            program: p,
            stack: Stack::new(),
            registers: [None, None, None, None,
                None, None, None, None,
                None, None, None, None,
                None, None, None, None],
            ip: 0,
            exited: false,
        }
    }

    pub fn run(&mut self, count: usize) -> bool {
        // Error checking
        if count == 0 {
            panic!("no cycles given for process to run")
        }
        if self.ip == self.program.len() {
            panic!("process reached end of program instructions without exiting")
        }

        // Compute the instruction range to execute = from ip to min(program_len, ip+count)
        let exec_instructions = &self.program[self.ip..min(self.program.len(), self.ip + count)];

        // Execute each instruction in the range
        for instruction in exec_instructions {
            if self.exited {
                panic!("process attempted to run an instruction past after having exited")
            }
            match instruction.code {
                OpCode::Noop => {},
                OpCode::Push => match &instruction.value {
                    None => panic!("no associated value to push opcode"),
                    Some(v) => self.stack.push(v.clone()),
                },
                OpCode::Pop => match self.stack.pop() {
                    None => panic!("stack underflow"),
                    Some(_) => {} // already pop'ed in the match clause
                },
                OpCode::Dump => for v in self.stack.iter() {
                    println!("{}", v);
                },
                OpCode::Clear => self.stack.clear(),
                OpCode::Dup => match self.stack.last() {
                    None => panic!("stack underflow"),
                    Some(v) => self.stack.push(v.clone()),
                },
                OpCode::Swap => match (self.stack.pop(), self.stack.pop()) {
                    (Some(v1), Some(v2)) => {
                        self.stack.push(v1);
                        self.stack.push(v2);
                    },
                    _ => panic!("stack underflow"),
                },
                OpCode::Assert => match self.stack.last() {
                    None => panic!("stack underflow"),
                    Some(v1) => match &instruction.value {
                        None => panic!("no associated value to assert instruction"),
                        Some(v2) => if !(v1 == v2) {
                            panic!("assertion failed: {:?} != {:?}", v1, v2)
                        },
                    },
                },
                OpCode::Add => match (self.stack.pop(), self.stack.pop()) {
                    (Some(v1), Some(v2)) => self.stack.push(v1 + v2),
                    _ => panic!("stack underflow"),
                },
                OpCode::Sub => {}
                OpCode::Mul => {}
                OpCode::Div => {}
                OpCode::Mod => {}
                OpCode::Load => {}
                OpCode::Store => {}
                OpCode::Print => match self.stack.last() {
                    None => panic!("stack underflow"),
                    Some(v) => match v {
                        Value::Int8(v) => {
                            let c = *v as u8;
                            match c.is_ascii() {
                                false => panic!("value is not ascii char: {}", c),
                                true => println!("{}", char::from(c))
                            }
                        },
                        _ => panic!("value is not int8: {:?}", v)
                    },
                },
                OpCode::Exit => self.exited = true,
            }
            self.ip = self.ip + 1;
        }
        !self.exited
    }
}
