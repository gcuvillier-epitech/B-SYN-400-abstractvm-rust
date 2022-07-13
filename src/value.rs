use bigdecimal::{BigDecimal, FromPrimitive};

use std::fmt::{Debug, Display, Formatter, Result};
use std::ops::Add;

// Unfortunately Value can't be Copy-able because BigDecimal is not.
// Let's enable Clone anyway
#[derive(Clone, PartialEq)]
pub enum Value {
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Float(f32),
    Double(f64),
    BigDecimal(BigDecimal),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Value::Int8(arg) => write!(f, "{}", arg),
            Value::Int16(arg) => write!(f, "{}", arg),
            Value::Int32(arg) => write!(f, "{}", arg),
            Value::Float(arg) => write!(f, "{}", format!("{:.7}", arg).trim_end_matches(['0']).trim_end_matches(['.'])),
            Value::Double(arg) => write!(f, "{}", format!("{:.15}", arg).trim_end_matches(['0']).trim_end_matches(['.'])),
            Value::BigDecimal(arg) => write!(f, "{}", format!("{:.200}", arg).trim_end_matches(['0']).trim_end_matches(['.'])),
        }
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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

impl Value {
    pub fn parse(s: &str) -> Value {
        match (s.find('('), s.find(')')) {
            (Some(a), Some(b)) => {
                let first_token = s[..a].trim();
                let second_token = s[a + 1..b].trim();
                match first_token {
                    "int8" => match second_token.parse::<i8>() {
                        Ok(v) => Value::Int8(v),
                        Err(_) => panic!("syntax error: illegal int8: {}", s),
                    },
                    "int16" => match second_token.parse::<i16>() {
                        Ok(v) => Value::Int16(v),
                        Err(_) => panic!("syntax error: illegal int16: {}", s),
                    },
                    "int32" => match second_token.parse::<i32>() {
                        Ok(v) => Value::Int32(v),
                        Err(_) => panic!("syntax error: illegal int32: {}", s),
                    },
                    "float" => match second_token.parse::<f32>() {
                        Ok(v) => Value::Float(v),
                        Err(_) => panic!("syntax error: illegal float: {}", s),
                    },
                    "double" => match second_token.parse::<f64>() {
                        Ok(v) => Value::Double(v),
                        Err(_) => panic!("syntax error: illegal double: {}", s),
                    },
                    "bigdecimal" => match second_token.parse::<BigDecimal>() {
                        Ok(v) => Value::BigDecimal(v),
                        Err(_) => panic!("syntax error: illegal bigdecimal: {}", s),
                    },
                    _ => panic!("syntax error: unknown value type: {}", s)
                }
            }
            _ => panic!("syntax error: missing parenthesis: {}", s),
        }
    }
}
