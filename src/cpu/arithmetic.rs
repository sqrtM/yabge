use crate::cpu::flag::Flag::{C, H, N, Z};
use crate::cpu::instruction::JumpCondition;
use crate::cpu::value::Value;
use crate::cpu::CPU;

impl CPU {
    pub(crate) fn should_jump(&self, condition: JumpCondition) -> bool {
        match condition {
            JumpCondition::FlagOn(flag) => self.registers.f.is_set(flag),
            JumpCondition::FlagOff(flag) => !self.registers.f.is_set(flag),
            JumpCondition::None => true,
        }
    }

    pub(crate) fn add(&mut self, a: Value, b: Value) -> Value {
        if Self::check_half_carry_add(a, b) {
            self.registers.f.set(H);
        } else {
            self.registers.f.unset(H);
        }
        if Self::check_carry_add(a, b) {
            self.registers.f.set(C);
        } else {
            self.registers.f.unset(C);
        }
        self.check_zero_flag(a + b);
        self.registers.f.unset(N);
        a + b
    }

    pub(crate) fn sub(&mut self, a: Value, b: Value) -> Value {
        if Self::check_half_carry_sub(a, b) {
            self.registers.f.set(H);
        } else {
            self.registers.f.unset(H);
        }
        if Self::check_carry_sub(a, b) {
            self.registers.f.set(C);
        } else {
            self.registers.f.unset(C);
        }
        self.registers.f.set(N);
        self.check_zero_flag(a - b);
        a - b
    }

    pub(crate) fn rol(&mut self, a: Value, use_carry: bool) -> Value {
        let result = match a {
            Value::EightBit(_) => {
                let val = a.rotate_left();
                if use_carry {
                    if self.registers.f.is_set(C) {
                        val | Value::EightBit(0x01)
                    } else {
                        val & Value::EightBit(0xFE)
                    }
                } else {
                    val
                }
            }
            Value::SixteenBit(_) => {
                let val = a.rotate_left();
                if use_carry {
                    if self.registers.f.is_set(C) {
                        val | Value::SixteenBit(0x0001)
                    } else {
                        val & Value::SixteenBit(0xFFFE)
                    }
                } else {
                    val
                }
            }
        };
        if Self::check_carry_left_rotate(a) {
            self.registers.f.set(C);
        } else {
            self.registers.f.unset(C);
        }
        self.registers.f.unset(N);
        self.check_zero_flag(result);
        result
    }

    pub(crate) fn ror(&mut self, a: Value, use_carry: bool) -> Value {
        let result = match a {
            Value::EightBit(_) => {
                let val = a.rotate_right();
                if use_carry {
                    if self.registers.f.is_set(C) {
                        val | Value::EightBit(0x80)
                    } else {
                        val & Value::EightBit(0x7F)
                    }
                } else {
                    val
                }
            }
            Value::SixteenBit(_) => {
                let val = a.rotate_right();
                if use_carry {
                    if self.registers.f.is_set(C) {
                        val | Value::SixteenBit(0x8000)
                    } else {
                        val & Value::SixteenBit(0x7FFF)
                    }
                } else {
                    val
                }
            }
        };
        if Self::check_carry_right_rotate(a) {
            self.registers.f.set(C);
        } else {
            self.registers.f.unset(C);
        };
        self.registers.f.unset(N);
        self.check_zero_flag(result);
        result
    }

    pub fn check_zero_flag(&mut self, a: Value) {
        if a == Value::EightBit(0) || a == Value::SixteenBit(0) {
            self.registers.f.set(Z)
        } else {
            self.registers.f.unset(Z)
        }
    }

    fn check_half_carry_add(a: Value, b: Value) -> bool {
        match (a, b) {
            (Value::EightBit(a), Value::EightBit(b)) => (((a & 0x0F) + (b & 0x0F)) & 0x10) == 0x10,
            (Value::SixteenBit(a), Value::SixteenBit(b)) => {
                (((a & 0x00FF) + (b & 0x00FF)) & 0x0100) == 0x0100
            }
            _ => panic!("Attempting to compare values of different sizes."),
        }
    }

    fn check_carry_add(a: Value, b: Value) -> bool {
        match (a, b) {
            (Value::EightBit(a), Value::EightBit(b)) => (u16::from(a) + u16::from(b)) > 0xFF,
            (Value::SixteenBit(a), Value::SixteenBit(b)) => (u32::from(a) + u32::from(b)) > 0xFFFF,
            _ => panic!("Attempting to compare values of different sizes."),
        }
    }

    fn check_half_carry_sub(a: Value, b: Value) -> bool {
        match (a, b) {
            (Value::EightBit(a), Value::EightBit(b)) => (a & 0xF) < (b & 0xF),
            (Value::SixteenBit(a), Value::SixteenBit(b)) => (a & 0xFFF) < (b & 0xFFF),
            _ => panic!("Attempting to compare values of different sizes."),
        }
    }

    fn check_carry_sub(a: Value, b: Value) -> bool {
        match (a, b) {
            (Value::EightBit(a), Value::EightBit(b)) => a < b,
            (Value::SixteenBit(a), Value::SixteenBit(b)) => a < b,
            _ => panic!("Attempting to compare values of different sizes."),
        }
    }

    fn check_carry_left_rotate(a: Value) -> bool {
        match a {
            Value::EightBit(a) => (a & 0x80) != 0,
            Value::SixteenBit(a) => (a & 0x8000) != 0,
        }
    }

    fn check_carry_right_rotate(a: Value) -> bool {
        match a {
            Value::EightBit(a) => (a & 0x01) != 0,
            Value::SixteenBit(a) => (a & 0x0001) != 0,
        }
    }
}

/// Bit manips to transmute u8/u16 to i16
/// while maintaining the bit order.
pub(crate) fn unsigned_to_signed(value: Value) -> i16 {
    match value {
        Value::EightBit(a) => {
            if a & 0x80 != 0 {
                -((!a).wrapping_add(1) as i8 as i16)
            } else {
                a as i16
            }
        }
        Value::SixteenBit(a) => {
            if a & 0x8000 != 0 {
                -((!a).wrapping_add(1) as i16)
            } else {
                a as i16
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::arithmetic::unsigned_to_signed;
    use crate::cpu::value::Value;
    use crate::cpu::CPU;

    #[test]
    fn test_check_half_carry_add() {
        let a = Value::EightBit(0x80);
        let b = Value::EightBit(0x90);
        let h = CPU::check_half_carry_add(a, b);
        assert!(!h);
    }

    #[test]
    fn test_check_carry_add() {
        let a = Value::EightBit(0x19);
        let b = Value::EightBit(0x28);
        let c = CPU::check_carry_add(a, b);
        assert!(!c);

        let a = Value::EightBit(0x80);
        let b = Value::EightBit(0x90);
        let c = CPU::check_carry_add(a, b);
        assert!(c);
    }

    #[test]
    fn test_unsigned_to_signed() {
        let a = Value::EightBit(0b1011_1101);
        let b = Value::SixteenBit(0b1000_1100_1110_1111);
        assert_eq!(a, Value::EightBit(189));
        assert_eq!(b, Value::SixteenBit(36079));

        let ia = unsigned_to_signed(a);
        let ib = unsigned_to_signed(b);
        assert_eq!(ia, -67);
        assert_eq!(ia as i8, 0b1011_1101u8 as i8);
        assert_eq!(ib, -29457);
        assert_eq!(ib, 0b1000_1100_1110_1111u16 as i16);
    }
}
