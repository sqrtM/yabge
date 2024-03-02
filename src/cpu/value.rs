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

impl std::ops::BitOr<Value> for Value {
    type Output = Value;

    fn bitor(self, rhs: Value) -> Value {
        match (self, rhs) {
            (Value::EightBit(a), Value::EightBit(b)) => Value::EightBit(a | b),
            (Value::SixteenBit(a), Value::SixteenBit(b)) => Value::SixteenBit(a | b),
            _ => panic!("Attempted to perform bitwise OR on values of different sizes"),
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

impl Value {
    pub(crate) fn rotate_right(self) -> Self {
        match self {
            Value::EightBit(a) => Value::EightBit(a.rotate_right(1)),
            Value::SixteenBit(a) => Value::SixteenBit(a.rotate_right(1)),
        }
    }

    pub(crate) fn rotate_left(self) -> Self {
        match self {
            Value::EightBit(a) => Value::EightBit(a.rotate_left(1)),
            Value::SixteenBit(a) => Value::SixteenBit(a.rotate_left(1)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ror() {
        let a = Value::SixteenBit(0b0010_0010_0000_0001);
        let b = a.rotate_right();
        assert_eq!(b, Value::SixteenBit(0b1001_0001_0000_0000))
    }

    #[test]
    fn test_rol() {
        let a = Value::EightBit(0b1010_0010);
        let b = a.rotate_left();
        assert_eq!(b, Value::EightBit(0b0100_0101))
    }

    #[test]
    fn test_or() {
        let a = Value::EightBit(0b1010_0010);
        let b = Value::EightBit(0b1110_1000);
        let c = a | b;
        assert_eq!(c, Value::EightBit(0b1110_1010))
    }
}
