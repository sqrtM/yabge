use crate::cpu::flag::Flag::{C, H, N};
use crate::cpu::value::Value;
use crate::cpu::{MemoryLocation, CPU};

pub(crate) enum Instruction {
    Load {
        to: MemoryLocation,
        what: Value,
        cycles: u8,
        length: InstructionLength,
    },
    Add {
        to: MemoryLocation,
        what: Value,
        cycles: u8,
        length: InstructionLength,
    },
    Adc {
        to: MemoryLocation,
        what: Value,
        cycles: u8,
        length: InstructionLength,
    },
    Sub {
        from: MemoryLocation,
        what: Value,
        cycles: u8,
        length: InstructionLength,
    },
    Sbc {
        from: MemoryLocation,
        what: Value,
        cycles: u8,
        length: InstructionLength,
    },
    Nop,
}

pub(crate) enum InstructionLength {
    One,
    Two,
    Three,
}

impl InstructionLength {
    pub fn count(&self) -> u16 {
        match self {
            InstructionLength::One => 1,
            InstructionLength::Two => 2,
            InstructionLength::Three => 3,
        }
    }
}

impl CPU {
    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Load {
                to,
                what,
                cycles,
                length,
            } => {
                // Does this work when loading into ram ?
                match to {
                    MemoryLocation::Register(reg) => {
                        self.registers.set(reg, what);
                    }
                    _ => panic!("NOT IMPLEMENTED!!!!"),
                };
                self.clock += cycles as u64;
                self.registers.inc_pc(length.count());
            }
            Instruction::Add {
                to,
                what,
                cycles,
                length,
            } => {
                match to {
                    MemoryLocation::Register(reg) => {
                        let result = self.add(self.registers.get(reg), what);
                        self.registers.set(reg, result);
                    }
                    _ => panic!("NOT IMPLEMENTED!!!!"),
                };
                self.registers.f.unset(N);
                self.clock += cycles as u64;
                self.registers.inc_pc(length.count());
            }
            Instruction::Adc {
                to,
                what,
                cycles,
                length,
            } => {
                let carry_flag_value = if self.registers.f.is_set(C) {
                    Value::EightBit(1)
                } else {
                    Value::EightBit(0)
                };
                let operand_with_carry = self.add(what, carry_flag_value);
                match to {
                    MemoryLocation::Register(reg) => {
                        let result = self.add(self.registers.get(reg), operand_with_carry);
                        self.registers.set(reg, result);
                    }
                    _ => panic!("NOT IMPLEMENTED!!!!"),
                };
                self.registers.f.unset(N);
                self.clock += cycles as u64;
                self.registers.inc_pc(length.count());
            }
            Instruction::Sub {
                from,
                what,
                cycles,
                length,
            } => {
                match from {
                    MemoryLocation::Register(reg) => {
                        let result = self.sub(self.registers.get(reg), what);
                        self.registers.set(reg, result);
                    }
                    _ => panic!("NOT IMPLEMENTED!!!!"),
                };
                self.clock += cycles as u64;
                self.registers.inc_pc(length.count());
            }
            Instruction::Sbc {
                from,
                what,
                cycles,
                length,
            } => {
                let carry_flag_value = if self.registers.f.is_set(C) {
                    Value::EightBit(1)
                } else {
                    Value::EightBit(0)
                };
                let operand_with_carry = self.add(what, carry_flag_value);
                match from {
                    MemoryLocation::Register(reg) => {
                        let result = self.sub(self.registers.get(reg), operand_with_carry);
                        self.registers.set(reg, result);
                    }
                    _ => panic!("NOT IMPLEMENTED!!!!"),
                };
                self.registers.f.set(N);
                self.clock += cycles as u64;
                self.registers.inc_pc(length.count());
            }
            _ => {}
        }
    }

    fn add(&mut self, a: Value, b: Value) -> Value {
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

    fn sub(&mut self, a: Value, b: Value) -> Value {
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
}
