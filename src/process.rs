use std::cmp::min;

use crate::opcode::OpCode;
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
                registers: [None, None, None, None,
                    None, None, None, None,
                    None, None, None, None,
                    None, None, None, None],
                ip: 0,
                exited: false,
            },
        }
    }

    pub fn run(&mut self, count: usize) -> bool {
        // Error checking
        if count == 0 {
            panic!("no cycles given for process to run")
        }
        if self.state.ip == self.program.len() {
            panic!("process reached end of program instructions without exiting")
        }

        // Compute the instruction range to execute = from ip to min(program_len, ip+count)
        let exec_instructions = &self.program[self.state.ip..min(self.program.len(), self.state.ip + count)];

        // Execute each instruction in the range
        for instruction in exec_instructions {
            if self.state.exited {
                panic!("process attempted to run an instruction past after having exited")
            }
            match instruction.code {
                OpCode::Noop => {}
                OpCode::Push => match &instruction.value {
                    None => panic!("no associated value to push opcode"),
                    Some(v) => self.state.stack.push(v.clone()),
                },
                OpCode::Pop => match self.state.stack.pop() {
                    None => panic!("stack underflow - case 1"),
                    Some(_) => {} // already pop'ed in the match clause
                },
                OpCode::Dump => for v in self.state.stack.iter().rev() {
                    println!("{}", v);
                },
                OpCode::Clear => self.state.stack.clear(),
                OpCode::Dup => match self.state.stack.pop() {
                    None => panic!("stack underflow - case 2"),
                    Some(v) => {
                        self.state.stack.push(v.clone());
                        self.state.stack.push(v);
                    }
                },
                OpCode::Swap => match (self.state.stack.pop(), self.state.stack.pop()) {
                    (Some(v1), Some(v2)) => {
                        self.state.stack.push(v1);
                        self.state.stack.push(v2);
                    }
                    _ => panic!("stack underflow - case 3"),
                },
                OpCode::Assert => match self.state.stack.last() {
                    None => panic!("stack underflow"),
                    Some(v1) => match &instruction.value {
                        None => panic!("no associated value to assert instruction"),
                        Some(v2) => if !(v1 == v2) {
                            panic!("assertion failed: {:?} != {:?}", v1, v2)
                        },
                    },
                },
                OpCode::Add => match (self.state.stack.pop(), self.state.stack.pop()) {
                    (Some(v1), Some(v2)) => self.state.stack.push(v1 + v2),
                    _ => panic!("stack underflow - case 4"),
                },
                OpCode::Mul => match (self.state.stack.pop(), self.state.stack.pop()) {
                    (Some(v1), Some(v2)) => self.state.stack.push(v1 * v2),
                    _ => panic!("stack underflow - case 5"),
                },
                OpCode::Sub => match (self.state.stack.pop(), self.state.stack.pop()) {
                    (Some(v1), Some(v2)) => {
                        println!("{:?} {:?}", v1, v2);
                        self.state.stack.push(v2 - v1)
                    }
                    _ => panic!("stack underflow - case 5"),
                },
                OpCode::Div => match (self.state.stack.pop(), self.state.stack.pop()) {
                    (Some(v1), Some(v2)) => self.state.stack.push(v2 / v1),
                    _ => panic!("stack underflow - case 5"),
                },
                OpCode::Mod => match (self.state.stack.pop(), self.state.stack.pop()) {
                    (Some(v1), Some(v2)) => self.state.stack.push(v2 % v1),
                    _ => panic!("stack underflow - case 5"),
                },
                OpCode::Load => match &instruction.value {
                    None => panic!("no associated value to load opcode"),
                    Some(regval) => {
                        self.state.stack.push(match regval {        // alternative: regval.clone(), allowing to not have to dereference 'reg' everywhere
                            Value::Int8(reg) => {
                                if *reg < 0 || *reg > 15 {
                                    panic!("invalid register: {:?}", reg)
                                }
                                match &self.state.registers[*reg as usize] {
                                    None => panic!("load: register is empty: {}", reg),
                                    Some(v) => v.clone(),
                                }
                            }
                            other => panic!("value for register is not int8: {:?}", other)
                        })
                    }
                }
                OpCode::Store => match &instruction.value {
                    None => panic!("no associated value to store opcode"),
                    Some(regval) => match self.state.stack.pop() {
                        Some(sval) => match regval {
                            Value::Int8(reg) => {
                                if *reg < 0 || *reg > 15 {
                                    panic!("invalid register: {:?}", reg)
                                }
                                self.state.registers[*reg as usize] = Some(sval)
                            }
                            other => panic!("value for register is not int8: {:?}", other)
                        }
                        _ => panic!("stack underflow - case 9"),
                    },
                }
                OpCode::Print => match self.state.stack.last() {
                    None => panic!("stack underflow - case 10"),
                    Some(v) => match *v {                       // Why this works ??? Normally, &Value can't be dereferenced. Even the IDE complains, but the compiler is OK...
                        Value::Int8(v) => {
                            let c = v as u8;
                            match c.is_ascii() {
                                false => panic!("value is not ascii char: {}", c),
                                true => println!("{}", char::from(c))
                            }
                        }
                        _ => panic!("value is not int8: {:?}", v)
                    },
                },
                OpCode::Exit => self.state.exited = true,
            }
            self.state.ip = self.state.ip + 1;
        }
        !self.state.exited
    }
}
