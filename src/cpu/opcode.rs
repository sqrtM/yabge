use crate::cpu::instruction::{Instruction, InstructionLength};
use crate::cpu::registers::Register;
use crate::cpu::registers::Register::{A, BC, SP};
use crate::cpu::value::Value;
use crate::cpu::{MemoryLocation, CPU};

impl CPU {
    pub fn lookup(&self, code: u8) -> Instruction {
        match code {
            // NOP
            0x00 => Instruction::Nop,
            // LD BC, d16
            0x01 => Instruction::Load {
                to: MemoryLocation::Register(BC),
                what: self.read(self.registers.get(SP) + Value::SixteenBit(1), true),
                cycles: 3,
                length: InstructionLength::Three,
            },
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
mod test {
    use crate::cpu::registers::Register::BC;
    use crate::cpu::value::Value;
    use crate::cpu::CPU;

    #[test]
    fn test_0x01() {
        let mut cpu: CPU = Default::default();
        cpu.write(Value::SixteenBit(0x00), 0x01);
        cpu.write(Value::SixteenBit(0x01), 0xCD);
        cpu.write(Value::SixteenBit(0x02), 0xAB);

        let val = cpu.read(Value::SixteenBit(0x00), false);
        if let Value::EightBit(code) = val {
            let inst = cpu.lookup(code);
            cpu.execute(inst);
        }

        assert_eq!(cpu.registers.get(BC), Value::SixteenBit(0xABCD));
    }
}
