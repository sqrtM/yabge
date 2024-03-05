use crate::cpu::flag::Flag::Z;
use crate::cpu::instruction::{
    AdditionalInstruction, Instruction, InstructionLength, JumpCondition, JumpCycles,
    RotateDirection,
};
use crate::cpu::registers::Register::{A, B, BC, C, D, DE, E, H, HL, SP};
use crate::cpu::{MemoryLocation, CPU};

impl CPU {
    pub fn lookup(&self, code: u8) -> Instruction {
        match code {
            // NOP
            0x00 => Instruction::Nop,
            // LD BC, d16
            0x01 => Instruction::Load {
                to: MemoryLocation::Register(BC),
                what: self.immediate_operand(true),
                additional_instruction: AdditionalInstruction::None,
                cycles: 3,
                length: InstructionLength::Three,
            },
            // LD (BC), A
            0x02 => Instruction::Load {
                to: MemoryLocation::Pointer(self.registers.get(BC)),
                what: self.registers.get(A),
                additional_instruction: AdditionalInstruction::None,
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
                what: self.immediate_operand(false),
                additional_instruction: AdditionalInstruction::None,
                cycles: 2,
                length: InstructionLength::Two,
            },
            // RLCA
            0x07 => Instruction::Rot {
                what: MemoryLocation::Register(A),
                direction: RotateDirection::Left,
                use_carry: false,
                cycles: 1,
                length: InstructionLength::One,
            },
            // LD (a16), SP
            0x08 => Instruction::Load {
                to: MemoryLocation::Pointer(self.immediate_operand(true)),
                what: self.registers.get(SP),
                additional_instruction: AdditionalInstruction::None,
                cycles: 5,
                length: InstructionLength::Three,
            },
            // ADD HL, BC
            0x09 => Instruction::Add {
                to: MemoryLocation::Register(HL),
                what: self.registers.get(BC),
                cycles: 2,
                length: InstructionLength::One,
            },
            // LD A, (BC)
            0x0A => Instruction::Load {
                to: MemoryLocation::Register(A),
                what: self.read(self.registers.get(BC), false),
                additional_instruction: AdditionalInstruction::None,
                cycles: 2,
                length: InstructionLength::One,
            },
            // DEC BC
            0x0B => Instruction::Dec {
                what: MemoryLocation::Register(BC),
                cycles: 2,
            },
            // INC C
            0x0C => Instruction::Inc {
                what: MemoryLocation::Register(C),
                cycles: 1,
            },
            // DEC C
            0x0D => Instruction::Dec {
                what: MemoryLocation::Register(C),
                cycles: 1,
            },
            // LD C, d8
            0x0E => Instruction::Load {
                to: MemoryLocation::Register(C),
                what: self.immediate_operand(false),
                additional_instruction: AdditionalInstruction::None,
                cycles: 2,
                length: InstructionLength::Two,
            },
            // RRCA
            0x0F => Instruction::Rot {
                what: MemoryLocation::Register(A),
                direction: RotateDirection::Right,
                use_carry: false,
                cycles: 1,
                length: InstructionLength::One,
            },
            // STOP
            0x10 => todo!("Stop is not yet implemented."),
            // LD DE, d16
            0x11 => Instruction::Load {
                to: MemoryLocation::Register(DE),
                what: self.immediate_operand(true),
                additional_instruction: AdditionalInstruction::None,
                cycles: 3,
                length: InstructionLength::Three,
            },
            // LD (DE), A
            0x12 => Instruction::Load {
                to: MemoryLocation::Pointer(self.registers.get(DE)),
                what: self.registers.get(A),
                additional_instruction: AdditionalInstruction::None,
                cycles: 2,
                length: InstructionLength::One,
            },
            // INC DE
            0x13 => Instruction::Inc {
                what: MemoryLocation::Register(DE),
                cycles: 2,
            },
            // INC D
            0x14 => Instruction::Inc {
                what: MemoryLocation::Register(D),
                cycles: 1,
            },
            // DEC D
            0x15 => Instruction::Dec {
                what: MemoryLocation::Register(D),
                cycles: 1,
            },
            // LD D, d8
            0x16 => Instruction::Load {
                to: MemoryLocation::Register(D),
                what: self.immediate_operand(false),
                additional_instruction: AdditionalInstruction::None,
                cycles: 2,
                length: InstructionLength::Two,
            },
            // RLA
            0x17 => Instruction::Rot {
                what: MemoryLocation::Register(A),
                direction: RotateDirection::Left,
                use_carry: true,
                cycles: 1,
                length: InstructionLength::One,
            },
            // JR s8
            0x18 => Instruction::Jr {
                how_far: self.immediate_operand(false),
                condition: JumpCondition::None,
                cycles: JumpCycles {
                    executed: 3,
                    not_executed: 3,
                },
                length: InstructionLength::Two,
            },
            // ADD HL, DE
            0x19 => Instruction::Add {
                to: MemoryLocation::Register(HL),
                what: self.registers.get(DE),
                cycles: 2,
                length: InstructionLength::One,
            },
            // LD A, (DE)
            0x1A => Instruction::Load {
                to: MemoryLocation::Register(A),
                what: self.read(self.registers.get(DE), false),
                additional_instruction: AdditionalInstruction::None,
                cycles: 2,
                length: InstructionLength::One,
            },
            // DEC DE
            0x1B => Instruction::Dec {
                what: MemoryLocation::Register(DE),
                cycles: 2,
            },
            // INC E
            0x1C => Instruction::Inc {
                what: MemoryLocation::Register(E),
                cycles: 2,
            },
            // DEC E
            0x1D => Instruction::Dec {
                what: MemoryLocation::Register(E),
                cycles: 1,
            },
            // LD E, d8
            0x1E => Instruction::Load {
                to: MemoryLocation::Register(E),
                what: self.immediate_operand(false),
                additional_instruction: AdditionalInstruction::None,
                cycles: 2,
                length: InstructionLength::Two,
            },
            // RRA
            0x1F => Instruction::Rot {
                what: MemoryLocation::Register(A),
                direction: RotateDirection::Right,
                use_carry: true,
                cycles: 1,
                length: InstructionLength::One,
            },
            // JR NZ, s8
            0x20 => Instruction::Jr {
                how_far: self.immediate_operand(false),
                condition: JumpCondition::FlagOff(Z),
                cycles: JumpCycles {
                    executed: 3,
                    not_executed: 2,
                },
                length: InstructionLength::Two,
            },
            // LD HL, d16
            0x21 => Instruction::Load {
                to: MemoryLocation::Register(HL),
                what: self.immediate_operand(true),
                additional_instruction: AdditionalInstruction::None,
                cycles: 3,
                length: InstructionLength::Three,
            },
            // LD (HL+), 1
            0x22 => Instruction::Load {
                to: MemoryLocation::Pointer(self.registers.get(HL)),
                what: self.registers.get(A),
                additional_instruction: AdditionalInstruction::Inc,
                cycles: 2,
                length: InstructionLength::One,
            },
            // INC HL
            0x23 => Instruction::Inc {
                what: MemoryLocation::Register(HL),
                cycles: 2,
            },
            // INC H
            0x24 => Instruction::Inc {
                what: MemoryLocation::Register(H),
                cycles: 1,
            },
            // DEC H
            0x25 => Instruction::Dec {
                what: MemoryLocation::Register(H),
                cycles: 1,
            },
            // LD H, d8
            0x26 => Instruction::Load {
                to: MemoryLocation::Register(H),
                what: self.immediate_operand(false),
                additional_instruction: AdditionalInstruction::None,
                cycles: 2,
                length: InstructionLength::Two,
            },
            // DAA
            0x27 => Instruction::Daa,
            // JR Z, s8
            0x28 => Instruction::Jr {
                how_far: self.immediate_operand(false),
                condition: JumpCondition::FlagOn(Z),
                cycles: JumpCycles {
                    executed: 3,
                    not_executed: 2,
                },
                length: InstructionLength::Two,
            },
            // ADD HL, HL
            0x29 => Instruction::Add {
                to: MemoryLocation::Register(HL),
                what: self.registers.get(HL),
                cycles: 2,
                length: InstructionLength::One,
            },
            // LD A, (HL+)
            0x30 => Instruction::Load {
                to: MemoryLocation::Register(A),
                what: self.read(self.registers.get(HL), false),
                additional_instruction: AdditionalInstruction::Inc,
                cycles: 2,
                length: InstructionLength::One,
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
    use crate::cpu::flag::Flag;
    use crate::cpu::flag::Flag::Z;
    use crate::cpu::registers::Register::{A, B, BC, C, DE, HL, PC, SP};
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

    #[test]
    fn test_0x07() {
        let mut cpu: CPU = Default::default();

        cpu.registers.set(A, Value::EightBit(0b10010010));
        cpu.write(Value::SixteenBit(0x00), Value::EightBit(0x07));

        let val = cpu.read(Value::SixteenBit(0x00), false);
        if let Value::EightBit(code) = val {
            let inst = cpu.lookup(code);
            cpu.execute(inst);
        }

        assert_eq!(cpu.registers.get(A), Value::EightBit(0b0010_0101));
        assert!(cpu.registers.f.is_set(Flag::C));
    }

    #[test]
    fn test_0x08() {
        let mut cpu: CPU = Default::default();
        cpu.registers.set(SP, Value::SixteenBit(0xABCD));

        cpu.write(Value::SixteenBit(0x00), Value::EightBit(0x08));
        cpu.write(Value::SixteenBit(0x01), Value::EightBit(0x34));
        cpu.write(Value::SixteenBit(0x02), Value::EightBit(0x12));

        let val = cpu.read(Value::SixteenBit(0x00), false);
        if let Value::EightBit(code) = val {
            let inst = cpu.lookup(code);
            cpu.execute(inst);
        }

        assert_eq!(
            cpu.read(Value::SixteenBit(0x1234), false),
            Value::EightBit(0xCD)
        );
        assert_eq!(
            cpu.read(Value::SixteenBit(0x1235), false),
            Value::EightBit(0xAB)
        );
        assert_eq!(
            cpu.read(Value::SixteenBit(0x1234), true),
            Value::SixteenBit(0xABCD)
        );
    }

    #[test]
    fn test_0x09() {
        let mut cpu: CPU = Default::default();

        cpu.registers.set(BC, Value::SixteenBit(0x2211));
        cpu.registers.set(HL, Value::SixteenBit(0x2211));
        cpu.write(Value::SixteenBit(0x00), Value::EightBit(0x09));

        let val = cpu.read(Value::SixteenBit(0x00), false);
        if let Value::EightBit(code) = val {
            let inst = cpu.lookup(code);
            cpu.execute(inst);
        }

        assert_eq!(cpu.registers.get(HL), Value::SixteenBit(0x4422));
        assert_eq!(cpu.registers.get(BC), Value::SixteenBit(0x2211));
    }

    #[test]
    fn test_0x0a() {
        let mut cpu: CPU = Default::default();
        cpu.registers.set(A, Value::EightBit(0xAA));
        cpu.registers.set(BC, Value::SixteenBit(0x1234));
        cpu.write(Value::SixteenBit(0x00), Value::EightBit(0x0A));
        cpu.write(Value::SixteenBit(0x1234), Value::EightBit(0xFF));

        let val = cpu.read(Value::SixteenBit(0x00), false);
        if let Value::EightBit(code) = val {
            let inst = cpu.lookup(code);
            cpu.execute(inst);
        }

        assert_eq!(cpu.registers.get(A), Value::EightBit(0xFF));
    }

    #[test]
    fn test_0x0b() {
        let mut cpu: CPU = Default::default();
        cpu.registers.set(BC, Value::SixteenBit(0x1234));

        cpu.write(Value::SixteenBit(0x00), Value::EightBit(0x0B));

        let val = cpu.read(Value::SixteenBit(0x00), false);
        if let Value::EightBit(code) = val {
            let inst = cpu.lookup(code);
            cpu.execute(inst);
        }

        assert_eq!(cpu.registers.get(BC), Value::SixteenBit(0x1233));
    }

    #[test]
    fn test_0x0c() {
        let mut cpu: CPU = Default::default();
        cpu.registers.set(C, Value::EightBit(0x12));

        cpu.write(Value::SixteenBit(0x00), Value::EightBit(0x0C));

        let val = cpu.read(Value::SixteenBit(0x00), false);
        if let Value::EightBit(code) = val {
            let inst = cpu.lookup(code);
            cpu.execute(inst);
        }

        assert_eq!(cpu.registers.get(C), Value::EightBit(0x13));
    }

    #[test]
    fn test_0x0f() {
        let mut cpu: CPU = Default::default();

        cpu.registers.set(A, Value::EightBit(0b1001_0011));
        cpu.write(Value::SixteenBit(0x00), Value::EightBit(0x0F));

        let val = cpu.read(Value::SixteenBit(0x00), false);
        if let Value::EightBit(code) = val {
            let inst = cpu.lookup(code);
            cpu.execute(inst);
        }

        assert_eq!(cpu.registers.get(A), Value::EightBit(0b1100_1001));
        assert!(cpu.registers.f.is_set(Flag::C));
    }

    #[test]
    fn test_0x11() {
        let mut cpu: CPU = Default::default();
        cpu.write(Value::SixteenBit(0x00), Value::EightBit(0x11));
        cpu.write(Value::SixteenBit(0x01), Value::EightBit(0xCD));
        cpu.write(Value::SixteenBit(0x02), Value::EightBit(0xAB));

        let val = cpu.read(Value::SixteenBit(0x00), false);
        if let Value::EightBit(code) = val {
            let inst = cpu.lookup(code);
            cpu.execute(inst);
        }

        assert_eq!(cpu.registers.get(DE), Value::SixteenBit(0xABCD));
    }

    #[test]
    fn test_0x12() {
        let mut cpu: CPU = Default::default();
        cpu.registers.set(A, Value::EightBit(0xAB));
        cpu.registers.set(DE, Value::SixteenBit(0x1234));
        cpu.write(Value::SixteenBit(0x00), Value::EightBit(0x12));

        let val = cpu.read(Value::SixteenBit(0x00), false);
        if let Value::EightBit(code) = val {
            let inst = cpu.lookup(code);
            cpu.execute(inst);
        }

        assert_eq!(
            cpu.read(Value::SixteenBit(0x1234), false),
            Value::EightBit(0xAB)
        );
    }

    #[test]
    fn test_0x17() {
        let mut cpu: CPU = Default::default();
        cpu.registers.set(A, Value::EightBit(0b0010_1000));
        cpu.registers.f.set(Flag::C);
        cpu.write(Value::SixteenBit(0x00), Value::EightBit(0x17));

        let val = cpu.read(Value::SixteenBit(0x00), false);
        if let Value::EightBit(code) = val {
            let inst = cpu.lookup(code);
            cpu.execute(inst);
        }

        assert_eq!(cpu.registers.get(A), Value::EightBit(0b0101_0001));
    }

    #[test]
    fn test_0x18() {
        let mut cpu: CPU = Default::default();
        cpu.write(Value::SixteenBit(0x00), Value::EightBit(0x18));
        cpu.write(Value::SixteenBit(0x01), Value::EightBit(0xAB));
        assert_eq!(0xAB, 0b10101011);
        assert_eq!(0xAB, 171u8);
        assert_eq!(0b10101011u8 as i8, -85);
        assert_eq!(0u16.wrapping_sub(85), 65451);

        let val = cpu.read(Value::SixteenBit(0x00), false);
        if let Value::EightBit(code) = val {
            let inst = cpu.lookup(code);
            cpu.execute(inst);
        }

        assert_eq!(cpu.registers.get(PC), Value::SixteenBit(65451));
    }

    #[test]
    fn test_0x1f() {
        let mut cpu: CPU = Default::default();
        cpu.registers.set(A, Value::EightBit(0b0010_1000));
        cpu.registers.f.set(Flag::C);
        cpu.write(Value::SixteenBit(0x00), Value::EightBit(0x1F));

        let val = cpu.read(Value::SixteenBit(0x00), false);
        if let Value::EightBit(code) = val {
            let inst = cpu.lookup(code);
            cpu.execute(inst);
        }

        assert_eq!(cpu.registers.get(A), Value::EightBit(0b1001_0100));
    }

    #[test]
    fn test_0x20() {
        let mut cpu: CPU = Default::default();
        cpu.write(Value::SixteenBit(0x00), Value::EightBit(0x20));
        cpu.write(Value::SixteenBit(0x01), Value::EightBit(0x10));

        let val = cpu.read(Value::SixteenBit(0x00), false);
        if let Value::EightBit(code) = val {
            let inst = cpu.lookup(code);
            cpu.execute(inst);
        }

        assert_eq!(cpu.registers.get(PC), Value::SixteenBit(0x10));

        let mut cpu2: CPU = Default::default();
        cpu2.registers.f.set(Z);
        cpu2.write(Value::SixteenBit(0x00), Value::EightBit(0x20));
        cpu2.write(Value::SixteenBit(0x01), Value::EightBit(0x10));

        let val = cpu2.read(Value::SixteenBit(0x00), false);
        if let Value::EightBit(code) = val {
            let inst = cpu2.lookup(code);
            cpu2.execute(inst);
        }

        assert_eq!(cpu2.registers.get(PC), Value::SixteenBit(0x02));
    }

    #[test]
    fn test_0x21() {
        let mut cpu: CPU = Default::default();
        cpu.write(Value::SixteenBit(0x00), Value::EightBit(0x21));
        cpu.write(Value::SixteenBit(0x01), Value::SixteenBit(0xABCD));

        let val = cpu.read(Value::SixteenBit(0x00), false);
        if let Value::EightBit(code) = val {
            let inst = cpu.lookup(code);
            cpu.execute(inst);
        }

        assert_eq!(cpu.registers.get(HL), Value::SixteenBit(0xABCD));
    }

    #[test]
    fn test_0x22() {
        let mut cpu: CPU = Default::default();
        cpu.write(Value::SixteenBit(0x00), Value::EightBit(0x22));
        cpu.registers.set(A, Value::EightBit(0xAB));
        cpu.registers.set(HL, Value::SixteenBit(0xCAFE));

        let val = cpu.read(Value::SixteenBit(0x00), false);
        if let Value::EightBit(code) = val {
            let inst = cpu.lookup(code);
            cpu.execute(inst);
        }

        assert_eq!(
            cpu.read(Value::SixteenBit(0xCAFE), false),
            Value::EightBit(0xAB)
        );
    }

    #[test]
    fn test_0x29() {
        let mut cpu: CPU = Default::default();
        cpu.write(Value::SixteenBit(0x00), Value::SixteenBit(0x29));
        cpu.registers.set(HL, Value::SixteenBit(0xAB));

        let val = cpu.read(Value::SixteenBit(0x00), false);
        if let Value::EightBit(code) = val {
            let inst = cpu.lookup(code);
            cpu.execute(inst);
        }

        assert_eq!(cpu.registers.get(HL), Value::SixteenBit(0x0156));
    }

    #[test]
    fn test_0x30() {
        let mut cpu: CPU = Default::default();
        cpu.write(Value::SixteenBit(0x0000), Value::EightBit(0x30));
        cpu.write(Value::SixteenBit(0xABCD), Value::EightBit(0xFE));
        cpu.registers.set(HL, Value::SixteenBit(0xABCD));

        let val = cpu.read(Value::SixteenBit(0x00), false);
        if let Value::EightBit(code) = val {
            let inst = cpu.lookup(code);
            cpu.execute(inst);
        }

        assert_eq!(cpu.registers.get(A), Value::EightBit(0xFE));
    }
}
