#[derive(Debug)]
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

#[derive(Default, Debug, Clone, Copy)]
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
        self.0
    }

    pub fn overwrite(&mut self, value: u8) {
        self.0 = value
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::flag::{Flag, FlagRegister};

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
}
