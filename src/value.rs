use bigdecimal::BigDecimal;
use bigdecimal::FromPrimitive;

use std::fmt;
use std::ops::Add;

#[allow(dead_code)]
pub enum Value {
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Float(f32),
    Double(f64),
    BigDecimal(BigDecimal),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int8(arg) => write!(f, "{}", arg),
            Value::Int16(arg) => write!(f, "{}", arg),
            Value::Int32(arg) => write!(f, "{}", arg),
            Value::Float(arg) => write!(f, "{}", arg),
            Value::Double(arg) => write!(f, "{}", arg),
            Value::BigDecimal(arg) => write!(f, "bigdecimal({})", arg),
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int8(arg) => write!(f, "int8({})", arg),
            Value::Int16(arg) => write!(f, "int16({})", arg),
            Value::Int32(arg) => write!(f, "int32({})", arg),
            Value::Float(arg) => write!(f, "float({})", arg),
            Value::Double(arg) => write!(f, "double({})", arg),
            Value::BigDecimal(arg) => write!(f, "bigdecimal({})", arg),
        }
    }
}

impl Add for Value {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match self {
            Value::Int8(arg1) => match other {
                Value::Int8(arg2) => Value::Int8(arg1 + arg2),
                Value::Int16(arg2) => Value::Int16(arg1 as i16 + arg2),
                Value::Int32(arg2) => Value::Int32(arg1 as i32 + arg2),
                Value::Float(arg2) => Value::Float(arg1 as f32 + arg2),
                Value::Double(arg2) => Value::Double(arg1 as f64 + arg2),
                Value::BigDecimal(arg2) => Value::BigDecimal(BigDecimal::from_i8(arg1).unwrap() + arg2),
            },
            Value::Int16(arg1) => match other {
                Value::Int8(arg2) => Value::Int16(arg1 + arg2 as i16),
                Value::Int16(arg2) => Value::Int16(arg1 + arg2),
                Value::Int32(arg2) => Value::Int32(arg1 as i32 + arg2),
                Value::Float(arg2) => Value::Float(arg1 as f32 + arg2),
                Value::Double(arg2) => Value::Double(arg1 as f64 + arg2),
                Value::BigDecimal(arg2) => Value::BigDecimal(BigDecimal::from_i16(arg1).unwrap() + arg2),
            },
            Value::Int32(arg1) => match other {
                Value::Int8(arg2) => Value::Int32(arg1 + arg2 as i32),
                Value::Int16(arg2) => Value::Int32(arg1 + arg2 as i32),
                Value::Int32(arg2) => Value::Int32(arg1 + arg2),
                Value::Float(arg2) => Value::Float(arg1 as f32 + arg2),
                Value::Double(arg2) => Value::Double(arg1 as f64 + arg2),
                Value::BigDecimal(arg2) => Value::BigDecimal(BigDecimal::from_i32(arg1).unwrap() + arg2),
            },
            Value::Float(arg1) => match other {
                Value::Int8(arg2) => Value::Float(arg1 + arg2 as f32),
                Value::Int16(arg2) => Value::Float(arg1 + arg2 as f32),
                Value::Int32(arg2) => Value::Float(arg1 + arg2 as f32),
                Value::Float(arg2) => Value::Float(arg1 + arg2),
                Value::Double(arg2) => Value::Double(arg1 as f64 + arg2),
                Value::BigDecimal(arg2) => Value::BigDecimal(BigDecimal::from_f32(arg1).unwrap() + arg2),
            },
            Value::Double(arg1) => match other {
                Value::Int8(arg2) => Value::Double(arg1 + arg2 as f64),
                Value::Int16(arg2) => Value::Double(arg1 + arg2 as f64),
                Value::Int32(arg2) => Value::Double(arg1 + arg2 as f64),
                Value::Float(arg2) => Value::Double(arg1 + arg2 as f64),
                Value::Double(arg2) => Value::Double(arg1 + arg2 as f64),
                Value::BigDecimal(arg2) => Value::BigDecimal(BigDecimal::from_f64(arg1).unwrap() + arg2),
            },
            Value::BigDecimal(arg1) => match other {
                Value::Int8(arg2) => Value::BigDecimal(arg1 + BigDecimal::from_i8(arg2).unwrap()),
                Value::Int16(arg2) => Value::BigDecimal(arg1 + BigDecimal::from_i16(arg2).unwrap()),
                Value::Int32(arg2) => Value::BigDecimal(arg1 + BigDecimal::from_i32(arg2).unwrap()),
                Value::Float(arg2) => Value::BigDecimal(arg1 + BigDecimal::from_f32(arg2).unwrap()),
                Value::Double(arg2) => Value::BigDecimal(arg1 + BigDecimal::from_f64(arg2).unwrap()),
                Value::BigDecimal(arg2) => Value::BigDecimal(arg1 + arg2),
            },
        }
    }
}

pub fn test() {
    let a = Value::Int8(3);
    let b = Value::Float(4.0);
    let c = a + b;
    println!("{} {:?}", c, c);
}
