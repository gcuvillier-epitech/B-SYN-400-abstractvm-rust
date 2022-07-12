use opcode::*;
use std::panic;
use value::*;

mod opcode;
mod value;

fn main() {
    let result = panic::catch_unwind(|| {
        println!("{:?}", parse_value("   int8      ( 127 )   "))
    });
    if result.is_ok() {
        std::process::exit(0);
    } else {
        std::process::exit(84);
    }
}
