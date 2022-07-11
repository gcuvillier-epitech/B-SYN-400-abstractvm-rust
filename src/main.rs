mod opcode;
mod value;

use opcode::*;
use value::*;

fn main() {
    let a = OpCode::Push;
    let b = op_code_needs_value(&a);
    let c = Value::Double(3.0);
    println!("Hello, world! {} {}", b, c);
}
