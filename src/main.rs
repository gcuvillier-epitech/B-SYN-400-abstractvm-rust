use std::env;
use std::process::ExitCode;

use program::compile_asm;
use vm::VM;

mod instruction;
mod process;
mod program;
mod value;
mod vm;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let filename = match &args[..] {
        [ _ , f ] => f.as_str(),
        _ => {
            eprintln!("Error: missing argument");
            eprintln!("Synopsys: abstract_vm <file_name>");
            return ExitCode::from(84);
        }
    };

    let mut vm: VM = VM::new();

    let ret_code = match compile_asm(filename) {
        Ok(prog) => {
            let pid = vm.load_program(prog);
            vm.run_process(pid)
        }
        Err(e) => Err(e),
    };

    match ret_code {
        Ok(code) => code, // should be ExitCode::SUCCESS, or the exit code given by the program that have been run
        Err(e) => {
            eprintln!("{}", e);
            ExitCode::from(84)
        }
    }
}
