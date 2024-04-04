use std::ops::{Add, BitXor, Not, Sub};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Value {
    EightBit(u8),
    SixteenBit(u16),
}

pub fn concat_values(hi: Value, lo: Value) -> Value {
    match (hi, lo) {
        (Value::EightBit(hi_val), Value::EightBit(lo_val)) => {
            let concatenated = ((hi_val as u16) << 8) | lo_val as u16;
            Value::SixteenBit(concatenated)
        }
        _ => panic!("Invalid concatenation arguments."),
    }
}

impl Add<Value> for Value {
    type Output = Value;

    fn add(self, rhs: Value) -> Value {
        match (self, rhs) {
            (Value::EightBit(a), Value::EightBit(b)) => Value::EightBit(a.wrapping_add(b)),
            (Value::SixteenBit(a), Value::SixteenBit(b)) => Value::SixteenBit(a.wrapping_add(b)),
            _ => panic!("Attempted to add values of different sizes"),
        }
    }
}

impl Add<u8> for Value {
    type Output = Value;

    fn add(self, rhs: u8) -> Value {
        match self {
            Value::EightBit(a) => Value::EightBit(a.wrapping_add(rhs)),
            Value::SixteenBit(a) => Value::SixteenBit(a.wrapping_add(rhs as u16)),
        }
    }
}

impl Add<i8> for Value {
    type Output = Value;

    fn add(self, rhs: i8) -> Value {
        match self {
            Value::EightBit(a) => Value::EightBit(a.wrapping_add(rhs as u8)),
            Value::SixteenBit(a) => Value::SixteenBit(a.wrapping_add(rhs as u16)),
        }
    }
}

impl Add<u16> for Value {
    type Output = Value;

    fn add(self, rhs: u16) -> Value {
        match self {
            Value::EightBit(a) => Value::EightBit(a.wrapping_add(rhs as u8)),
            Value::SixteenBit(a) => Value::SixteenBit(a.wrapping_add(rhs)),
        }
    }
}

impl Add<i16> for Value {
    type Output = Value;

    fn add(self, rhs: i16) -> Value {
        match self {
            Value::EightBit(a) => Value::EightBit(a.wrapping_add(rhs as u8)),
            Value::SixteenBit(a) => Value::SixteenBit(a.wrapping_add(rhs as u16)),
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

impl std::ops::Shl<u8> for Value {
    type Output = Value;

    fn shl(self, rhs: u8) -> Value {
        match self {
            Value::EightBit(a) => Value::EightBit(a << rhs),
            Value::SixteenBit(a) => Value::SixteenBit(a << rhs),
        }
    }
}

impl std::ops::Shr<u8> for Value {
    type Output = Value;

    fn shr(self, rhs: u8) -> Value {
        match self {
            Value::EightBit(a) => Value::EightBit(a >> rhs),
            Value::SixteenBit(a) => Value::SixteenBit(a >> rhs),
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

impl BitXor<Value> for Value {
    type Output = Value;

    fn bitxor(self, rhs: Value) -> Value {
        match (self, rhs) {
            (Value::EightBit(a), Value::EightBit(b)) => Value::EightBit(a ^ b),
            (Value::SixteenBit(a), Value::SixteenBit(b)) => Value::SixteenBit(a ^ b),
            _ => panic!("Attempted to perform bitwise XOR on values of different sizes"),
        }
    }
}

impl Sub<Value> for Value {
    type Output = Value;

    fn sub(self, rhs: Value) -> Value {
        match (self, rhs) {
            (Value::EightBit(a), Value::EightBit(b)) => Value::EightBit(a.wrapping_sub(b)),
            (Value::SixteenBit(a), Value::SixteenBit(b)) => Value::SixteenBit(a.wrapping_sub(b)),
            _ => panic!("Attempted to subtract values of different sizes"),
        }
    }
}

impl Sub<u8> for Value {
    type Output = Value;

    fn sub(self, rhs: u8) -> Value {
        match self {
            Value::EightBit(a) => Value::EightBit(a.wrapping_sub(rhs)),
            Value::SixteenBit(a) => Value::SixteenBit(a.wrapping_sub(rhs as u16)),
        }
    }
}

impl Sub<i8> for Value {
    type Output = Value;

    fn sub(self, rhs: i8) -> Value {
        match self {
            Value::EightBit(a) => Value::EightBit(a.wrapping_sub(rhs as u8)),
            Value::SixteenBit(a) => Value::SixteenBit(a.wrapping_sub(rhs as u16)),
        }
    }
}

impl Sub<u16> for Value {
    type Output = Value;

    fn sub(self, rhs: u16) -> Value {
        match self {
            Value::EightBit(a) => Value::EightBit(a.wrapping_sub(rhs as u8)),
            Value::SixteenBit(a) => Value::SixteenBit(a.wrapping_sub(rhs)),
        }
    }
}

impl Sub<i16> for Value {
    type Output = Value;

    fn sub(self, rhs: i16) -> Value {
        match self {
            Value::EightBit(a) => Value::EightBit(a.wrapping_sub(rhs as u8)),
            Value::SixteenBit(a) => Value::SixteenBit(a.wrapping_sub(rhs as u16)),
        }
    }
}

impl Not for Value {
    type Output = Value;

    fn not(self) -> Value {
        match self {
            Value::EightBit(a) => Value::EightBit(!a),
            Value::SixteenBit(a) => Value::SixteenBit(!a),
        }
    }
}

impl Value {
    pub fn rotate_right(self) -> Self {
        match self {
            Value::EightBit(a) => Value::EightBit(a.rotate_right(1)),
            Value::SixteenBit(a) => Value::SixteenBit(a.rotate_right(1)),
        }
    }

    pub fn rotate_left(self) -> Self {
        match self {
            Value::EightBit(a) => Value::EightBit(a.rotate_left(1)),
            Value::SixteenBit(a) => Value::SixteenBit(a.rotate_left(1)),
        }
    }
}

impl Value {
    pub fn high_byte(&self) -> Value {
        match self {
            Value::EightBit(_) => panic!("Attempting to take high byte from u8"),
            Value::SixteenBit(val) => Value::EightBit(((*val >> 8) & 0xFF) as u8),
        }
    }

    pub fn low_byte(&self) -> Value {
        match self {
            Value::EightBit(_) => panic!("Attempting to take low byte from u8"),
            Value::SixteenBit(val) => Value::EightBit((*val & 0xFF) as u8),
        }
    }
}

impl Value {
    pub fn swap(&self) -> Value {
        match *self {
            Value::EightBit(val) => {
                let upper_nibble = (val & 0xF0) >> 4;
                let lower_nibble = (val & 0x0F) << 4;
                Value::EightBit(upper_nibble | lower_nibble)
            }
            _ => panic!("Swap operation is only applicable to 8-bit values"),
        }
    }
}

impl Value {
    pub fn extract(&self) -> u16 {
        match *self {
            Value::EightBit(val) => {
                val as u16
            },
            Value::SixteenBit(val) => {
                val
            }
            _ => panic!("Failed to extract value"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let a = Value::SixteenBit(0x0002);
        let b = a + 2i8;
        assert_eq!(b, Value::SixteenBit(0x0004));

        let a = Value::EightBit(0x02);
        let b = a + 2u16;
        assert_eq!(b, Value::EightBit(0x04))
    }

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

    #[test]
    fn test_not() {
        let a = Value::EightBit(0b1010_0010);
        let b = Value::SixteenBit(0b1110_1000);
        assert_eq!(!a, Value::EightBit(0b0101_1101));
        assert_eq!(!b, Value::SixteenBit(0b1111_1111_0001_0111u16))
    }

    #[test]
    fn test_shl() {
        let a = Value::EightBit(0b1010_0010);
        let b = Value::SixteenBit(0b1110_1000);
        assert_eq!(a << 1, Value::EightBit(0b0100_0100));
        assert_eq!(b << 1, Value::SixteenBit(0b0000_0001_1101_0000u16))
    }

    #[test]
    fn test_high_byte() {
        let a = Value::SixteenBit(0b1010_0010_1110_1011);
        assert_eq!(a.high_byte(), Value::EightBit(0b1010_0010));
    }

    #[test]
    fn test_low_byte() {
        let a = Value::SixteenBit(0b1010_0010_1110_1011);
        assert_eq!(a.low_byte(), Value::EightBit(0b1110_1011));
    }

    #[test]
    fn test_swap() {
        let a = Value::EightBit(0b1010_0010);
        let b = Value::EightBit(0b1111_0000);
        let c = Value::EightBit(0b0000_0100);
        assert_eq!(a.swap(), Value::EightBit(0b0010_1010));
        assert_eq!(b.swap(), Value::EightBit(0b0000_1111));
        assert_eq!(c.swap(), Value::EightBit(0b0100_0000));
    }
}
