use crate::cpu::flag::Flag::{C, H};
use crate::cpu::value::Value;

pub enum Flag {
    // Zero
    Z,
    // Subtract
    N,
    // Half Carry
    H,
    // Carry
    C,
}

#[derive(Default, Clone, Copy)]
pub struct FlagRegister(u8);

impl FlagRegister {
    pub fn is_set(&self, f: Flag) -> bool {
        match f {
            Flag::Z => (1 << 7) & self.0 != 0,
            Flag::N => (1 << 6) & self.0 != 0,
            Flag::H => (1 << 5) & self.0 != 0,
            Flag::C => (1 << 4) & self.0 != 0,
        }
    }

    pub fn set(&mut self, f: Flag) {
        self.0 |= match f {
            Flag::Z => 1 << 7,
            Flag::N => 1 << 6,
            Flag::H => 1 << 5,
            Flag::C => 1 << 4,
        };
    }

    pub fn unset(&mut self, f: Flag) {
        self.0 &= match f {
            Flag::Z => !(1 << 7),
            Flag::N => !(1 << 6),
            Flag::H => !(1 << 5),
            Flag::C => !(1 << 4),
        };
    }

    pub fn get(&self) -> u8 {
        self.0 >> 4
    }

    pub fn overwrite(&mut self, value: u8) {
        self.0 = value
    }

    pub fn set_carry_flags(&mut self, value: Value) {
        if Self::check_carry(&value) {
            self.set(C)
        }
        if Self::check_half_carry(&value) {
            self.set(H)
        }
    }

    fn check_carry(value: &Value) -> bool {
        match value {
            Value::EightBit(val) => (val & (1 << 7)) != 0,
            Value::SixteenBit(val) => (val & (1 << 15)) != 0,
        }
    }

    fn check_half_carry(value: &Value) -> bool {
        match value {
            Value::EightBit(val) => (val & (1 << 4)) != 0,
            Value::SixteenBit(val) => (val & (1 << 12)) != 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::flag::{Flag, FlagRegister};
    use crate::cpu::value::Value;

    #[test]
    fn test_flag_setting() {
        let mut flag_register = FlagRegister::default();
        flag_register.set(Flag::Z);
        flag_register.set(Flag::C);
        assert!(flag_register.is_set(Flag::Z));
        assert!(!flag_register.is_set(Flag::N));
        assert!(!flag_register.is_set(Flag::H));
        assert!(flag_register.is_set(Flag::C));
    }

    #[test]
    fn test_flag_unset() {
        let mut flag_register = FlagRegister::default();
        flag_register.set(Flag::Z);
        flag_register.set(Flag::C);
        assert!(flag_register.is_set(Flag::Z));
        assert!(!flag_register.is_set(Flag::N));
        assert!(!flag_register.is_set(Flag::H));
        assert!(flag_register.is_set(Flag::C));
        flag_register.unset(Flag::Z);
        assert!(!flag_register.is_set(Flag::Z));
    }

    #[test]
    fn test_flag_initial_state() {
        let flag_register = FlagRegister::default();
        assert!(!flag_register.is_set(Flag::Z));
        assert!(!flag_register.is_set(Flag::N));
        assert!(!flag_register.is_set(Flag::H));
        assert!(!flag_register.is_set(Flag::C));
    }

    #[test]
    fn test_check_carry_8_bit_with_carry() {
        let value = Value::EightBit(0b1000_0000);
        assert!(FlagRegister::check_carry(&value));
    }

    #[test]
    fn test_check_carry_8_bit_without_carry() {
        let value = Value::EightBit(0b0000_0001);
        assert!(!FlagRegister::check_carry(&value));
    }

    #[test]
    fn test_check_carry_16_bit_with_carry() {
        let value = Value::SixteenBit(0b1000_0000_0000_0000);
        assert!(FlagRegister::check_carry(&value));
    }

    #[test]
    fn test_check_carry_16_bit_without_carry() {
        let value = Value::SixteenBit(0b0000_0000_0000_0001);
        assert!(!FlagRegister::check_carry(&value));
    }

    #[test]
    fn test_check_half_carry_flag_8_bit_with_half_carry() {
        let value = Value::EightBit(0b0001_0000);
        assert!(FlagRegister::check_half_carry(&value));
    }

    #[test]
    fn test_check_half_carry_flag_8_bit_without_half_carry() {
        let value = Value::EightBit(0b0000_0000);
        assert!(!FlagRegister::check_half_carry(&value));
    }

    #[test]
    fn test_check_half_carry_flag_16_bit_with_half_carry() {
        let value = Value::SixteenBit(0b0001_0000_0000_0000);
        assert!(FlagRegister::check_half_carry(&value));
    }

    #[test]
    fn test_check_half_carry_flag_16_bit_without_half_carry() {
        let value = Value::SixteenBit(0b0000_0000_0000_0000);
        assert!(!FlagRegister::check_half_carry(&value));
    }
}
