use crate::cpu::flag::Flag;
use crate::cpu::flag::Flag::Z;
use crate::cpu::instruction::{
    AdditionalInstruction, Instruction, InstructionLength, JumpCondition, JumpCycles,
    RotateDirection,
};
use crate::cpu::registers::Register::{A, B, BC, C, D, DE, E, H, HL, L, SP};
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
            // LD (HL+), A
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
            0x2A => Instruction::Load {
                to: MemoryLocation::Register(A),
                what: self.read(self.registers.get(HL), false),
                additional_instruction: AdditionalInstruction::Inc,
                cycles: 2,
                length: InstructionLength::One,
            },
            // DEC HL
            0x2B => Instruction::Dec {
                what: MemoryLocation::Register(HL),
                cycles: 2,
            },
            // INC L
            0x2C => Instruction::Inc {
                what: MemoryLocation::Register(L),
                cycles: 1,
            },
            // DEC L
            0x2D => Instruction::Dec {
                what: MemoryLocation::Register(L),
                cycles: 1,
            },
            // LD L, d8
            0x2E => Instruction::Load {
                to: MemoryLocation::Register(L),
                what: self.immediate_operand(false),
                additional_instruction: AdditionalInstruction::None,
                cycles: 2,
                length: InstructionLength::Two,
            },
            // CPL
            0x2F => Instruction::Cpl,
            // JP NC, s8
            0x30 => Instruction::Jr {
                how_far: self.immediate_operand(false),
                condition: JumpCondition::FlagOff(Flag::C),
                cycles: JumpCycles {
                    executed: 3,
                    not_executed: 2,
                },
                length: InstructionLength::Two,
            },
            // LD SP, d16
            0x31 => Instruction::Load {
                to: MemoryLocation::Register(SP),
                what: self.immediate_operand(true),
                additional_instruction: AdditionalInstruction::None,
                cycles: 3,
                length: InstructionLength::Three,
            },
            // LD (HL-), A
            0x32 => Instruction::Load {
                to: MemoryLocation::Pointer(self.registers.get(HL)),
                what: self.registers.get(A),
                additional_instruction: AdditionalInstruction::Dec,
                cycles: 2,
                length: InstructionLength::One,
            },
            // INC SP
            0x33 => Instruction::Inc {
                what: MemoryLocation::Register(SP),
                cycles: 2,
            },
            // INC HL
            0x34 => Instruction::Inc {
                what: MemoryLocation::Register(HL),
                cycles: 2,
            },
            // DEC HL
            0x35 => Instruction::Dec {
                what: MemoryLocation::Register(HL),
                cycles: 2,
            },
            // LD (HL), d8
            0x36 => Instruction::Load {
                to: MemoryLocation::Pointer(self.registers.get(HL)),
                what: self.immediate_operand(false),
                additional_instruction: AdditionalInstruction::None,
                cycles: 3,
                length: InstructionLength::Two,
            },
            // SCF
            0x37 => Instruction::Scf,
            // JR C, s8
            0x38 => Instruction::Jr {
                how_far: self.immediate_operand(false),
                condition: JumpCondition::FlagOn(Flag::C),
                cycles: JumpCycles {
                    executed: 3,
                    not_executed: 2,
                },
                length: InstructionLength::Two,
            },
            // ADD HL, SP
            0x39 => Instruction::Add {
                to: MemoryLocation::Register(HL),
                what: self.registers.get(SP),
                cycles: 2,
                length: InstructionLength::One,
            },
            // LD A, (HL-)
            0x3A => Instruction::Load {
                to: MemoryLocation::Register(A),
                what: self.read(self.registers.get(HL), true),
                additional_instruction: AdditionalInstruction::Dec,
                cycles: 2,
                length: InstructionLength::One,
            },
            // DEC SP
            0x3B => Instruction::Dec {
                what: MemoryLocation::Register(SP),
                cycles: 2,
            },
            // INC A
            0x3C => Instruction::Inc {
                what: MemoryLocation::Register(A),
                cycles: 1,
            },
            // DEC A
            0x3D => Instruction::Dec {
                what: MemoryLocation::Register(A),
                cycles: 1,
            },
            // LD A, d8
            0x3E => Instruction::Load {
                to: MemoryLocation::Register(A),
                what: self.immediate_operand(false),
                additional_instruction: AdditionalInstruction::None,
                cycles: 2,
                length: InstructionLength::Two,
            },
            // CCF
            0x3F => Instruction::Ccf,
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
