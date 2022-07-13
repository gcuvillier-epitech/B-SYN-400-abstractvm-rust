use crate::opcode::OpCode;
use crate::program::Program;
use crate::stack::Stack;
use crate::value::Value;

pub struct Process<'a> {
    program: &'a Program,
}

impl Process<'_> {
    pub fn new(p: &Program) -> Process {
        Process { program: p }
    }

    pub fn run(&self) {
        let mut st = Stack::new();
        let mut stop = false;
        for instruction in self.program {
            match instruction.code {
                OpCode::Noop => {}
                OpCode::Push => match &instruction.value {
                    Some(a) => st.push(a.clone()),
                    None => panic!("logic error: no value for push")
                },
                OpCode::Pop => {
                    match st.pop() {
                        Some(_) => {}
                        None => panic!("stack underflow"),
                    }
                }
                OpCode::Dump => {}
                OpCode::Clear => {
                    st.drain(..);
                }
                OpCode::Dup => {
                    match st.last() {
                        Some(a) => st.push(a.clone()),
                        _ => panic!("stack underflow"),
                    }
                }
                OpCode::Swap => {
                    match (st.pop(), st.pop()) {
                        (Some(v1), Some(v2)) => {
                            st.push(v1);
                            st.push(v2);
                        }
                        _ => panic!("stack underflow"),
                    }
                }
                OpCode::Assert => {
                    match st.last() {
                        Some(a) => {
                            match &instruction.value {
                                Some(b) => *a == *b,
                                None => panic!("logic error: no value for assert"),
                            }
                        }
                        _ => panic!("stack underflow"),
                    };
                }
                OpCode::Add => {
                    match (st.pop(), st.pop()) {
                        (Some(v1), Some(v2)) => {
                            st.push(v1 + v2);
                        }
                        _ => panic!("stack underflow"),
                    }
                }
                OpCode::Sub => {}
                OpCode::Mul => {}
                OpCode::Div => {}
                OpCode::Mod => {}
                OpCode::Load => {}
                OpCode::Store => {}
                OpCode::Print => {}
                OpCode::Exit => stop = true,
            }
            if stop {
                break;
            }
        }
        if !stop {
            panic!("program have not exited properly")
        }
    }
}