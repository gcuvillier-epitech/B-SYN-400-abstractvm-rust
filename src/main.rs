use program::*;
use std::panic;
use value::*;
use vm::*;

mod opcode;
mod value;
mod instruction;
mod stack;
mod program;
mod vm;
mod process;

fn main() {
    let result = panic::catch_unwind(|| {
        let p: Program = compile_asm("./toto.avm");
        let mut vm: VM = VM::new();

        let pid = vm.load_program(p);

        vm.run_process(pid);

        let a = Value::Int8(0);
        let b = Value::Int16(0);
        println!("{}", a == b);
    });
    if result.is_ok() {
        std::process::exit(0);
    } else {
        std::process::exit(84);
    }
}
