use crate::cpu::flag::FlagRegister;

pub enum Register {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Default)]
pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    pub f: FlagRegister,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
}

impl Registers {
    pub(crate) fn set(&mut self, register: Register, value: u8) {
        match register {
            Register::A => self.a = value,
            Register::B => self.b = value,
            Register::C => self.c = value,
            Register::D => self.d = value,
            Register::E => self.e = value,
            Register::H => self.h = value,
            Register::L => self.l = value,
        }
    }

    pub(crate) fn af(&self) -> u16 {
        Self::merge_registers(self.a, self.f.get())
    }

    pub(crate) fn bc(&self) -> u16 {
        Self::merge_registers(self.b, self.c)
    }

    pub(crate) fn de(&self) -> u16 {
        Self::merge_registers(self.d, self.e)
    }

    pub(crate) fn hl(&self) -> u16 {
        Self::merge_registers(self.h, self.l)
    }

    pub(crate) fn merge_registers(hi: u8, lo: u8) -> u16 {
        (hi as u16) << 8 | lo as u16
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
