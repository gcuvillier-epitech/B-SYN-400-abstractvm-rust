use std::fmt;

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub enum Value {
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Float(f32),
    Double(f64),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}
