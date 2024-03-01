#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Value {
    EightBit(u8),
    SixteenBit(u16),
}

impl std::ops::Add<Value> for Value {
    type Output = Value;

    fn add(self, rhs: Value) -> Value {
        match (self, rhs) {
            (Value::EightBit(a), Value::EightBit(b)) => Value::EightBit(a.wrapping_add(b)),
            (Value::SixteenBit(a), Value::SixteenBit(b)) => Value::SixteenBit(a.wrapping_add(b)),
            _ => panic!("Attempted to add values of different sizes"),
        }
    }
}

impl std::ops::BitAnd<Value> for Value {
    type Output = Value;

    fn bitand(self, rhs: Value) -> Value {
        match (self, rhs) {
            (Value::EightBit(a), Value::EightBit(b)) => Value::EightBit(a & b),
            (Value::SixteenBit(a), Value::SixteenBit(b)) => Value::SixteenBit(a & b),
            _ => panic!("Attempted to perform bitwise AND on values of different sizes"),
        }
    }
}

impl std::ops::Sub<Value> for Value {
    type Output = Value;

    fn sub(self, rhs: Value) -> Value {
        match (self, rhs) {
            (Value::EightBit(a), Value::EightBit(b)) => Value::EightBit(a.wrapping_sub(b)),
            (Value::SixteenBit(a), Value::SixteenBit(b)) => Value::SixteenBit(a.wrapping_sub(b)),
            _ => panic!("Attempted to subtract values of different sizes"),
        }
    }
}
