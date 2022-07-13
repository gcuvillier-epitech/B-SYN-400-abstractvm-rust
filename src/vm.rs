use std::collections::hash_map::HashMap;

use crate::process::Process;
use crate::program::Program;
use crate::value::Value;

pub struct VM<'a> {
    last_pid: usize,
    processes: HashMap<usize, Process<'a>>,
    registers: [Option<Value>; 16],
}

impl<'a> VM<'a> {
    pub fn new() -> VM<'a> {
        VM {
            last_pid: 0,
            processes: HashMap::new(),
            registers: [None, None, None, None,
                None, None, None, None,
                None, None, None, None,
                None, None, None, None],
        }
    }

    pub fn load_program(&mut self, p: &'a Program) -> usize {
        self.last_pid = self.last_pid + 1;
        self.processes.insert(self.last_pid, Process::new(&p));
        self.last_pid
    }

    pub fn run_process(&mut self, pid: usize) {
        match self.processes.get(&pid) {
            Some(v) => {
                v.run();
                self.processes.remove(&pid);
            }
            None => panic!("unknown process ID")
        }
    }
}