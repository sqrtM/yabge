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
    use crate::cpu::flag::Flag::{C, H};
    use crate::cpu::registers::Register::{A, HL, SP};
    use crate::cpu::value::Value;
    use crate::cpu::{registers, MemoryLocation};

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

        assert_eq!(cpu.registers.get(Register::A), Value::EightBit(0xD3));
        assert!(!cpu.registers.f.is_set(C));
        assert!(cpu.registers.f.is_set(H));
    }

    #[test]
    fn test_sbc_with_carry() {
        let mut cpu: CPU = Default::default();
        cpu.registers.f.set(C);
        cpu.registers.set(Register::A, Value::EightBit(0x3E));
        cpu.registers.set(Register::B, Value::EightBit(0x23));
        let instruction = Instruction::Sbc {
            from: MemoryLocation::Register(A),
            what: cpu.registers.get(Register::B),
            cycles: 4,
            length: InstructionLength::One,
        };
        cpu.execute(instruction);

        assert_eq!(cpu.registers.get(Register::A), Value::EightBit(0x1A));
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

        assert_eq!(cpu.registers.get(Register::A), Value::EightBit(0x1B));
        assert!(!cpu.registers.f.is_set(C));
        assert!(!cpu.registers.f.is_set(H));
    }
}
