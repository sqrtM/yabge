use crate::cpu::instruction::{Instruction, InstructionLength};
use crate::cpu::registers::Register;
use crate::cpu::CPU;

impl CPU {
    pub fn lookup(&self, code: u8) -> Instruction {
        match code {
            0x80 => Instruction::Add {
                to: Register::A,
                value: self.registers.get(Register::B),
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
    use crate::cpu::value::Value;

    #[test]
    fn test_add() {
        let mut cpu: CPU = Default::default();

        cpu.registers.set(Register::A, Value::EightBit(0x10));
        cpu.registers.set(Register::B, Value::EightBit(0x20));

        let instruction = Instruction::Add {
            to: Register::A,
            value: cpu.registers.get(Register::B),
            cycles: 4,
            length: InstructionLength::One,
        };
        cpu.execute(instruction);

        assert_eq!(cpu.registers.get(Register::A), Value::EightBit(0x30));
        assert!(!cpu.registers.f.is_set(C)); // No carry expected
        assert!(!cpu.registers.f.is_set(H)); // No half carry expected
    }
}
