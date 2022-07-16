use std::collections::hash_map::HashMap;
use std::process::ExitCode;
use std::result;

use crate::process::Process;
use crate::program::Program;

const VM_RUN_CYCLES: usize = 7;

pub struct VM {
    last_pid: usize,
    processes: HashMap<usize, Process>,
}

impl VM {
    pub fn new() -> VM {
        VM {
            last_pid: 0,
            processes: HashMap::new(),
        }
    }

    pub fn load_program(&mut self, p: Program) -> usize {
        self.last_pid = self.last_pid + 1;
        self.processes.insert(self.last_pid, Process::new(p));
        self.last_pid
    }

    pub fn run_process(&mut self, pid: usize) -> result::Result<ExitCode, String> {
        match self.processes.get_mut(&pid) {
            None => Err(format!("process ID does not exists: {}", pid)),
            Some(p) => loop {
                match p.run(VM_RUN_CYCLES) {
                    Ok(r) => {
                        if r == false {
                            self.processes.remove(&pid);
                            return Ok(ExitCode::SUCCESS);
                        }
                    }
                    Err(e) => {
                        self.processes.remove(&pid);
                        return Err(e);
                    }
                }
            },
        }
    }
}
