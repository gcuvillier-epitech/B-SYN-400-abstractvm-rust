use std::collections::hash_map::HashMap;

use crate::process::Process;
use crate::program::Program;

pub struct VM<'a> {
    lastpid: usize,
    processes: HashMap<usize, Process<'a>>,
}

impl<'a> VM<'a> {
    pub fn new() -> VM<'a> {
        VM { lastpid: 0, processes: HashMap::new() }
    }

    pub fn load_program(&mut self, p: &'a Program) -> usize {
        self.lastpid = self.lastpid + 1;
        self.processes.insert(self.lastpid, Process::new(&p));
        self.lastpid
    }

    pub fn run_process(&mut self, pid: usize) {
        match self.processes.get_mut(&pid) {
            Some(v) => {
                v.run()
            }
            None => {}
        }
    }
}