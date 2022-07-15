use std::cmp::min;

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
            match instruction {
                Instruction::Noop => {}
                Instruction::Push(v) => self.state.stack.push(v.clone()),
                Instruction::Pop => match self.state.stack.pop() {
                    None => panic!("stack underflow - case 1"),
                    Some(_) => {} // already pop'ed in the match clause
                },
                Instruction::Dump => for v in self.state.stack.iter().rev() {
                    println!("{}", v);
                },
                Instruction::Clear => self.state.stack.clear(),
                Instruction::Dup => match self.state.stack.pop() {
                    None => panic!("stack underflow - case 2"),
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
                    _ => panic!("stack underflow - case 3"),
                },
                Instruction::Assert(v1) => match self.state.stack.last() {
                    None => panic!("stack underflow"),
                    Some(v2) => if !(v1 == v2) {
                        panic!("assertion failed: {:?} != {:?}", v1, v2)
                    },
                },
                Instruction::Add => match (self.state.stack.pop(), self.state.stack.pop()) {
                    (Some(v1), Some(v2)) => self.state.stack.push(v1 + v2),
                    _ => panic!("stack underflow - case 4"),
                },
                Instruction::Mul => match (self.state.stack.pop(), self.state.stack.pop()) {
                    (Some(v1), Some(v2)) => self.state.stack.push(v1 * v2),
                    _ => panic!("stack underflow - case 5"),
                },
                Instruction::Sub => match (self.state.stack.pop(), self.state.stack.pop()) {
                    (Some(v1), Some(v2)) => self.state.stack.push(v2 - v1),
                    _ => panic!("stack underflow - case 5"),
                },
                Instruction::Div => match (self.state.stack.pop(), self.state.stack.pop()) {
                    (Some(v1), Some(v2)) => self.state.stack.push(v2 / v1),
                    _ => panic!("stack underflow - case 5"),
                },
                Instruction::Mod => match (self.state.stack.pop(), self.state.stack.pop()) {
                    (Some(v1), Some(v2)) => self.state.stack.push(v2 % v1),
                    _ => panic!("stack underflow - case 5"),
                },
                Instruction::Load(v) => match &self.state.registers[*v] {
                    None => panic!("load: register is empty: {}", v),
                    Some(v) => self.state.stack.push(v.clone()),
                },
                Instruction::Store(v) => match self.state.stack.pop() {
                    None => panic!("stack underflow - case 9"),
                    val => self.state.registers[*v] = val,
                },
                Instruction::Print => match self.state.stack.last() {
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
                Instruction::Exit => self.state.exited = true,
            }
            self.state.ip = self.state.ip + 1;
        }
        !self.state.exited
    }
}
