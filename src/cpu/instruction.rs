use crate::cpu::arithmetic::unsigned_to_signed;
use crate::cpu::flag::Flag;
use crate::cpu::flag::Flag::{C, H, N};
use crate::cpu::registers::Register::{A, PC};
use crate::cpu::value::Value;
use crate::cpu::{MemoryLocation, CPU};

pub(crate) enum RotateDirection {
    Right,
    Left,
}

pub(crate) enum JumpCondition {
    FlagOn(Flag),
    FlagOff(Flag),
    None,
}

pub(crate) struct JumpCycles {
    pub(crate) executed: u8,
    pub(crate) not_executed: u8,
}

pub(crate) enum AdditionalInstruction {
    Inc,
    Dec,
    None,
}

pub(crate) enum Instruction {
    Load {
        to: MemoryLocation,
        what: Value,
        additional_instruction: AdditionalInstruction,
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
    Inc {
        what: MemoryLocation,
        cycles: u8,
    },
    Dec {
        what: MemoryLocation,
        cycles: u8,
    },
    Rot {
        what: MemoryLocation,
        direction: RotateDirection,
        use_carry: bool,
        cycles: u8,
        length: InstructionLength,
    },
    Jr {
        how_far: Value,
        condition: JumpCondition,
        cycles: JumpCycles,
        length: InstructionLength,
    },
    Jp {
        to: Value,
        condition: JumpCondition,
        cycles: JumpCycles,
        length: InstructionLength,
    },
    Daa,
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
                additional_instruction,
                cycles,
                length,
            } => {
                match to {
                    MemoryLocation::Register(reg) => self.registers.set(reg, what),
                    MemoryLocation::Pointer(addr) => self.write(addr, what),
                };
                match additional_instruction {
                    AdditionalInstruction::Inc => {
                        if let MemoryLocation::Register(reg) = to {
                            self.registers.set(reg, self.registers.get(reg) + 1u8)
                        };
                    }
                    AdditionalInstruction::Dec => {
                        if let MemoryLocation::Register(reg) = to {
                            self.registers.set(reg, self.registers.get(reg) - 1u8)
                        };
                    }
                    AdditionalInstruction::None => {}
                }
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
            Instruction::Inc { what, cycles } => {
                match what {
                    MemoryLocation::Register(reg) => {
                        let result = if reg.is_eight_bits() {
                            self.add(self.registers.get(reg), Value::EightBit(1))
                        } else {
                            self.add(self.registers.get(reg), Value::SixteenBit(1))
                        };
                        self.registers.set(reg, result);
                    }
                    _ => panic!("NOT IMPLEMENTED!!!!"),
                };
                self.clock += cycles as u64;
                self.registers.inc_pc(1);
            }
            Instruction::Dec { what, cycles } => {
                match what {
                    MemoryLocation::Register(reg) => {
                        let result = if reg.is_eight_bits() {
                            self.sub(self.registers.get(reg), Value::EightBit(1))
                        } else {
                            self.sub(self.registers.get(reg), Value::SixteenBit(1))
                        };
                        self.registers.set(reg, result);
                    }
                    _ => panic!("NOT IMPLEMENTED!!!!"),
                };
                self.registers.f.set(N);
                self.clock += cycles as u64;
                self.registers.inc_pc(1);
            }
            Instruction::Rot {
                what,
                direction,
                use_carry,
                cycles,
                length,
            } => {
                match what {
                    MemoryLocation::Register(reg) => {
                        let val = self.registers.get(reg);
                        match direction {
                            RotateDirection::Right => {
                                let result = self.ror(val, use_carry);
                                self.registers.set(reg, result);
                            }
                            RotateDirection::Left => {
                                let result = self.rol(val, use_carry);
                                self.registers.set(reg, result);
                            }
                        }
                    }
                    MemoryLocation::Pointer(_) => {
                        panic!("NOT IMPLEMENTED")
                    }
                }
                self.clock += cycles as u64;
                self.registers.inc_pc(length.count());
            }
            Instruction::Jr {
                how_far,
                condition,
                cycles,
                length,
            } => {
                if self.should_jump(condition) {
                    let new_location = self.registers.get(PC) + unsigned_to_signed(how_far);
                    self.registers.set(PC, new_location);
                    self.clock += cycles.executed as u64;
                } else {
                    self.registers.inc_pc(length.count());
                    self.clock += cycles.not_executed as u64;
                }
            }
            Instruction::Jp {
                to,
                condition,
                cycles,
                length,
            } => {
                if self.should_jump(condition) {
                    self.registers.set(PC, to);
                    self.clock += cycles.executed as u64;
                } else {
                    self.registers.inc_pc(length.count());
                    self.clock += cycles.not_executed as u64;
                }
            }
            // TODO. I don't understand this particularly well, so this may not work as anticipated.
            Instruction::Daa => {
                let mut result = self.registers.get(A);
                // Previous operation was not subtraction
                if !self.registers.f.is_set(N) {
                    // After an addition, adjust if (half-)carry occurred or if result is out of bounds
                    if self.registers.f.is_set(C) || result > Value::EightBit(0x99) {
                        result = result + 0x60u8;
                        self.registers.f.set(C);
                    }
                    if self.registers.f.is_set(H)
                        || (result & Value::EightBit(0x0F)) > Value::EightBit(0x09)
                    {
                        result = result + 0x06u8;
                    }
                    // Previous operation was a subtraction
                } else {
                    // After a subtraction, only adjust if (half-)carry occurred
                    if self.registers.f.is_set(C) {
                        result = result - 0x60u8;
                    }
                    if self.registers.f.is_set(H) {
                        result = result - 0x06u8;
                    }
                }
                self.registers.set(A, result);
                self.check_zero_flag(result);
                self.registers.f.unset(H);
            }
            Instruction::Nop => {
                self.clock += 4;
                self.registers.inc_pc(1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::flag::Flag::{C, H, N, Z};
    use crate::cpu::instruction::RotateDirection;
    use crate::cpu::registers::Register;
    use crate::cpu::registers::Register::{A, HL, PC, SP};
    use crate::cpu::value::Value;
    use crate::cpu::MemoryLocation;

    use super::*;

    #[test]
    fn test_load_reg() {
        let mut cpu: CPU = Default::default();
        cpu.registers.set(HL, Value::SixteenBit(0x1234));

        let instruction = Instruction::Load {
            to: MemoryLocation::Register(SP),
            what: cpu.registers.get(HL),
            additional_instruction: AdditionalInstruction::None,
            cycles: 1,
            length: InstructionLength::One,
        };
        cpu.execute(instruction);

        assert_eq!(cpu.registers.get(SP), Value::SixteenBit(0x1234));
        assert!(!cpu.registers.f.is_set(C));
        assert!(!cpu.registers.f.is_set(H));
    }

    #[test]
    fn test_add() {
        let mut cpu: CPU = Default::default();

        cpu.registers.set(A, Value::EightBit(0x3E));
        cpu.registers.set(Register::B, Value::EightBit(0x23));

        let instruction = Instruction::Add {
            to: MemoryLocation::Register(A),
            what: cpu.registers.get(Register::B),
            cycles: 4,
            length: InstructionLength::One,
        };
        cpu.execute(instruction);

        assert_eq!(cpu.registers.get(A), Value::EightBit(0x61));
        assert!(!cpu.registers.f.is_set(C));
        assert!(cpu.registers.f.is_set(H));
    }

    #[test]
    fn test_adc_with_carry() {
        let mut cpu: CPU = Default::default();
        cpu.registers.f.set(C);
        cpu.registers.set(A, Value::EightBit(0x3E));
        cpu.registers.set(Register::B, Value::EightBit(0x23));
        let instruction = Instruction::Adc {
            to: MemoryLocation::Register(A),
            what: cpu.registers.get(Register::B),
            cycles: 4,
            length: InstructionLength::One,
        };
        cpu.execute(instruction);

        assert_eq!(cpu.registers.get(A), Value::EightBit(0x62));
        assert!(!cpu.registers.f.is_set(C));
        assert!(cpu.registers.f.is_set(H));
    }

    #[test]
    fn test_adc_no_carry() {
        let mut cpu: CPU = Default::default();
        cpu.registers.set(A, Value::EightBit(0x3E));
        cpu.registers.set(Register::B, Value::EightBit(0x23));
        let instruction = Instruction::Adc {
            to: MemoryLocation::Register(A),
            what: cpu.registers.get(Register::B),
            cycles: 4,
            length: InstructionLength::One,
        };
        cpu.execute(instruction);

        assert_eq!(cpu.registers.get(A), Value::EightBit(0x61));
        assert!(!cpu.registers.f.is_set(C));
        assert!(cpu.registers.f.is_set(H));
    }

    #[test]
    fn test_sub() {
        let mut cpu = CPU::default();

        cpu.registers.set(A, Value::EightBit(0xF2));
        cpu.registers.set(Register::B, Value::EightBit(0x1F));

        let instruction = Instruction::Sub {
            from: MemoryLocation::Register(A),
            what: cpu.registers.get(Register::B),
            cycles: 4,
            length: InstructionLength::One,
        };
        cpu.execute(instruction);

        assert_eq!(cpu.registers.get(A), Value::EightBit(0xD3));
        assert!(!cpu.registers.f.is_set(C));
        assert!(cpu.registers.f.is_set(H));
    }

    #[test]
    fn test_sub_to_zero() {
        let mut cpu = CPU::default();

        cpu.registers.set(A, Value::EightBit(0x50));
        cpu.registers.set(Register::B, Value::EightBit(0x50));

        let instruction = Instruction::Sub {
            from: MemoryLocation::Register(A),
            what: cpu.registers.get(Register::B),
            cycles: 4,
            length: InstructionLength::One,
        };
        cpu.execute(instruction);

        assert_eq!(cpu.registers.get(A), Value::EightBit(0x00));
        assert!(!cpu.registers.f.is_set(C));
        assert!(!cpu.registers.f.is_set(H));
        assert!(cpu.registers.f.is_set(Z));
    }

    #[test]
    fn test_sbc_with_carry() {
        let mut cpu: CPU = Default::default();
        cpu.registers.f.set(C);
        cpu.registers.set(A, Value::EightBit(0x3E));
        cpu.registers.set(Register::B, Value::EightBit(0x23));
        let instruction = Instruction::Sbc {
            from: MemoryLocation::Register(A),
            what: cpu.registers.get(Register::B),
            cycles: 4,
            length: InstructionLength::One,
        };
        cpu.execute(instruction);

        assert_eq!(cpu.registers.get(A), Value::EightBit(0x1A));
        assert!(!cpu.registers.f.is_set(C));
        assert!(!cpu.registers.f.is_set(H));
    }

    #[test]
    fn test_sbc_no_carry() {
        let mut cpu: CPU = Default::default();
        cpu.registers.set(A, Value::EightBit(0x3E));
        cpu.registers.set(Register::B, Value::EightBit(0x23));
        let instruction = Instruction::Sbc {
            from: MemoryLocation::Register(A),
            what: cpu.registers.get(Register::B),
            cycles: 4,
            length: InstructionLength::One,
        };
        cpu.execute(instruction);

        assert_eq!(cpu.registers.get(A), Value::EightBit(0x1B));
        assert!(!cpu.registers.f.is_set(C));
        assert!(!cpu.registers.f.is_set(H));
    }

    #[test]
    fn test_inc_eight_bit() {
        let mut cpu: CPU = Default::default();
        cpu.registers.set(A, Value::EightBit(0x3E));

        let instruction = Instruction::Inc {
            what: MemoryLocation::Register(A),
            cycles: 4,
        };
        cpu.execute(instruction);

        assert_eq!(cpu.registers.get(A), Value::EightBit(0x3F));
        assert!(!cpu.registers.f.is_set(N));
    }

    #[test]
    fn test_inc_sixteen_bit() {
        let mut cpu: CPU = Default::default();
        cpu.registers
            .set(Register::BC, Value::SixteenBit(0b0000_0000_1111_1111));

        let instruction = Instruction::Inc {
            what: MemoryLocation::Register(Register::BC),
            cycles: 4,
        };
        cpu.execute(instruction);

        assert_eq!(
            cpu.registers.get(Register::BC),
            Value::SixteenBit(0b0000_0001_0000_0000)
        );
        assert!(cpu.registers.f.is_set(H));
        assert!(!cpu.registers.f.is_set(N));
        assert!(!cpu.registers.f.is_set(C));
    }

    #[test]
    fn test_dec_eight_bit() {
        let mut cpu: CPU = Default::default();
        cpu.registers.set(A, Value::EightBit(0x00));

        let instruction = Instruction::Dec {
            what: MemoryLocation::Register(A),
            cycles: 4,
        };
        cpu.execute(instruction);

        assert_eq!(cpu.registers.get(A), Value::EightBit(0xFF));
        assert!(cpu.registers.f.is_set(N));
    }

    #[test]
    fn test_dec_sixteen_bit() {
        let mut cpu: CPU = Default::default();
        cpu.registers.set(Register::BC, Value::SixteenBit(0x1234));

        let instruction = Instruction::Dec {
            what: MemoryLocation::Register(Register::BC),
            cycles: 4,
        };
        cpu.execute(instruction);

        assert_eq!(cpu.registers.get(Register::BC), Value::SixteenBit(0x1233));
        assert!(cpu.registers.f.is_set(N));
    }

    #[test]
    fn test_rotate_right() {
        let mut cpu: CPU = Default::default();
        cpu.registers.set(A, Value::EightBit(0b1100_0011));
        let instruction = Instruction::Rot {
            what: MemoryLocation::Register(A),
            direction: RotateDirection::Right,
            use_carry: false,
            cycles: 4,
            length: InstructionLength::One,
        };
        cpu.execute(instruction);
        assert_eq!(cpu.registers.get(A), Value::EightBit(0b1110_0001));
        assert!(cpu.registers.f.is_set(C));
    }

    #[test]
    fn test_rotate_left() {
        let mut cpu: CPU = Default::default();
        cpu.registers.set(A, Value::EightBit(0b1100_0011));
        let instruction = Instruction::Rot {
            what: MemoryLocation::Register(A),
            direction: RotateDirection::Left,
            use_carry: false,
            cycles: 4,
            length: InstructionLength::Two,
        };
        cpu.execute(instruction);
        assert_eq!(cpu.registers.get(A), Value::EightBit(0b1000_0111));
        assert!(cpu.registers.f.is_set(C));
    }

    #[test]
    fn test_rotate_right_carry() {
        let mut cpu: CPU = Default::default();
        cpu.registers.set(A, Value::EightBit(0b1100_0010));
        cpu.registers.f.set(C);
        let instruction = Instruction::Rot {
            what: MemoryLocation::Register(A),
            direction: RotateDirection::Right,
            use_carry: true,
            cycles: 4,
            length: InstructionLength::One,
        };
        cpu.execute(instruction);
        assert_eq!(cpu.registers.get(A), Value::EightBit(0b1110_0001));
        assert!(!cpu.registers.f.is_set(C));
    }

    #[test]
    fn test_rotate_left_carry() {
        let mut cpu: CPU = Default::default();
        cpu.registers.set(A, Value::EightBit(0b0100_1000));
        cpu.registers.f.set(C);
        let instruction = Instruction::Rot {
            what: MemoryLocation::Register(A),
            direction: RotateDirection::Left,
            use_carry: true,
            cycles: 4,
            length: InstructionLength::Two,
        };
        cpu.execute(instruction);
        assert_eq!(cpu.registers.get(A), Value::EightBit(0b1001_0001));
        assert!(!cpu.registers.f.is_set(C));
    }

    #[test]
    fn test_jr() {
        let mut cpu: CPU = Default::default();
        cpu.registers.set(PC, Value::SixteenBit(0x1234));
        cpu.registers.f.set(C);

        let instruction = Instruction::Jr {
            how_far: Value::EightBit(0b1001_1001),
            condition: JumpCondition::FlagOn(C),
            cycles: JumpCycles {
                executed: 2,
                not_executed: 3,
            },
            length: InstructionLength::Two,
        };
        cpu.execute(instruction);
        assert_eq!(cpu.registers.get(PC), Value::SixteenBit(0x11CD));
        assert!(cpu.registers.f.is_set(C));
    }

    #[test]
    fn test_jp() {
        let mut cpu: CPU = Default::default();
        cpu.registers.set(PC, Value::SixteenBit(0x1234));

        let instruction = Instruction::Jp {
            to: Value::SixteenBit(0x5678),
            condition: JumpCondition::FlagOff(Z),
            cycles: JumpCycles {
                executed: 2,
                not_executed: 3,
            },
            length: InstructionLength::Two,
        };
        cpu.execute(instruction);
        assert_eq!(cpu.registers.get(PC), Value::SixteenBit(0x5678));
    }

    #[test]
    fn test_daa_after_add_1() {
        let mut cpu: CPU = Default::default();

        // ADD
        cpu.registers.set(A, Value::EightBit(0x54));
        cpu.registers.set(Register::B, Value::EightBit(0x28));

        let instruction = Instruction::Add {
            to: MemoryLocation::Register(A),
            what: cpu.registers.get(Register::B),
            cycles: 4,
            length: InstructionLength::One,
        };
        cpu.execute(instruction);

        assert_eq!(cpu.registers.get(A), Value::EightBit(0x7C));
        assert!(!cpu.registers.f.is_set(C));

        // DAA Correction
        cpu.execute(Instruction::Daa);
        assert_eq!(cpu.registers.get(A), Value::EightBit(0x82));
        assert!(!cpu.registers.f.is_set(C));
    }

    #[test]
    fn test_daa_after_add_2() {
        let mut cpu: CPU = Default::default();

        // ADD
        cpu.registers.set(A, Value::EightBit(0x98));
        cpu.registers.set(Register::B, Value::EightBit(0x04));

        let instruction = Instruction::Add {
            to: MemoryLocation::Register(A),
            what: cpu.registers.get(Register::B),
            cycles: 4,
            length: InstructionLength::One,
        };
        cpu.execute(instruction);

        assert_eq!(cpu.registers.get(A), Value::EightBit(0x9C));
        assert!(!cpu.registers.f.is_set(C));
        assert!(!cpu.registers.f.is_set(H));

        // DAA Correction
        cpu.execute(Instruction::Daa);
        assert_eq!(cpu.registers.get(A), Value::EightBit(0x02));
        assert!(cpu.registers.f.is_set(C));
        assert!(!cpu.registers.f.is_set(H));
    }

    #[test]
    fn test_daa_after_add_3() {
        let mut cpu: CPU = Default::default();

        // ADD
        cpu.registers.set(A, Value::EightBit(0x80));
        cpu.registers.set(Register::B, Value::EightBit(0x90));

        let instruction = Instruction::Add {
            to: MemoryLocation::Register(A),
            what: cpu.registers.get(Register::B),
            cycles: 4,
            length: InstructionLength::One,
        };
        cpu.execute(instruction);

        assert_eq!(cpu.registers.get(A), Value::EightBit(0x10));
        assert!(cpu.registers.f.is_set(C));

        // DAA Correction
        cpu.execute(Instruction::Daa);
        assert_eq!(cpu.registers.get(A), Value::EightBit(0x70));
    }

    #[test]
    fn test_daa_after_add_4() {
        let mut cpu: CPU = Default::default();

        // ADD
        cpu.registers.set(A, Value::EightBit(0x19));
        cpu.registers.set(Register::B, Value::EightBit(0x28));

        let instruction = Instruction::Add {
            to: MemoryLocation::Register(A),
            what: cpu.registers.get(Register::B),
            cycles: 4,
            length: InstructionLength::One,
        };
        cpu.execute(instruction);

        assert_eq!(cpu.registers.get(A), Value::EightBit(0x41));
        assert!(cpu.registers.f.is_set(H));
        assert!(!cpu.registers.f.is_set(C));

        // DAA Correction
        cpu.execute(Instruction::Daa);
        assert_eq!(cpu.registers.get(A), Value::EightBit(0x47));
    }

    #[test]
    fn test_daa_after_sub_1() {
        let mut cpu: CPU = Default::default();

        // ADD
        cpu.registers.set(A, Value::EightBit(0x47));
        cpu.registers.set(Register::B, Value::EightBit(0x28));

        let instruction = Instruction::Sub {
            from: MemoryLocation::Register(A),
            what: cpu.registers.get(Register::B),
            cycles: 4,
            length: InstructionLength::One,
        };
        cpu.execute(instruction);

        assert_eq!(cpu.registers.get(A), Value::EightBit(0x1F));
        assert!(cpu.registers.f.is_set(H));
        assert!(!cpu.registers.f.is_set(C));

        // DAA Correction
        cpu.execute(Instruction::Daa);
        assert_eq!(cpu.registers.get(A), Value::EightBit(0x19));
    }

    #[test]
    fn test_nop() {
        let mut cpu: CPU = Default::default();
        let instruction = Instruction::Nop;
        cpu.execute(instruction);

        assert_eq!(cpu.registers.get(PC), Value::SixteenBit(1));
    }
}
