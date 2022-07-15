use program::compile_asm;
use std::env;
use std::panic;
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

    match compile_asm(args[1].as_str()) {
        Ok(prog) => {
            let ret_code = panic::catch_unwind(|| {
                let mut vm: VM = VM::new();
                let pid = vm.load_program(prog);
                vm.run_process(pid)
            });
            return match ret_code.is_ok() {
                true => ExitCode::SUCCESS,
                false => ExitCode::from(84)
            };
        }
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::from(84);
        }
    }
}
