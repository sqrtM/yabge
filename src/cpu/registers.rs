use crate::cpu::flag::FlagRegister;
use crate::cpu::value::Value;
use crate::cpu::{concat_bytes, split_bytes};

#[derive(Clone, Copy, Debug)]
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
    SP,
    PC,
}

impl Register {
    pub fn is_eight_bits(&self) -> bool {
        match self {
            Register::A => true,
            Register::B => true,
            Register::C => true,
            Register::D => true,
            Register::E => true,
            Register::H => true,
            Register::L => true,
            Register::AF => false,
            Register::BC => false,
            Register::DE => false,
            Register::HL => false,
            Register::SP => false,
            Register::PC => false,
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
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
    pub fn set(&mut self, register: Register, value: Value) {
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
                    let (hi, lo) = split_bytes(v);
                    self.a = hi;
                    self.f.overwrite(lo);
                }
                Register::BC => {
                    let (hi, lo) = split_bytes(v);
                    self.b = hi;
                    self.c = lo;
                }
                Register::DE => {
                    let (hi, lo) = split_bytes(v);
                    self.d = hi;
                    self.e = lo;
                }
                Register::HL => {
                    let (hi, lo) = split_bytes(v);
                    self.h = hi;
                    self.l = lo;
                }
                Register::SP => self.sp = v,
                Register::PC => self.pc = v,
                _ => {
                    panic!("Attempting to load an 16 bit value to a 8 bit register")
                }
            },
        }
    }

    pub fn get(&self, register: Register) -> Value {
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
            Register::SP => Value::SixteenBit(self.sp),
            Register::PC => Value::SixteenBit(self.pc),
        }
    }

    pub fn flags(&self) -> &FlagRegister {
        &self.f
    }

    pub fn inc_pc(&mut self, value: u16) {
        self.pc += value;
    }

    pub fn af(&self) -> u16 {
        concat_bytes(self.a, self.f.get())
    }

    pub fn bc(&self) -> u16 {
        concat_bytes(self.b, self.c)
    }

    pub fn de(&self) -> u16 {
        concat_bytes(self.d, self.e)
    }

    pub fn hl(&self) -> u16 {
        concat_bytes(self.h, self.l)
    }
}
