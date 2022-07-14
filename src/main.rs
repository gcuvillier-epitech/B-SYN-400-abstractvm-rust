use program::compile_asm;
use std::env;
use std::panic;
use vm::VM;

mod opcode;
mod value;
mod instruction;
mod program;
mod vm;
mod process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Error: missing argument");
        eprintln!("Synopsys: abstract_vm <file_name>");
        std::process::exit(84);
    }

    let result = panic::catch_unwind(|| {
        let p = compile_asm(args[1].as_str());

        let mut vm: VM = VM::new();
        let pid = vm.load_program(p);
        vm.run_process(pid);
    });
    if result.is_ok() {
        std::process::exit(0);
    } else {
        std::process::exit(84);
    }
}
