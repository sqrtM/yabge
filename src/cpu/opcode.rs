use crate::cpu::instruction::{Instruction, InstructionLength};
use crate::cpu::registers::Register::{A, B, BC, PC};
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
                what: self.read(self.registers.get(PC) + Value::SixteenBit(1), true),
                cycles: 3,
                length: InstructionLength::Three,
            },
            // LD (BC), A
            0x02 => Instruction::Load {
                to: MemoryLocation::Pointer(self.registers.get(BC)),
                what: self.registers.get(A),
                cycles: 2,
                length: InstructionLength::One,
            },
            // INC BC
            0x03 => Instruction::Inc {
                what: MemoryLocation::Register(BC),
                cycles: 2,
            },
            // INC B
            0x04 => Instruction::Inc {
                what: MemoryLocation::Register(B),
                cycles: 1,
            },
            // DEC B
            0x05 => Instruction::Dec {
                what: MemoryLocation::Register(B),
                cycles: 1,
            },
            // LD B, d8
            0x06 => Instruction::Load {
                to: MemoryLocation::Register(B),
                what: self.read(self.registers.get(PC) + Value::SixteenBit(1), false),
                cycles: 2,
                length: InstructionLength::Two,
            },
            0x80 => Instruction::Add {
                to: MemoryLocation::Register(A),
                what: self.registers.get(B),
                cycles: 4,
                length: InstructionLength::One,
            },
            _ => Instruction::Nop,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::registers::Register::{A, B, BC};
    use crate::cpu::value::Value;
    use crate::cpu::CPU;

    #[test]
    fn test_0x01() {
        let mut cpu: CPU = Default::default();
        cpu.write(Value::SixteenBit(0x00), Value::EightBit(0x01));
        cpu.write(Value::SixteenBit(0x01), Value::EightBit(0xCD));
        cpu.write(Value::SixteenBit(0x02), Value::EightBit(0xAB));

        let val = cpu.read(Value::SixteenBit(0x00), false);
        if let Value::EightBit(code) = val {
            let inst = cpu.lookup(code);
            cpu.execute(inst);
        }

        assert_eq!(cpu.registers.get(BC), Value::SixteenBit(0xABCD));
    }

    #[test]
    fn test_0x02() {
        let mut cpu: CPU = Default::default();
        cpu.registers.set(A, Value::EightBit(0xEF));
        cpu.registers.set(BC, Value::SixteenBit(0x1234));

        cpu.write(Value::SixteenBit(0x00), Value::EightBit(0x02));

        let val = cpu.read(Value::SixteenBit(0x00), false);
        if let Value::EightBit(code) = val {
            let inst = cpu.lookup(code);
            cpu.execute(inst);
        }

        assert_eq!(
            cpu.read(Value::SixteenBit(0x1234), false),
            Value::EightBit(0xEF)
        );
    }

    #[test]
    fn test_0x03() {
        let mut cpu: CPU = Default::default();
        cpu.registers.set(BC, Value::SixteenBit(0x1234));

        cpu.write(Value::SixteenBit(0x00), Value::EightBit(0x03));

        let val = cpu.read(Value::SixteenBit(0x00), false);
        if let Value::EightBit(code) = val {
            let inst = cpu.lookup(code);
            cpu.execute(inst);
        }

        assert_eq!(cpu.registers.get(BC), Value::SixteenBit(0x1235));
    }

    #[test]
    fn test_0x06() {
        let mut cpu: CPU = Default::default();

        cpu.write(Value::SixteenBit(0x00), Value::EightBit(0x06));
        cpu.write(Value::SixteenBit(0x01), Value::EightBit(0xAB));

        let val = cpu.read(Value::SixteenBit(0x00), false);
        if let Value::EightBit(code) = val {
            let inst = cpu.lookup(code);
            cpu.execute(inst);
        }

        assert_eq!(cpu.registers.get(B), Value::EightBit(0xAB));
    }
}
