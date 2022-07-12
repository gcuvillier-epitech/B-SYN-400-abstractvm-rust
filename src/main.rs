use std::panic;

use program::*;

mod opcode;
mod value;
mod instruction;
mod stack;
mod program;

fn main() {
    let result = panic::catch_unwind(|| {
        let _p: Program = compile_asm("./toto.avm");
    });
    if result.is_ok() {
        std::process::exit(0);
    } else {
        std::process::exit(84);
    }
}
