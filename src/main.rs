mod opcode;
mod value;

use opcode::*;
use value::*;

fn main() {
    let a = OpCode::Push;
    let b = op_code_needs_value(a);
    let c = Value::Int16(3);
    println!("Hello, world! {} {}", b, c);

    let d = Value::Int8(4);

    let e = c + d;
    println!("Hello, world! {}", e);

    test();

}
