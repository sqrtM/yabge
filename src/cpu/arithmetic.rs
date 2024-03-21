use crate::cpu::flag::Flag::{C, H, N, Z};
use crate::cpu::instruction::Condition;
use crate::cpu::value::Value;
use crate::cpu::CPU;

impl CPU {
    pub fn condition_passes(&self, condition: Condition) -> bool {
        match condition {
            Condition::FlagOn(flag) => self.registers.f.is_set(flag),
            Condition::FlagOff(flag) => !self.registers.f.is_set(flag),
            Condition::None => true,
        }
    }

    pub fn add(&mut self, a: Value, b: Value) -> Value {
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

    pub fn add_signed(&mut self, a: Value, b: i8) -> Value {
        if Self::check_half_carry_add_signed(a, b) {
            self.registers.f.set(H);
        } else {
            self.registers.f.unset(H);
        }
        if Self::check_carry_add_signed(a, b) {
            self.registers.f.set(C);
        } else {
            self.registers.f.unset(C);
        }
        self.check_zero_flag(a + b);
        self.registers.f.unset(N);
        a + b
    }

    pub fn sub(&mut self, a: Value, b: Value) -> Value {
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

    pub fn rol(&mut self, a: Value, use_carry: bool) -> Value {
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

    pub fn ror(&mut self, a: Value, use_carry: bool) -> Value {
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

    pub fn shl(&mut self, a: Value) -> Value {
        let result = match a {
            Value::EightBit(_) => a << 1u8,
            Value::SixteenBit(_) => a << 1u8,
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

    pub fn shr(&mut self, a: Value, arithmetic: bool) -> Value {
        let result = match a {
            Value::EightBit(_) => {
                if arithmetic {
                    let val = a >> 1u8;
                    val | Value::EightBit(0x80)
                } else {
                    a >> 1u8
                }
            }
            Value::SixteenBit(_) => {
                if arithmetic {
                    let val = a >> 1u8;
                    val | Value::SixteenBit(0x8000)
                } else {
                    a >> 1u8
                }
            }
        };
        if Self::check_carry_right_rotate(a) {
            self.registers.f.set(C);
        } else {
            self.registers.f.unset(C);
        }
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
            (Value::EightBit(a), Value::EightBit(b)) => {
                (((a & 0x0F).wrapping_add(b & 0x0F)) & 0x10) == 0x10
            }
            (Value::SixteenBit(a), Value::SixteenBit(b)) => {
                (((a & 0x00FF).wrapping_add(b & 0x00FF)) & 0x0100) == 0x0100
            }
            _ => panic!("Attempting to compare values of different sizes."),
        }
    }

    fn check_half_carry_add_signed(a: Value, b: i8) -> bool {
        match a {
            Value::EightBit(val) => (((val & 0x0F).wrapping_add_signed(b & 0x0F)) & 0x10) == 0x10,
            Value::SixteenBit(val) => {
                (((val & 0x00FF).wrapping_add_signed(b as i16 & 0x00FF)) & 0x0100) == 0x0100
            }
        }
    }

    fn check_carry_add(a: Value, b: Value) -> bool {
        match (a, b) {
            (Value::EightBit(a), Value::EightBit(b)) => (u16::from(a) + u16::from(b)) > 0xFF,
            (Value::SixteenBit(a), Value::SixteenBit(b)) => (u32::from(a) + u32::from(b)) > 0xFFFF,
            _ => panic!("Attempting to compare values of different sizes."),
        }
    }

    fn check_carry_add_signed(a: Value, i: i8) -> bool {
        match a {
            Value::EightBit(u) => {
                if i >= 0 {
                    let abs_i = i as u8;
                    u > u8::MAX - abs_i
                } else {
                    let abs_i = (-i) as u8;
                    abs_i > u
                }
            }
            Value::SixteenBit(u) => {
                if i >= 0 {
                    let abs_i = i as u16;
                    u > u16::MAX - abs_i
                } else {
                    let abs_i = (-i) as u16;
                    abs_i > u
                }
            }
        }
    }

    fn check_half_carry_sub(a: Value, b: Value) -> bool {
        match (a, b) {
            (Value::EightBit(a), Value::EightBit(b)) => {
                ((a & 0x0F).wrapping_sub(b & 0x0F)) & 0x10 == 0x10
            }
            (Value::SixteenBit(a), Value::SixteenBit(b)) => {
                ((a & 0x00FF).wrapping_sub(b & 0x00FF)) & 0x0100 == 0x0100
            }
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
pub fn unsigned_to_signed_16(value: Value) -> i16 {
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

/// Bit manips to transmute u8/u16 to i16
/// while maintaining the bit order.
pub fn unsigned_to_signed_8(value: Value) -> i8 {
    match value {
        Value::EightBit(a) => {
            if a & 0x80 != 0 {
                -((!a).wrapping_add(1) as i8)
            } else {
                a as i8
            }
        }
        Value::SixteenBit(a) => {
            if a & 0x8000 != 0 {
                -((!a).wrapping_add(1) as i8)
            } else {
                a as i8
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::arithmetic::unsigned_to_signed_16;
    use crate::cpu::flag::Flag::C;
    use crate::cpu::value::Value;
    use crate::cpu::CPU;

    #[test]
    fn test_add_signed() {
        let a = Value::EightBit(0x80);
        let b = 0b1100_1101u8 as i8; // -51
        let mut cpu = CPU::default();
        let h = cpu.add_signed(a, b);
        assert_eq!(h, Value::EightBit(0x4D));
    }

    #[test]
    fn test_check_half_carry_add() {
        let a = Value::EightBit(0x80);
        let b = Value::EightBit(0x90);
        let h = CPU::check_half_carry_add(a, b);
        assert!(!h);
    }

    #[test]
    fn test_check_half_carry_add_signed() {
        let a = Value::EightBit(0x88);
        let b = 0x78;
        let h = CPU::check_half_carry_add_signed(a, b);
        assert!(h);
    }

    #[test]
    fn test_check_half_carry_sub() {
        let a = Value::EightBit(0x47);
        let b = Value::EightBit(0x28);
        assert!(CPU::check_half_carry_sub(a, b));

        let c = Value::EightBit(0x60);
        let d = Value::EightBit(0x30);
        assert!(!CPU::check_half_carry_sub(c, d));
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
    fn test_check_carry_add_signed() {
        let a = Value::SixteenBit(0xFFFF);
        let b = 0x69;
        let c = CPU::check_carry_add_signed(a, b);
        assert!(c);

        let a = Value::SixteenBit(0xFFFF);
        let b = 0b1111_1111u8 as i8;
        let c = CPU::check_carry_add_signed(a, b);
        assert_eq!(b, -1);
        assert!(!c);
    }

    #[test]
    fn test_unsigned_to_signed() {
        let a = Value::EightBit(0b1011_1101);
        let b = Value::SixteenBit(0b1000_1100_1110_1111);
        assert_eq!(a, Value::EightBit(189));
        assert_eq!(b, Value::SixteenBit(36079));

        let ia = unsigned_to_signed_16(a);
        let ib = unsigned_to_signed_16(b);
        assert_eq!(ia, -67);
        assert_eq!(ia as i8, 0b1011_1101u8 as i8);
        assert_eq!(ib, -29457);
        assert_eq!(ib, 0b1000_1100_1110_1111u16 as i16);
    }

    #[test]
    fn test_shl() {
        let mut cpu = CPU::default();
        let a = Value::EightBit(0b1011_1101);
        let b = cpu.shl(a);
        assert!(cpu.registers.f.is_set(C));
        assert_eq!(b, Value::EightBit(0b0111_1010))
    }

    #[test]
    fn test_srl() {
        let mut cpu = CPU::default();
        let a = Value::EightBit(0b1011_1101);
        let b = cpu.shr(a, false);
        assert!(cpu.registers.f.is_set(C));
        assert_eq!(b, Value::EightBit(0b0101_1110))
    }

    #[test]
    fn test_sra() {
        let mut cpu = CPU::default();
        let a = Value::EightBit(0b1011_1101);
        let b = cpu.shr(a, true);
        assert!(cpu.registers.f.is_set(C));
        assert_eq!(b, Value::EightBit(0b1101_1110))
    }
}
