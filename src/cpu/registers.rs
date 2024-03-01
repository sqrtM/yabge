use crate::cpu::flag::FlagRegister;
use crate::cpu::value::Value;

#[derive(Clone, Copy)]
pub enum Register {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    AF,
    BC,
    DE,
    HL,
}

#[derive(Default, Clone, Copy)]
pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    pub f: FlagRegister,
    h: u8,
    l: u8,
}

impl Registers {
    pub(crate) fn set(&mut self, register: Register, value: Value) {
        match value {
            Value::EightBit(v) => match register {
                Register::A => self.a = v,
                Register::B => self.b = v,
                Register::C => self.c = v,
                Register::D => self.d = v,
                Register::E => self.e = v,
                Register::H => self.h = v,
                Register::L => self.l = v,
                _ => {
                    panic!("Attempting to load an 8 bit value to a 16 bit register")
                }
            },
            Value::SixteenBit(v) => match register {
                Register::AF => {
                    let (hi, lo) = Self::split_bytes(v);
                    self.a = hi;
                    self.f.overwrite(lo);
                }
                Register::BC => {
                    let (hi, lo) = Self::split_bytes(v);
                    self.b = hi;
                    self.c = lo;
                }
                Register::DE => {
                    let (hi, lo) = Self::split_bytes(v);
                    self.d = hi;
                    self.e = lo;
                }
                Register::HL => {
                    let (hi, lo) = Self::split_bytes(v);
                    self.h = hi;
                    self.l = lo;
                }
                _ => {
                    panic!("Attempting to load an 16 bit value to a 8 bit register")
                }
            },
        }
    }

    pub(crate) fn get(&self, register: Register) -> Value {
        match register {
            Register::A => Value::EightBit(self.a),
            Register::B => Value::EightBit(self.b),
            Register::C => Value::EightBit(self.c),
            Register::D => Value::EightBit(self.d),
            Register::E => Value::EightBit(self.e),
            Register::H => Value::EightBit(self.h),
            Register::L => Value::EightBit(self.l),
            Register::AF => Value::SixteenBit(self.af()),
            Register::BC => Value::SixteenBit(self.bc()),
            Register::DE => Value::SixteenBit(self.de()),
            Register::HL => Value::SixteenBit(self.hl()),
        }
    }

    pub(crate) fn flags(&self) -> &FlagRegister {
        &self.f
    }

    pub(crate) fn af(&self) -> u16 {
        Self::concat_bytes(self.a, self.f.get())
    }

    pub(crate) fn bc(&self) -> u16 {
        Self::concat_bytes(self.b, self.c)
    }

    pub(crate) fn de(&self) -> u16 {
        Self::concat_bytes(self.d, self.e)
    }

    pub(crate) fn hl(&self) -> u16 {
        Self::concat_bytes(self.h, self.l)
    }

    pub(crate) fn concat_bytes(hi: u8, lo: u8) -> u16 {
        (hi as u16) << 8 | lo as u16
    }

    pub(crate) fn split_bytes(value: u16) -> (u8, u8) {
        let high_byte = (value >> 8) as u8;
        let low_byte = value as u8;
        (high_byte, low_byte)
    }
}
