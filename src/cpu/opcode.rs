use crate::cpu::instruction::{Instruction, InstructionLength};
use crate::cpu::registers::Register;
use crate::cpu::registers::Register::A;
use crate::cpu::{MemoryLocation, CPU};

impl CPU {
    pub fn lookup(&self, code: u8) -> Instruction {
        match code {
            0x80 => Instruction::Add {
                to: MemoryLocation::Register(A),
                what: self.registers.get(Register::B),
                cycles: 4,
                length: InstructionLength::One,
            },
            _ => Instruction::Nop,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::flag::Flag::{C, H, N};
    use crate::cpu::instruction::RotateDirection;
    use crate::cpu::registers::Register::{A, HL, PC, SP};
    use crate::cpu::value::Value;
    use crate::cpu::MemoryLocation;

    #[test]
    fn test_load() {
        let mut cpu = CPU {
            registers: Default::default(),
            clock: 0,
        };
        cpu.registers.set(HL, Value::SixteenBit(0x1234));

        let instruction = Instruction::Load {
            to: MemoryLocation::Register(SP),
            what: cpu.registers.get(HL),
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
        cpu.registers.set(Register::BC, Value::SixteenBit(0x1234));

        let instruction = Instruction::Inc {
            what: MemoryLocation::Register(Register::BC),
            cycles: 4,
        };
        cpu.execute(instruction);

        assert_eq!(cpu.registers.get(Register::BC), Value::SixteenBit(0x1235));
        assert!(!cpu.registers.f.is_set(N));
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
    fn test_nop() {
        let mut cpu: CPU = Default::default();
        let instruction = Instruction::Nop;
        cpu.execute(instruction);

        assert_eq!(cpu.registers.get(PC), Value::SixteenBit(1));
    }
}
