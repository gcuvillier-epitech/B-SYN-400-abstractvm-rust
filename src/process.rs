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
                OpCode::Noop => {},
                OpCode::Push => match &instruction.value {
                    Some(a) => st.push(a.clone()),
                    None => panic!("stack underflow")
                },
                OpCode::Pop => {
                    if st.len() < 1 {
                        panic!("stack underflow")
                    } else {
                        //st.pop()
                    }
                },
                OpCode::Dump => {},
                OpCode::Clear => {},
                OpCode::Dup => {},
                OpCode::Swap => {},
                OpCode::Assert => {},
                OpCode::Add => {
                    if st.len() < 2 {
                        panic!("stack underflow")
                    } else {
                        let v1 = Value::Int8(0);//stack.pop();
                        let v2 = Value::Int8(0);//stack.pop();
                        st.push(v1 + v2);
                    }
                },
                OpCode::Sub => {},
                OpCode::Mul => {},
                OpCode::Div => {},
                OpCode::Mod => {},
                OpCode::Load => {},
                OpCode::Store => {},
                OpCode::Print => {},
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