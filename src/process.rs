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
        for instruction in self.program {
            match instruction.code {
                OpCode::Noop => { st.push(Value::Int8(0)) }
                _ => {}
            }
        }
    }
}