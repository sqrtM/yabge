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

#[derive(Default)]
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
}
