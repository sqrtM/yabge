use crate::cpu::flag::Flag::{C, H, N};
use crate::cpu::value::Value;
use crate::cpu::CPU;

impl CPU {
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
        result
    }

    fn check_half_carry_add(a: Value, b: Value) -> bool {
        match (a, b) {
            (Value::EightBit(_), Value::EightBit(_)) => {
                (((a & Value::EightBit(0xF)) + (b & Value::EightBit(0xF))) & Value::EightBit(0x10))
                    == Value::EightBit(0x10)
            }

            (Value::SixteenBit(_), Value::SixteenBit(_)) => {
                (((a & Value::SixteenBit(0xFFF)) + (b & Value::SixteenBit(0xFFF)))
                    & Value::SixteenBit(0x1000))
                    == Value::SixteenBit(0x1000)
            }
            _ => {
                panic!("Attempting to compare values of different sizes.")
            }
        }
    }

    fn check_carry_add(a: Value, b: Value) -> bool {
        match (a, b) {
            (Value::EightBit(a), Value::EightBit(b)) => (u16::from(a) + u16::from(b)) > 255,
            (Value::SixteenBit(a), Value::SixteenBit(b)) => (u32::from(a) + u32::from(b)) > 65535,
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
