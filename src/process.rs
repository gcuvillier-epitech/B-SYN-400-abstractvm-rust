use std::cmp::min;
use std::panic;
use std::result;

use crate::instruction::Instruction;
use crate::program::Program;
use crate::value::Value;

pub struct Process {
    program: Program,
    state: State,
}

struct State {
    stack: Vec<Value>,
    registers: [Option<Value>; 16],
    ip: usize,
    exited: bool,
}

impl Process {
    pub fn new(p: Program) -> Process {
        Process {
            program: p,
            state: State {
                stack: Vec::new(),
                registers: [
                    None, None, None, None, None, None, None, None, None, None, None, None, None,
                    None, None, None,
                ],
                ip: 0,
                exited: false,
            },
        }
    }

    pub fn run(&mut self, count: usize) -> result::Result<bool, String> {
        // Error checking
        if count == 0 {
            return Err(format!("no cycles given for process to run"));
        }
        if self.state.ip == self.program.len() {
            return Err(format!(
                "process reached end of program instructions without exiting"
            ));
        }

        // Compute the instruction range to execute = from ip to min(program_len, ip+count)
        let exec_instructions =
            &self.program[self.state.ip..min(self.program.len(), self.state.ip + count)];

        // Execute each instruction in the range
        for instruction in exec_instructions {
            if self.state.exited {
                return Err(format!(
                    "process attempted to run an instruction past after having exited"
                ));
            }
            match instruction {
                Instruction::Noop => {}
                Instruction::Push(v) => self.state.stack.push(v.clone()),
                Instruction::Pop => match self.state.stack.pop() {
                    None => return Err(format!("stack underflow - pop")),
                    Some(_) => {} // already pop'ed in the match clause
                },
                Instruction::Dump => {
                    for v in self.state.stack.iter().rev() {
                        println!("{}", v);
                    }
                }
                Instruction::Clear => self.state.stack.clear(),
                Instruction::Dup => match self.state.stack.pop() {
                    None => return Err(format!("stack underflow - dup")),
                    Some(v) => {
                        self.state.stack.push(v.clone());
                        self.state.stack.push(v);
                    }
                },
                Instruction::Swap => match (self.state.stack.pop(), self.state.stack.pop()) {
                    (Some(v1), Some(v2)) => {
                        self.state.stack.push(v1);
                        self.state.stack.push(v2);
                    }
                    _ => return Err(format!("stack underflow - swap")),
                },
                Instruction::Assert(v1) => match self.state.stack.last() {
                    None => return Err(format!("stack underflow - assert")),
                    Some(v2) => {
                        if !(v1 == v2) {
                            return Err(format!("assertion failed: {:?} != {:?}", v1, v2));
                        }
                    }
                },
                Instruction::Add => match (self.state.stack.pop(), self.state.stack.pop()) {
                    (Some(v1), Some(v2)) => match panic::catch_unwind(|| v1 + v2) {
                        Ok(v) => self.state.stack.push(v),
                        Err(_) => {
                            return Err(format!("Arithmetic panic (see panic message) - add"))
                        } // This is useless to catch the cause, as it is an Any { ... } on arithmetic panics
                    },
                    _ => return Err(format!("stack underflow - add")),
                },
                Instruction::Mul => match (self.state.stack.pop(), self.state.stack.pop()) {
                    (Some(v1), Some(v2)) => match panic::catch_unwind(|| v1 * v2) {
                        Ok(v) => self.state.stack.push(v),
                        Err(_) => {
                            return Err(format!("Arithmetic panic (see panic message) - mul"))
                        }
                    },
                    _ => return Err(format!("stack underflow - mul")),
                },
                Instruction::Sub => match (self.state.stack.pop(), self.state.stack.pop()) {
                    (Some(v1), Some(v2)) => match panic::catch_unwind(|| v2 - v1) {
                        Ok(v) => self.state.stack.push(v),
                        Err(_) => {
                            return Err(format!("Arithmetic panic (see panic message) - sub"))
                        }
                    },
                    _ => return Err(format!("stack underflow - sub")),
                },
                Instruction::Div => match (self.state.stack.pop(), self.state.stack.pop()) {
                    (Some(v1), Some(v2)) => match panic::catch_unwind(|| v2 / v1) {
                        Ok(v) => self.state.stack.push(v),
                        Err(_) => {
                            return Err(format!("Arithmetic panic (see panic message) - div"))
                        }
                    },
                    _ => return Err(format!("stack underflow - div")),
                },
                Instruction::Mod => match (self.state.stack.pop(), self.state.stack.pop()) {
                    (Some(v1), Some(v2)) => match panic::catch_unwind(|| v2 % v1) {
                        Ok(v) => self.state.stack.push(v),
                        Err(_) => {
                            return Err(format!("Arithmetic panic (see panic message) - mod"))
                        }
                    },
                    _ => return Err(format!("stack underflow - mod")),
                },
                Instruction::Load(v) => match &self.state.registers[*v] {
                    None => return Err(format!("load: register is empty: {}", v)),
                    Some(v) => self.state.stack.push(v.clone()),
                },
                Instruction::Store(v) => match self.state.stack.pop() {
                    None => return Err(format!("stack underflow - store")),
                    val => self.state.registers[*v] = val,
                },
                Instruction::Print => match self.state.stack.last() {
                    None => return Err(format!("stack underflow - print")),
                    Some(v) => match v {
                        Value::Int8(v) => {
                            let c = *v as u8;
                            match c.is_ascii() {
                                false => return Err(format!("value is not ascii char: {}", c)),
                                true => println!("{}", char::from(c)),
                            }
                        }
                        _ => return Err(format!("value is not int8: {:?}", v)),
                    },
                },
                Instruction::Exit => self.state.exited = true,
            }
            self.state.ip = self.state.ip + 1;
        }
        Ok(!self.state.exited)
    }
}
