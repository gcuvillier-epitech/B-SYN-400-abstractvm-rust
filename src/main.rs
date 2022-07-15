use program::compile_asm;
use std::env;
use std::process::ExitCode;
use vm::VM;

mod value;
mod instruction;
mod program;
mod vm;
mod process;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Error: missing argument");
        eprintln!("Synopsys: abstract_vm <file_name>");
        return ExitCode::from(84);
    }

    let mut vm: VM = VM::new();

    let ret_code = match compile_asm(args[1].as_str()) {
        Ok(prog) => {
            let pid = vm.load_program(prog);
            vm.run_process(pid)
        }
        Err(e) => Err(e)
    };

    match ret_code {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{}", e);
            ExitCode::from(84)
        }
    }
}
