use crate::cpu::registers::Register;
use crate::cpu::value::Value;
use crate::cpu::CPU;

pub(crate) enum Instruction {
    Load {
        to: Register,
        value: Value,
        cycles: u8,
        length: InstructionLength,
    },
    Add {
        to: Register,
        value: Value,
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
            Instruction::Load { .. } => {}
            Instruction::Add {
                to,
                value,
                cycles,
                length,
            } => {
                let result = self.registers.get(to) + value;
                self.registers.set(to, result);
                self.clock += cycles as u64;
                self.program_counter += length.count();
            }
            _ => {}
        }
    }
}
