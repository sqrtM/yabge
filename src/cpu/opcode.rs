use crate::cpu::arithmetic::unsigned_to_signed_8;
use crate::cpu::flag::Flag;
use crate::cpu::flag::Flag::Z;
use crate::cpu::instruction::BitAddr::{Five, Four, One, Seven, Six, Three, Two, Zero};
use crate::cpu::instruction::Condition::{FlagOff, FlagOn};
use crate::cpu::instruction::{
    AdditionalInstruction, BitAddr, Condition, Instruction, InstructionLength, JumpCycles,
    RotateDirection,
};
use crate::cpu::registers::Register::{A, AF, B, BC, C, D, DE, E, H, HL, L, PC, SP};
use crate::cpu::value::{concat_values, Value};
use crate::cpu::{MemoryLocation, CPU};

impl CPU {
    pub fn lookup(&mut self, code: u8) -> Instruction {
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
                condition: Condition::None,
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
                condition: Condition::FlagOff(Z),
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
                condition: Condition::FlagOn(Z),
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
                condition: Condition::FlagOff(Flag::C),
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
                condition: Condition::FlagOn(Flag::C),
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
            // LD B, B
            0x40 => Instruction::Load {
                to: MemoryLocation::Register(B),
                what: self.registers.get(B),
                additional_instruction: AdditionalInstruction::None,
                cycles: 1,
                length: InstructionLength::One,
            },
            // LD B, C
            0x41 => Instruction::Load {
                to: MemoryLocation::Register(B),
                what: self.registers.get(C),
                additional_instruction: AdditionalInstruction::None,
                cycles: 1,
                length: InstructionLength::One,
            },
            // LD B, D
            0x42 => Instruction::Load {
                to: MemoryLocation::Register(B),
                what: self.registers.get(D),
                additional_instruction: AdditionalInstruction::None,
                cycles: 1,
                length: InstructionLength::One,
            },
            // LD B, E
            0x43 => Instruction::Load {
                to: MemoryLocation::Register(B),
                what: self.registers.get(E),
                additional_instruction: AdditionalInstruction::None,
                cycles: 1,
                length: InstructionLength::One,
            },
            // LD B, H
            0x44 => Instruction::Load {
                to: MemoryLocation::Register(B),
                what: self.registers.get(H),
                additional_instruction: AdditionalInstruction::None,
                cycles: 1,
                length: InstructionLength::One,
            },
            // LD B, L
            0x45 => Instruction::Load {
                to: MemoryLocation::Register(B),
                what: self.registers.get(L),
                additional_instruction: AdditionalInstruction::None,
                cycles: 1,
                length: InstructionLength::One,
            },
            // LD B, (HL)
            0x46 => Instruction::Load {
                to: MemoryLocation::Register(B),
                what: self.read(self.registers.get(HL), false),
                additional_instruction: AdditionalInstruction::None,
                cycles: 2,
                length: InstructionLength::One,
            },
            // LD B, A
            0x47 => Instruction::Load {
                to: MemoryLocation::Register(B),
                what: self.registers.get(A),
                additional_instruction: AdditionalInstruction::None,
                cycles: 1,
                length: InstructionLength::One,
            },
            // LD C, B
            0x48 => Instruction::Load {
                to: MemoryLocation::Register(C),
                what: self.registers.get(B),
                additional_instruction: AdditionalInstruction::None,
                cycles: 1,
                length: InstructionLength::One,
            },
            // LD C, C
            0x49 => Instruction::Load {
                to: MemoryLocation::Register(C),
                what: self.registers.get(C),
                additional_instruction: AdditionalInstruction::None,
                cycles: 1,
                length: InstructionLength::One,
            },
            // LD C, D
            0x4A => Instruction::Load {
                to: MemoryLocation::Register(C),
                what: self.registers.get(D),
                additional_instruction: AdditionalInstruction::None,
                cycles: 1,
                length: InstructionLength::One,
            },
            // LD C, E
            0x4B => Instruction::Load {
                to: MemoryLocation::Register(C),
                what: self.registers.get(E),
                additional_instruction: AdditionalInstruction::None,
                cycles: 1,
                length: InstructionLength::One,
            },
            // LD C, H
            0x4C => Instruction::Load {
                to: MemoryLocation::Register(C),
                what: self.registers.get(H),
                additional_instruction: AdditionalInstruction::None,
                cycles: 1,
                length: InstructionLength::One,
            },
            // LD C, L
            0x4D => Instruction::Load {
                to: MemoryLocation::Register(C),
                what: self.registers.get(L),
                additional_instruction: AdditionalInstruction::None,
                cycles: 1,
                length: InstructionLength::One,
            },
            // LD B, (HL)
            0x4E => Instruction::Load {
                to: MemoryLocation::Register(C),
                what: self.read(self.registers.get(HL), false),
                additional_instruction: AdditionalInstruction::None,
                cycles: 2,
                length: InstructionLength::One,
            },
            // LD C, A
            0x4F => Instruction::Load {
                to: MemoryLocation::Register(C),
                what: self.registers.get(A),
                additional_instruction: AdditionalInstruction::None,
                cycles: 1,
                length: InstructionLength::One,
            },
            // LD D, B
            0x50 => Instruction::Load {
                to: MemoryLocation::Register(D),
                what: self.registers.get(B),
                additional_instruction: AdditionalInstruction::None,
                cycles: 1,
                length: InstructionLength::One,
            },
            // LD D, C
            0x51 => Instruction::Load {
                to: MemoryLocation::Register(D),
                what: self.registers.get(C),
                additional_instruction: AdditionalInstruction::None,
                cycles: 1,
                length: InstructionLength::One,
            },
            // LD D, D
            0x52 => Instruction::Load {
                to: MemoryLocation::Register(D),
                what: self.registers.get(D),
                additional_instruction: AdditionalInstruction::None,
                cycles: 1,
                length: InstructionLength::One,
            },
            // LD D, E
            0x53 => Instruction::Load {
                to: MemoryLocation::Register(D),
                what: self.registers.get(E),
                additional_instruction: AdditionalInstruction::None,
                cycles: 1,
                length: InstructionLength::One,
            },
            // LD D, H
            0x54 => Instruction::Load {
                to: MemoryLocation::Register(D),
                what: self.registers.get(H),
                additional_instruction: AdditionalInstruction::None,
                cycles: 1,
                length: InstructionLength::One,
            },
            // LD D, L
            0x55 => Instruction::Load {
                to: MemoryLocation::Register(D),
                what: self.registers.get(L),
                additional_instruction: AdditionalInstruction::None,
                cycles: 1,
                length: InstructionLength::One,
            },
            // LD D, (HL)
            0x56 => Instruction::Load {
                to: MemoryLocation::Register(D),
                what: self.read(self.registers.get(HL), false),
                additional_instruction: AdditionalInstruction::None,
                cycles: 2,
                length: InstructionLength::One,
            },
            // LD D, A
            0x57 => Instruction::Load {
                to: MemoryLocation::Register(D),
                what: self.registers.get(A),
                additional_instruction: AdditionalInstruction::None,
                cycles: 1,
                length: InstructionLength::One,
            },
            // LD E, B
            0x58 => Instruction::Load {
                to: MemoryLocation::Register(E),
                what: self.registers.get(B),
                additional_instruction: AdditionalInstruction::None,
                cycles: 1,
                length: InstructionLength::One,
            },
            // LD E, C
            0x59 => Instruction::Load {
                to: MemoryLocation::Register(E),
                what: self.registers.get(C),
                additional_instruction: AdditionalInstruction::None,
                cycles: 1,
                length: InstructionLength::One,
            },
            // LD E, D
            0x5A => Instruction::Load {
                to: MemoryLocation::Register(E),
                what: self.registers.get(D),
                additional_instruction: AdditionalInstruction::None,
                cycles: 1,
                length: InstructionLength::One,
            },
            // LD E, E
            0x5B => Instruction::Load {
                to: MemoryLocation::Register(E),
                what: self.registers.get(E),
                additional_instruction: AdditionalInstruction::None,
                cycles: 1,
                length: InstructionLength::One,
            },
            // LD E, H
            0x5C => Instruction::Load {
                to: MemoryLocation::Register(E),
                what: self.registers.get(H),
                additional_instruction: AdditionalInstruction::None,
                cycles: 1,
                length: InstructionLength::One,
            },
            // LD E, L
            0x5D => Instruction::Load {
                to: MemoryLocation::Register(E),
                what: self.registers.get(L),
                additional_instruction: AdditionalInstruction::None,
                cycles: 1,
                length: InstructionLength::One,
            },
            // LD E, (HL)
            0x5E => Instruction::Load {
                to: MemoryLocation::Register(E),
                what: self.read(self.registers.get(HL), false),
                additional_instruction: AdditionalInstruction::None,
                cycles: 2,
                length: InstructionLength::One,
            },
            // LD E, A
            0x5F => Instruction::Load {
                to: MemoryLocation::Register(E),
                what: self.registers.get(A),
                additional_instruction: AdditionalInstruction::None,
                cycles: 1,
                length: InstructionLength::One,
            },
            // LD H, B
            0x60 => Instruction::Load {
                to: MemoryLocation::Register(H),
                what: self.registers.get(B),
                additional_instruction: AdditionalInstruction::None,
                cycles: 1,
                length: InstructionLength::One,
            },
            // LD H, C
            0x61 => Instruction::Load {
                to: MemoryLocation::Register(H),
                what: self.registers.get(C),
                additional_instruction: AdditionalInstruction::None,
                cycles: 1,
                length: InstructionLength::One,
            },
            // LD H, D
            0x62 => Instruction::Load {
                to: MemoryLocation::Register(H),
                what: self.registers.get(D),
                additional_instruction: AdditionalInstruction::None,
                cycles: 1,
                length: InstructionLength::One,
            },
            // LD H, E
            0x63 => Instruction::Load {
                to: MemoryLocation::Register(H),
                what: self.registers.get(E),
                additional_instruction: AdditionalInstruction::None,
                cycles: 1,
                length: InstructionLength::One,
            },
            // LD H, H
            0x64 => Instruction::Load {
                to: MemoryLocation::Register(H),
                what: self.registers.get(H),
                additional_instruction: AdditionalInstruction::None,
                cycles: 1,
                length: InstructionLength::One,
            },
            // LD H, L
            0x65 => Instruction::Load {
                to: MemoryLocation::Register(H),
                what: self.registers.get(L),
                additional_instruction: AdditionalInstruction::None,
                cycles: 1,
                length: InstructionLength::One,
            },
            // LD H, (HL)
            0x66 => Instruction::Load {
                to: MemoryLocation::Register(H),
                what: self.read(self.registers.get(HL), false),
                additional_instruction: AdditionalInstruction::None,
                cycles: 2,
                length: InstructionLength::One,
            },
            // LD H, A
            0x67 => Instruction::Load {
                to: MemoryLocation::Register(H),
                what: self.registers.get(A),
                additional_instruction: AdditionalInstruction::None,
                cycles: 1,
                length: InstructionLength::One,
            },
            // LD L, B
            0x68 => Instruction::Load {
                to: MemoryLocation::Register(L),
                what: self.registers.get(B),
                additional_instruction: AdditionalInstruction::None,
                cycles: 1,
                length: InstructionLength::One,
            },
            // LD L, C
            0x69 => Instruction::Load {
                to: MemoryLocation::Register(L),
                what: self.registers.get(C),
                additional_instruction: AdditionalInstruction::None,
                cycles: 1,
                length: InstructionLength::One,
            },
            // LD L, D
            0x6A => Instruction::Load {
                to: MemoryLocation::Register(L),
                what: self.registers.get(D),
                additional_instruction: AdditionalInstruction::None,
                cycles: 1,
                length: InstructionLength::One,
            },
            // LD L, E
            0x6B => Instruction::Load {
                to: MemoryLocation::Register(L),
                what: self.registers.get(E),
                additional_instruction: AdditionalInstruction::None,
                cycles: 1,
                length: InstructionLength::One,
            },
            // LD L, H
            0x6C => Instruction::Load {
                to: MemoryLocation::Register(L),
                what: self.registers.get(H),
                additional_instruction: AdditionalInstruction::None,
                cycles: 1,
                length: InstructionLength::One,
            },
            // LD L, L
            0x6D => Instruction::Load {
                to: MemoryLocation::Register(L),
                what: self.registers.get(L),
                additional_instruction: AdditionalInstruction::None,
                cycles: 1,
                length: InstructionLength::One,
            },
            // LD L, (HL)
            0x6E => Instruction::Load {
                to: MemoryLocation::Register(L),
                what: self.read(self.registers.get(HL), false),
                additional_instruction: AdditionalInstruction::None,
                cycles: 2,
                length: InstructionLength::One,
            },
            // LD L, A
            0x6F => Instruction::Load {
                to: MemoryLocation::Register(L),
                what: self.registers.get(A),
                additional_instruction: AdditionalInstruction::None,
                cycles: 1,
                length: InstructionLength::One,
            },
            // LD (HL), B
            0x70 => Instruction::Load {
                to: MemoryLocation::Pointer(self.registers.get(HL)),
                what: self.registers.get(B),
                additional_instruction: AdditionalInstruction::None,
                cycles: 2,
                length: InstructionLength::One,
            },
            // LD (HL), C
            0x71 => Instruction::Load {
                to: MemoryLocation::Pointer(self.registers.get(HL)),
                what: self.registers.get(C),
                additional_instruction: AdditionalInstruction::None,
                cycles: 2,
                length: InstructionLength::One,
            },
            // LD (HL), D
            0x72 => Instruction::Load {
                to: MemoryLocation::Pointer(self.registers.get(HL)),
                what: self.registers.get(D),
                additional_instruction: AdditionalInstruction::None,
                cycles: 2,
                length: InstructionLength::One,
            },
            // LD (HL), E
            0x73 => Instruction::Load {
                to: MemoryLocation::Pointer(self.registers.get(HL)),
                what: self.registers.get(E),
                additional_instruction: AdditionalInstruction::None,
                cycles: 2,
                length: InstructionLength::One,
            },
            // LD (HL), H
            0x74 => Instruction::Load {
                to: MemoryLocation::Pointer(self.registers.get(HL)),
                what: self.registers.get(H),
                additional_instruction: AdditionalInstruction::None,
                cycles: 2,
                length: InstructionLength::One,
            },
            // LD (HL), L
            0x75 => Instruction::Load {
                to: MemoryLocation::Pointer(self.registers.get(HL)),
                what: self.registers.get(L),
                additional_instruction: AdditionalInstruction::None,
                cycles: 2,
                length: InstructionLength::One,
            },
            // Halt
            0x76 => todo!("Halt not yet implemented!!"),
            // LD (HL), A
            0x77 => Instruction::Load {
                to: MemoryLocation::Pointer(self.registers.get(HL)),
                what: self.registers.get(A),
                additional_instruction: AdditionalInstruction::None,
                cycles: 2,
                length: InstructionLength::One,
            },
            // LD A, B
            0x78 => Instruction::Load {
                to: MemoryLocation::Register(A),
                what: self.registers.get(B),
                additional_instruction: AdditionalInstruction::None,
                cycles: 1,
                length: InstructionLength::One,
            },
            // LD A, C
            0x79 => Instruction::Load {
                to: MemoryLocation::Register(A),
                what: self.registers.get(C),
                additional_instruction: AdditionalInstruction::None,
                cycles: 1,
                length: InstructionLength::One,
            },
            // LD A, D
            0x7A => Instruction::Load {
                to: MemoryLocation::Register(A),
                what: self.registers.get(D),
                additional_instruction: AdditionalInstruction::None,
                cycles: 1,
                length: InstructionLength::One,
            },
            // LD A, E
            0x7B => Instruction::Load {
                to: MemoryLocation::Register(A),
                what: self.registers.get(E),
                additional_instruction: AdditionalInstruction::None,
                cycles: 1,
                length: InstructionLength::One,
            },
            // LD A, H
            0x7C => Instruction::Load {
                to: MemoryLocation::Register(A),
                what: self.registers.get(H),
                additional_instruction: AdditionalInstruction::None,
                cycles: 1,
                length: InstructionLength::One,
            },
            // LD A, L
            0x7D => Instruction::Load {
                to: MemoryLocation::Register(A),
                what: self.registers.get(L),
                additional_instruction: AdditionalInstruction::None,
                cycles: 1,
                length: InstructionLength::One,
            },
            // LD A, (HL)
            0x7E => Instruction::Load {
                to: MemoryLocation::Register(A),
                what: self.read(self.registers.get(HL), false),
                additional_instruction: AdditionalInstruction::None,
                cycles: 2,
                length: InstructionLength::One,
            },
            // LD A, A
            0x7F => Instruction::Load {
                to: MemoryLocation::Register(A),
                what: self.registers.get(A),
                additional_instruction: AdditionalInstruction::None,
                cycles: 1,
                length: InstructionLength::One,
            },
            // ADD A, B
            0x80 => Instruction::Add {
                to: MemoryLocation::Register(A),
                what: self.registers.get(B),
                cycles: 1,
                length: InstructionLength::One,
            },
            // ADD A, C
            0x81 => Instruction::Add {
                to: MemoryLocation::Register(A),
                what: self.registers.get(C),
                cycles: 1,
                length: InstructionLength::One,
            },
            // ADD A, D
            0x82 => Instruction::Add {
                to: MemoryLocation::Register(A),
                what: self.registers.get(D),
                cycles: 1,
                length: InstructionLength::One,
            },
            // ADD A, E
            0x83 => Instruction::Add {
                to: MemoryLocation::Register(A),
                what: self.registers.get(E),
                cycles: 1,
                length: InstructionLength::One,
            },
            // ADD A, H
            0x84 => Instruction::Add {
                to: MemoryLocation::Register(A),
                what: self.registers.get(H),
                cycles: 1,
                length: InstructionLength::One,
            },
            // ADD A, L
            0x85 => Instruction::Add {
                to: MemoryLocation::Register(A),
                what: self.registers.get(L),
                cycles: 1,
                length: InstructionLength::One,
            },
            // ADD A, (HL)
            0x86 => Instruction::Add {
                to: MemoryLocation::Register(A),
                what: self.read(self.registers.get(HL), false),
                cycles: 2,
                length: InstructionLength::One,
            },
            // ADD A, A
            0x87 => Instruction::Add {
                to: MemoryLocation::Register(A),
                what: self.registers.get(L),
                cycles: 1,
                length: InstructionLength::One,
            },
            // ADC A, B
            0x88 => Instruction::Adc {
                to: MemoryLocation::Register(A),
                what: self.registers.get(B),
                cycles: 1,
                length: InstructionLength::One,
            },
            // ADC A, C
            0x89 => Instruction::Adc {
                to: MemoryLocation::Register(A),
                what: self.registers.get(C),
                cycles: 1,
                length: InstructionLength::One,
            },
            // ADC A, D
            0x8A => Instruction::Adc {
                to: MemoryLocation::Register(A),
                what: self.registers.get(D),
                cycles: 1,
                length: InstructionLength::One,
            },
            // ADC A, E
            0x8B => Instruction::Adc {
                to: MemoryLocation::Register(A),
                what: self.registers.get(E),
                cycles: 1,
                length: InstructionLength::One,
            },
            // ADC A, H
            0x8C => Instruction::Adc {
                to: MemoryLocation::Register(A),
                what: self.registers.get(H),
                cycles: 1,
                length: InstructionLength::One,
            },
            // ADC A, L
            0x8D => Instruction::Adc {
                to: MemoryLocation::Register(A),
                what: self.registers.get(L),
                cycles: 1,
                length: InstructionLength::One,
            },
            // ADC A, (HL)
            0x8E => Instruction::Add {
                to: MemoryLocation::Register(A),
                what: self.read(self.registers.get(HL), false),
                cycles: 2,
                length: InstructionLength::One,
            },
            // ADC A, A
            0x8F => Instruction::Adc {
                to: MemoryLocation::Register(A),
                what: self.registers.get(A),
                cycles: 1,
                length: InstructionLength::One,
            },
            // SUB A, B
            0x90 => Instruction::Sub {
                from: MemoryLocation::Register(A),
                what: self.registers.get(B),
                cycles: 1,
                length: InstructionLength::One,
            },
            // SUB A, C
            0x91 => Instruction::Sub {
                from: MemoryLocation::Register(A),
                what: self.registers.get(C),
                cycles: 1,
                length: InstructionLength::One,
            },
            // SUB A, D
            0x92 => Instruction::Sub {
                from: MemoryLocation::Register(A),
                what: self.registers.get(D),
                cycles: 1,
                length: InstructionLength::One,
            },
            // SUB A, E
            0x93 => Instruction::Sub {
                from: MemoryLocation::Register(A),
                what: self.registers.get(E),
                cycles: 1,
                length: InstructionLength::One,
            },
            // SUB A, H
            0x94 => Instruction::Sub {
                from: MemoryLocation::Register(A),
                what: self.registers.get(H),
                cycles: 1,
                length: InstructionLength::One,
            },
            // SUB A, L
            0x95 => Instruction::Sub {
                from: MemoryLocation::Register(A),
                what: self.registers.get(L),
                cycles: 1,
                length: InstructionLength::One,
            },
            // SUB A, (HL)
            0x96 => Instruction::Sub {
                from: MemoryLocation::Register(A),
                what: self.read(self.registers.get(HL), false),
                cycles: 2,
                length: InstructionLength::One,
            },
            // SUB A, A
            0x97 => Instruction::Sub {
                from: MemoryLocation::Register(A),
                what: self.registers.get(L),
                cycles: 1,
                length: InstructionLength::One,
            },
            // SBC A, B
            0x98 => Instruction::Sbc {
                from: MemoryLocation::Register(A),
                what: self.registers.get(B),
                cycles: 1,
                length: InstructionLength::One,
            },
            // SBC A, C
            0x99 => Instruction::Sbc {
                from: MemoryLocation::Register(A),
                what: self.registers.get(C),
                cycles: 1,
                length: InstructionLength::One,
            },
            // SBC A, D
            0x9A => Instruction::Sbc {
                from: MemoryLocation::Register(A),
                what: self.registers.get(D),
                cycles: 1,
                length: InstructionLength::One,
            },
            // SBC A, E
            0x9B => Instruction::Sbc {
                from: MemoryLocation::Register(A),
                what: self.registers.get(E),
                cycles: 1,
                length: InstructionLength::One,
            },
            // SBC A, H
            0x9C => Instruction::Sbc {
                from: MemoryLocation::Register(A),
                what: self.registers.get(H),
                cycles: 1,
                length: InstructionLength::One,
            },
            // SBC A, L
            0x9D => Instruction::Sbc {
                from: MemoryLocation::Register(A),
                what: self.registers.get(L),
                cycles: 1,
                length: InstructionLength::One,
            },
            // SBC A, (HL)
            0x9E => Instruction::Sbc {
                from: MemoryLocation::Register(A),
                what: self.read(self.registers.get(HL), false),
                cycles: 2,
                length: InstructionLength::One,
            },
            // SBC A, A
            0x9F => Instruction::Sbc {
                from: MemoryLocation::Register(A),
                what: self.registers.get(A),
                cycles: 1,
                length: InstructionLength::One,
            },
            // AND B
            0xA0 => Instruction::And {
                what: self.registers.get(B),
                cycles: 1,
                length: InstructionLength::One,
            },
            // AND C
            0xA1 => Instruction::And {
                what: self.registers.get(C),
                cycles: 1,
                length: InstructionLength::One,
            },
            // AND D
            0xA2 => Instruction::And {
                what: self.registers.get(D),
                cycles: 1,
                length: InstructionLength::One,
            },
            // AND E
            0xA3 => Instruction::And {
                what: self.registers.get(E),
                cycles: 1,
                length: InstructionLength::One,
            },
            // AND H
            0xA4 => Instruction::And {
                what: self.registers.get(H),
                cycles: 1,
                length: InstructionLength::One,
            },
            // AND L
            0xA5 => Instruction::And {
                what: self.registers.get(L),
                cycles: 1,
                length: InstructionLength::One,
            },
            // AND (HL)
            0xA6 => Instruction::And {
                what: self.read(self.registers.get(HL), false),
                cycles: 2,
                length: InstructionLength::One,
            },
            // AND A
            0xA7 => Instruction::And {
                what: self.registers.get(A),
                cycles: 1,
                length: InstructionLength::One,
            },
            // XOR B
            0xA8 => Instruction::Xor {
                what: self.registers.get(B),
                cycles: 1,
                length: InstructionLength::One,
            },
            // XOR C
            0xA9 => Instruction::Xor {
                what: self.registers.get(C),
                cycles: 1,
                length: InstructionLength::One,
            },
            // XOR D
            0xAA => Instruction::Xor {
                what: self.registers.get(D),
                cycles: 1,
                length: InstructionLength::One,
            },
            // XOR E
            0xAB => Instruction::Xor {
                what: self.registers.get(E),
                cycles: 1,
                length: InstructionLength::One,
            },
            // XOR H
            0xAC => Instruction::Xor {
                what: self.registers.get(H),
                cycles: 1,
                length: InstructionLength::One,
            },
            // XOR L
            0xAD => Instruction::Xor {
                what: self.registers.get(L),
                cycles: 1,
                length: InstructionLength::One,
            },
            // XOR (HL)
            0xAE => Instruction::Xor {
                what: self.read(self.registers.get(HL), false),
                cycles: 2,
                length: InstructionLength::One,
            },
            // XOR A
            0xAF => Instruction::Xor {
                what: self.registers.get(A),
                cycles: 1,
                length: InstructionLength::One,
            },
            // OR B
            0xB0 => Instruction::Or {
                what: self.registers.get(B),
                cycles: 1,
                length: InstructionLength::One,
            },
            // OR C
            0xB1 => Instruction::Or {
                what: self.registers.get(C),
                cycles: 1,
                length: InstructionLength::One,
            },
            // OR D
            0xB2 => Instruction::Or {
                what: self.registers.get(D),
                cycles: 1,
                length: InstructionLength::One,
            },
            // OR E
            0xB3 => Instruction::Or {
                what: self.registers.get(E),
                cycles: 1,
                length: InstructionLength::One,
            },
            // OR H
            0xB4 => Instruction::Or {
                what: self.registers.get(H),
                cycles: 1,
                length: InstructionLength::One,
            },
            // OR L
            0xB5 => Instruction::Or {
                what: self.registers.get(L),
                cycles: 1,
                length: InstructionLength::One,
            },
            // OR (HL)
            0xB6 => Instruction::Or {
                what: self.read(self.registers.get(HL), false),
                cycles: 2,
                length: InstructionLength::One,
            },
            // OR A
            0xB7 => Instruction::Or {
                what: self.registers.get(A),
                cycles: 1,
                length: InstructionLength::One,
            },
            // CP B
            0xB8 => Instruction::Cp {
                what: self.registers.get(B),
                cycles: 1,
                length: InstructionLength::One,
            },
            // CP C
            0xB9 => Instruction::Cp {
                what: self.registers.get(C),
                cycles: 1,
                length: InstructionLength::One,
            },
            // CP D
            0xBA => Instruction::Cp {
                what: self.registers.get(D),
                cycles: 1,
                length: InstructionLength::One,
            },
            // CP E
            0xBB => Instruction::Cp {
                what: self.registers.get(E),
                cycles: 1,
                length: InstructionLength::One,
            },
            // CP H
            0xBC => Instruction::Cp {
                what: self.registers.get(H),
                cycles: 1,
                length: InstructionLength::One,
            },
            // CP L
            0xBD => Instruction::Cp {
                what: self.registers.get(L),
                cycles: 1,
                length: InstructionLength::One,
            },
            // CP (HL)
            0xBE => Instruction::Cp {
                what: self.read(self.registers.get(HL), false),
                cycles: 2,
                length: InstructionLength::One,
            },
            // CP A
            0xBF => Instruction::Cp {
                what: self.registers.get(A),
                cycles: 1,
                length: InstructionLength::One,
            },
            // RET NZ
            0xC0 => Instruction::Ret(FlagOff(Z)),
            // POP BC
            0xC1 => Instruction::Pop(BC),
            // JP NZ, a16
            0xC2 => Instruction::Jp {
                to: self.immediate_operand(true),
                condition: FlagOff(Z),
                cycles: JumpCycles {
                    executed: 4,
                    not_executed: 3,
                },
                length: InstructionLength::Three,
            },
            // JP a16
            0xC3 => Instruction::Jp {
                to: self.immediate_operand(true),
                condition: Condition::None,
                cycles: JumpCycles {
                    executed: 4,
                    not_executed: 4,
                },
                length: InstructionLength::Three,
            },
            // CALL NZ, a16
            0xC4 => Instruction::Call(FlagOff(Z)),
            // PUSH BC
            0xC5 => Instruction::Push(BC),
            // ADD A, d8
            0xC6 => Instruction::Add {
                to: MemoryLocation::Register(A),
                what: self.immediate_operand(false),
                cycles: 2,
                length: InstructionLength::Two,
            },
            // RST 0
            0xC7 => Instruction::Rst(Zero),
            // RET Z
            0xC8 => Instruction::Ret(FlagOn(Z)),
            // RET
            0xC9 => Instruction::Ret(Condition::None),
            // JP Z, a16
            0xCA => Instruction::Jp {
                to: self.immediate_operand(true),
                condition: FlagOn(Z),
                cycles: JumpCycles {
                    executed: 4,
                    not_executed: 3,
                },
                length: InstructionLength::Three,
            },
            // Prefixed Ops
            0xCB => {
                if let Value::EightBit(postfix) = self.read(self.registers.get(PC) + 1u16, false) {
                    match postfix {
                        // RLC B
                        0x00 => Instruction::Rot {
                            what: MemoryLocation::Register(B),
                            direction: RotateDirection::Left,
                            use_carry: false,
                            cycles: 2,
                            length: InstructionLength::Two,
                        },
                        // RLC C
                        0x01 => Instruction::Rot {
                            what: MemoryLocation::Register(C),
                            direction: RotateDirection::Left,
                            use_carry: false,
                            cycles: 2,
                            length: InstructionLength::Two,
                        },
                        // RLC D
                        0x02 => Instruction::Rot {
                            what: MemoryLocation::Register(D),
                            direction: RotateDirection::Left,
                            use_carry: false,
                            cycles: 2,
                            length: InstructionLength::Two,
                        },
                        // RLC E
                        0x03 => Instruction::Rot {
                            what: MemoryLocation::Register(E),
                            direction: RotateDirection::Left,
                            use_carry: false,
                            cycles: 2,
                            length: InstructionLength::Two,
                        },
                        // RLC H
                        0x04 => Instruction::Rot {
                            what: MemoryLocation::Register(H),
                            direction: RotateDirection::Left,
                            use_carry: false,
                            cycles: 2,
                            length: InstructionLength::Two,
                        },
                        // RLC L
                        0x05 => Instruction::Rot {
                            what: MemoryLocation::Register(L),
                            direction: RotateDirection::Left,
                            use_carry: false,
                            cycles: 2,
                            length: InstructionLength::Two,
                        },
                        // RLC (HL)
                        0x06 => Instruction::Rot {
                            what: MemoryLocation::Pointer(self.registers.get(HL)),
                            direction: RotateDirection::Left,
                            use_carry: false,
                            cycles: 2,
                            length: InstructionLength::Two,
                        },
                        // RLC A
                        0x07 => Instruction::Rot {
                            what: MemoryLocation::Register(A),
                            direction: RotateDirection::Left,
                            use_carry: false,
                            cycles: 2,
                            length: InstructionLength::Two,
                        },
                        // RRC B
                        0x08 => Instruction::Rot {
                            what: MemoryLocation::Register(B),
                            direction: RotateDirection::Right,
                            use_carry: false,
                            cycles: 2,
                            length: InstructionLength::Two,
                        },
                        // RRC C
                        0x09 => Instruction::Rot {
                            what: MemoryLocation::Register(C),
                            direction: RotateDirection::Right,
                            use_carry: false,
                            cycles: 2,
                            length: InstructionLength::Two,
                        },
                        // RRC D
                        0x0A => Instruction::Rot {
                            what: MemoryLocation::Register(D),
                            direction: RotateDirection::Right,
                            use_carry: false,
                            cycles: 2,
                            length: InstructionLength::Two,
                        },
                        // RRC E
                        0x0B => Instruction::Rot {
                            what: MemoryLocation::Register(E),
                            direction: RotateDirection::Right,
                            use_carry: false,
                            cycles: 2,
                            length: InstructionLength::Two,
                        },
                        // RRC H
                        0x0C => Instruction::Rot {
                            what: MemoryLocation::Register(H),
                            direction: RotateDirection::Right,
                            use_carry: false,
                            cycles: 2,
                            length: InstructionLength::Two,
                        },
                        // RRC L
                        0x0D => Instruction::Rot {
                            what: MemoryLocation::Register(L),
                            direction: RotateDirection::Right,
                            use_carry: false,
                            cycles: 2,
                            length: InstructionLength::Two,
                        },
                        // RRC (HL)
                        0x0E => Instruction::Rot {
                            what: MemoryLocation::Pointer(self.registers.get(HL)),
                            direction: RotateDirection::Right,
                            use_carry: false,
                            cycles: 2,
                            length: InstructionLength::Two,
                        },
                        // RRC A
                        0x0F => Instruction::Rot {
                            what: MemoryLocation::Register(A),
                            direction: RotateDirection::Right,
                            use_carry: false,
                            cycles: 2,
                            length: InstructionLength::Two,
                        },
                        // RL B
                        0x10 => Instruction::Rot {
                            what: MemoryLocation::Register(B),
                            direction: RotateDirection::Left,
                            use_carry: true,
                            cycles: 2,
                            length: InstructionLength::Two,
                        },
                        // RL C
                        0x11 => Instruction::Rot {
                            what: MemoryLocation::Register(C),
                            direction: RotateDirection::Left,
                            use_carry: true,
                            cycles: 2,
                            length: InstructionLength::Two,
                        },
                        // RL D
                        0x12 => Instruction::Rot {
                            what: MemoryLocation::Register(D),
                            direction: RotateDirection::Left,
                            use_carry: true,
                            cycles: 2,
                            length: InstructionLength::Two,
                        },
                        // RL E
                        0x13 => Instruction::Rot {
                            what: MemoryLocation::Register(E),
                            direction: RotateDirection::Left,
                            use_carry: true,
                            cycles: 2,
                            length: InstructionLength::Two,
                        },
                        // RL H
                        0x14 => Instruction::Rot {
                            what: MemoryLocation::Register(H),
                            direction: RotateDirection::Left,
                            use_carry: true,
                            cycles: 2,
                            length: InstructionLength::Two,
                        },
                        // RL L
                        0x15 => Instruction::Rot {
                            what: MemoryLocation::Register(L),
                            direction: RotateDirection::Left,
                            use_carry: true,
                            cycles: 2,
                            length: InstructionLength::Two,
                        },
                        // RL (HL)
                        0x16 => Instruction::Rot {
                            what: MemoryLocation::Pointer(self.registers.get(HL)),
                            direction: RotateDirection::Left,
                            use_carry: true,
                            cycles: 2,
                            length: InstructionLength::Two,
                        },
                        // RL A
                        0x17 => Instruction::Rot {
                            what: MemoryLocation::Register(A),
                            direction: RotateDirection::Left,
                            use_carry: true,
                            cycles: 2,
                            length: InstructionLength::Two,
                        },
                        // RR B
                        0x18 => Instruction::Rot {
                            what: MemoryLocation::Register(B),
                            direction: RotateDirection::Right,
                            use_carry: true,
                            cycles: 2,
                            length: InstructionLength::Two,
                        },
                        // RR C
                        0x19 => Instruction::Rot {
                            what: MemoryLocation::Register(C),
                            direction: RotateDirection::Right,
                            use_carry: true,
                            cycles: 2,
                            length: InstructionLength::Two,
                        },
                        // RR D
                        0x1A => Instruction::Rot {
                            what: MemoryLocation::Register(D),
                            direction: RotateDirection::Right,
                            use_carry: true,
                            cycles: 2,
                            length: InstructionLength::Two,
                        },
                        // RR E
                        0x1B => Instruction::Rot {
                            what: MemoryLocation::Register(E),
                            direction: RotateDirection::Right,
                            use_carry: true,
                            cycles: 2,
                            length: InstructionLength::Two,
                        },
                        // RR H
                        0x1C => Instruction::Rot {
                            what: MemoryLocation::Register(H),
                            direction: RotateDirection::Right,
                            use_carry: true,
                            cycles: 2,
                            length: InstructionLength::Two,
                        },
                        // RR L
                        0x1D => Instruction::Rot {
                            what: MemoryLocation::Register(L),
                            direction: RotateDirection::Right,
                            use_carry: true,
                            cycles: 2,
                            length: InstructionLength::Two,
                        },
                        // RR (HL)
                        0x1E => Instruction::Rot {
                            what: MemoryLocation::Pointer(self.registers.get(HL)),
                            direction: RotateDirection::Right,
                            use_carry: true,
                            cycles: 2,
                            length: InstructionLength::Two,
                        },
                        // RR A
                        0x1F => Instruction::Rot {
                            what: MemoryLocation::Register(A),
                            direction: RotateDirection::Right,
                            use_carry: true,
                            cycles: 2,
                            length: InstructionLength::Two,
                        },
                        // SLA B
                        0x20 => Instruction::Shift {
                            what: MemoryLocation::Register(B),
                            direction: RotateDirection::Left,
                            arithmetic: true,
                            cycles: 2,
                            length: InstructionLength::Two,
                        },
                        // SLA C
                        0x21 => Instruction::Shift {
                            what: MemoryLocation::Register(C),
                            direction: RotateDirection::Left,
                            arithmetic: true,
                            cycles: 2,
                            length: InstructionLength::Two,
                        },
                        // SLA D
                        0x22 => Instruction::Shift {
                            what: MemoryLocation::Register(D),
                            direction: RotateDirection::Left,
                            arithmetic: true,
                            cycles: 2,
                            length: InstructionLength::Two,
                        },
                        // SLA E
                        0x23 => Instruction::Shift {
                            what: MemoryLocation::Register(E),
                            direction: RotateDirection::Left,
                            arithmetic: true,
                            cycles: 2,
                            length: InstructionLength::Two,
                        },
                        // SLA H
                        0x24 => Instruction::Shift {
                            what: MemoryLocation::Register(H),
                            direction: RotateDirection::Left,
                            arithmetic: true,
                            cycles: 2,
                            length: InstructionLength::Two,
                        },
                        // SLA L
                        0x25 => Instruction::Shift {
                            what: MemoryLocation::Register(L),
                            direction: RotateDirection::Left,
                            arithmetic: true,
                            cycles: 2,
                            length: InstructionLength::Two,
                        },
                        // SLA (HL)
                        0x26 => Instruction::Shift {
                            what: MemoryLocation::Pointer(self.registers.get(HL)),
                            direction: RotateDirection::Left,
                            arithmetic: true,
                            cycles: 2,
                            length: InstructionLength::Two,
                        },
                        // SLA A
                        0x27 => Instruction::Shift {
                            what: MemoryLocation::Register(A),
                            direction: RotateDirection::Left,
                            arithmetic: true,
                            cycles: 2,
                            length: InstructionLength::Two,
                        },
                        // SRA B
                        0x28 => Instruction::Shift {
                            what: MemoryLocation::Register(B),
                            direction: RotateDirection::Right,
                            arithmetic: true,
                            cycles: 2,
                            length: InstructionLength::Two,
                        },
                        // SRA C
                        0x29 => Instruction::Shift {
                            what: MemoryLocation::Register(C),
                            direction: RotateDirection::Right,
                            arithmetic: true,
                            cycles: 2,
                            length: InstructionLength::Two,
                        },
                        // SRA D
                        0x2A => Instruction::Shift {
                            what: MemoryLocation::Register(D),
                            direction: RotateDirection::Right,
                            arithmetic: true,
                            cycles: 2,
                            length: InstructionLength::Two,
                        },
                        // SRA E
                        0x2B => Instruction::Shift {
                            what: MemoryLocation::Register(E),
                            direction: RotateDirection::Right,
                            arithmetic: true,
                            cycles: 2,
                            length: InstructionLength::Two,
                        },
                        // SRA H
                        0x2C => Instruction::Shift {
                            what: MemoryLocation::Register(H),
                            direction: RotateDirection::Right,
                            arithmetic: true,
                            cycles: 2,
                            length: InstructionLength::Two,
                        },
                        // SRA L
                        0x2D => Instruction::Shift {
                            what: MemoryLocation::Register(L),
                            direction: RotateDirection::Right,
                            arithmetic: true,
                            cycles: 2,
                            length: InstructionLength::Two,
                        },
                        // SRA (HL)
                        0x2E => Instruction::Shift {
                            what: MemoryLocation::Pointer(self.registers.get(HL)),
                            direction: RotateDirection::Right,
                            arithmetic: true,
                            cycles: 2,
                            length: InstructionLength::Two,
                        },
                        // SRA A
                        0x2F => Instruction::Shift {
                            what: MemoryLocation::Register(A),
                            direction: RotateDirection::Right,
                            arithmetic: true,
                            cycles: 2,
                            length: InstructionLength::Two,
                        },
                        // SWAP B
                        0x30 => Instruction::Swap(MemoryLocation::Register(B)),
                        // SWAP C
                        0x31 => Instruction::Swap(MemoryLocation::Register(C)),
                        // SWAP D
                        0x32 => Instruction::Swap(MemoryLocation::Register(D)),
                        // SWAP E
                        0x33 => Instruction::Swap(MemoryLocation::Register(E)),
                        // SWAP H
                        0x34 => Instruction::Swap(MemoryLocation::Register(H)),
                        // SWAP L
                        0x35 => Instruction::Swap(MemoryLocation::Register(L)),
                        // SWAP (HL)
                        0x36 => Instruction::Swap(MemoryLocation::Pointer(self.registers.get(HL))),
                        // SWAP A
                        0x37 => Instruction::Swap(MemoryLocation::Register(A)),
                        // SRL B
                        0x38 => Instruction::Shift {
                            what: MemoryLocation::Register(B),
                            direction: RotateDirection::Right,
                            arithmetic: false,
                            cycles: 2,
                            length: InstructionLength::Two,
                        },
                        // SRL C
                        0x39 => Instruction::Shift {
                            what: MemoryLocation::Register(C),
                            direction: RotateDirection::Right,
                            arithmetic: false,
                            cycles: 2,
                            length: InstructionLength::Two,
                        },
                        // SRL D
                        0x3A => Instruction::Shift {
                            what: MemoryLocation::Register(D),
                            direction: RotateDirection::Right,
                            arithmetic: false,
                            cycles: 2,
                            length: InstructionLength::Two,
                        },
                        // SRL E
                        0x3B => Instruction::Shift {
                            what: MemoryLocation::Register(E),
                            direction: RotateDirection::Right,
                            arithmetic: false,
                            cycles: 2,
                            length: InstructionLength::Two,
                        },
                        // SRL H
                        0x3C => Instruction::Shift {
                            what: MemoryLocation::Register(H),
                            direction: RotateDirection::Right,
                            arithmetic: false,
                            cycles: 2,
                            length: InstructionLength::Two,
                        },
                        // SRL L
                        0x3D => Instruction::Shift {
                            what: MemoryLocation::Register(L),
                            direction: RotateDirection::Right,
                            arithmetic: false,
                            cycles: 2,
                            length: InstructionLength::Two,
                        },
                        // SRL (HL)
                        0x3E => Instruction::Shift {
                            what: MemoryLocation::Pointer(self.registers.get(HL)),
                            direction: RotateDirection::Right,
                            arithmetic: false,
                            cycles: 2,
                            length: InstructionLength::Two,
                        },
                        // SRL A
                        0x3F => Instruction::Shift {
                            what: MemoryLocation::Register(A),
                            direction: RotateDirection::Right,
                            arithmetic: false,
                            cycles: 2,
                            length: InstructionLength::Two,
                        },
                        // BIT 0, B
                        0x40 => Instruction::Bit {
                            what: MemoryLocation::Register(B),
                            bit: Zero,
                        },
                        // BIT 0, C
                        0x41 => Instruction::Bit {
                            what: MemoryLocation::Register(C),
                            bit: Zero,
                        },
                        // BIT 0, D
                        0x42 => Instruction::Bit {
                            what: MemoryLocation::Register(D),
                            bit: Zero,
                        },
                        // BIT 0, E
                        0x43 => Instruction::Bit {
                            what: MemoryLocation::Register(E),
                            bit: Zero,
                        },
                        // BIT 0, H
                        0x44 => Instruction::Bit {
                            what: MemoryLocation::Register(H),
                            bit: Zero,
                        },
                        // BIT 0, L
                        0x45 => Instruction::Bit {
                            what: MemoryLocation::Register(L),
                            bit: Zero,
                        },
                        // BIT 0, (HL)
                        0x46 => Instruction::Bit {
                            what: MemoryLocation::Pointer(self.registers.get(HL)),
                            bit: Zero,
                        },
                        // BIT 0, A
                        0x47 => Instruction::Bit {
                            what: MemoryLocation::Register(A),
                            bit: Zero,
                        },
                        // BIT 1, B
                        0x48 => Instruction::Bit {
                            what: MemoryLocation::Register(B),
                            bit: One,
                        },
                        // BIT 1, C
                        0x49 => Instruction::Bit {
                            what: MemoryLocation::Register(C),
                            bit: One,
                        },
                        // BIT 1, D
                        0x4A => Instruction::Bit {
                            what: MemoryLocation::Register(D),
                            bit: One,
                        },
                        // BIT 1, E
                        0x4B => Instruction::Bit {
                            what: MemoryLocation::Register(E),
                            bit: One,
                        },
                        // BIT 1, H
                        0x4C => Instruction::Bit {
                            what: MemoryLocation::Register(H),
                            bit: One,
                        },
                        // BIT 1, L
                        0x4D => Instruction::Bit {
                            what: MemoryLocation::Register(L),
                            bit: One,
                        },
                        // BIT 1, (HL)
                        0x4E => Instruction::Bit {
                            what: MemoryLocation::Pointer(self.registers.get(HL)),
                            bit: One,
                        },
                        // BIT 1, A
                        0x4F => Instruction::Bit {
                            what: MemoryLocation::Register(A),
                            bit: One,
                        },
                        // BIT 2, B
                        0x50 => Instruction::Bit {
                            what: MemoryLocation::Register(B),
                            bit: Two,
                        },
                        // BIT 2, C
                        0x51 => Instruction::Bit {
                            what: MemoryLocation::Register(C),
                            bit: Two,
                        },
                        // BIT 2, D
                        0x52 => Instruction::Bit {
                            what: MemoryLocation::Register(D),
                            bit: Two,
                        },
                        // BIT 2, E
                        0x53 => Instruction::Bit {
                            what: MemoryLocation::Register(E),
                            bit: Two,
                        },
                        // BIT 2, H
                        0x54 => Instruction::Bit {
                            what: MemoryLocation::Register(H),
                            bit: Two,
                        },
                        // BIT 2, L
                        0x55 => Instruction::Bit {
                            what: MemoryLocation::Register(L),
                            bit: Two,
                        },
                        // BIT 2, (HL)
                        0x56 => Instruction::Bit {
                            what: MemoryLocation::Pointer(self.registers.get(HL)),
                            bit: Two,
                        },
                        // BIT 2, A
                        0x57 => Instruction::Bit {
                            what: MemoryLocation::Register(A),
                            bit: Two,
                        },
                        // BIT 3, B
                        0x58 => Instruction::Bit {
                            what: MemoryLocation::Register(B),
                            bit: Three,
                        },
                        // BIT 3, C
                        0x59 => Instruction::Bit {
                            what: MemoryLocation::Register(C),
                            bit: Three,
                        },
                        // BIT 3, D
                        0x5A => Instruction::Bit {
                            what: MemoryLocation::Register(D),
                            bit: Three,
                        },
                        // BIT 3, E
                        0x5B => Instruction::Bit {
                            what: MemoryLocation::Register(E),
                            bit: Three,
                        },
                        // BIT 3, H
                        0x5C => Instruction::Bit {
                            what: MemoryLocation::Register(H),
                            bit: Three,
                        },
                        // BIT 3, L
                        0x5D => Instruction::Bit {
                            what: MemoryLocation::Register(L),
                            bit: Three,
                        },
                        // BIT 3, (HL)
                        0x5E => Instruction::Bit {
                            what: MemoryLocation::Pointer(self.registers.get(HL)),
                            bit: Three,
                        },
                        // BIT 3, A
                        0x5F => Instruction::Bit {
                            what: MemoryLocation::Register(A),
                            bit: Three,
                        },
                        // BIT 4, B
                        0x60 => Instruction::Bit {
                            what: MemoryLocation::Register(B),
                            bit: Four,
                        },
                        // BIT 4, C
                        0x61 => Instruction::Bit {
                            what: MemoryLocation::Register(C),
                            bit: Four,
                        },
                        // BIT 4, D
                        0x62 => Instruction::Bit {
                            what: MemoryLocation::Register(D),
                            bit: Four,
                        },
                        // BIT 4, E
                        0x63 => Instruction::Bit {
                            what: MemoryLocation::Register(E),
                            bit: Four,
                        },
                        // BIT 4, H
                        0x64 => Instruction::Bit {
                            what: MemoryLocation::Register(H),
                            bit: Four,
                        },
                        // BIT 4, L
                        0x65 => Instruction::Bit {
                            what: MemoryLocation::Register(L),
                            bit: Four,
                        },
                        // BIT 4, (HL)
                        0x66 => Instruction::Bit {
                            what: MemoryLocation::Pointer(self.registers.get(HL)),
                            bit: Four,
                        },
                        // BIT 4, A
                        0x67 => Instruction::Bit {
                            what: MemoryLocation::Register(A),
                            bit: Four,
                        },
                        // BIT 5, B
                        0x68 => Instruction::Bit {
                            what: MemoryLocation::Register(B),
                            bit: Five,
                        },
                        // BIT 5, C
                        0x69 => Instruction::Bit {
                            what: MemoryLocation::Register(C),
                            bit: Five,
                        },
                        // BIT 5, D
                        0x6A => Instruction::Bit {
                            what: MemoryLocation::Register(D),
                            bit: Five,
                        },
                        // BIT 5, E
                        0x6B => Instruction::Bit {
                            what: MemoryLocation::Register(E),
                            bit: Five,
                        },
                        // BIT 5, H
                        0x6C => Instruction::Bit {
                            what: MemoryLocation::Register(H),
                            bit: Five,
                        },
                        // BIT 5, L
                        0x6D => Instruction::Bit {
                            what: MemoryLocation::Register(L),
                            bit: Five,
                        },
                        // BIT 5, (HL)
                        0x6E => Instruction::Bit {
                            what: MemoryLocation::Pointer(self.registers.get(HL)),
                            bit: Five,
                        },
                        // BIT 5, A
                        0x6F => Instruction::Bit {
                            what: MemoryLocation::Register(A),
                            bit: Five,
                        },
                        // BIT 6, B
                        0x70 => Instruction::Bit {
                            what: MemoryLocation::Register(B),
                            bit: Six,
                        },
                        // BIT 6, C
                        0x71 => Instruction::Bit {
                            what: MemoryLocation::Register(C),
                            bit: Six,
                        },
                        // BIT 6, D
                        0x72 => Instruction::Bit {
                            what: MemoryLocation::Register(D),
                            bit: Six,
                        },
                        // BIT 6, E
                        0x73 => Instruction::Bit {
                            what: MemoryLocation::Register(E),
                            bit: Six,
                        },
                        // BIT 6, H
                        0x74 => Instruction::Bit {
                            what: MemoryLocation::Register(H),
                            bit: Six,
                        },
                        // BIT 6, L
                        0x75 => Instruction::Bit {
                            what: MemoryLocation::Register(L),
                            bit: Six,
                        },
                        // BIT 6, (HL)
                        0x76 => Instruction::Bit {
                            what: MemoryLocation::Pointer(self.registers.get(HL)),
                            bit: Six,
                        },
                        // BIT 6, A
                        0x77 => Instruction::Bit {
                            what: MemoryLocation::Register(A),
                            bit: Six,
                        },
                        // BIT 7, B
                        0x78 => Instruction::Bit {
                            what: MemoryLocation::Register(B),
                            bit: Seven,
                        },
                        // BIT 7, C
                        0x79 => Instruction::Bit {
                            what: MemoryLocation::Register(C),
                            bit: Seven,
                        },
                        // BIT 7, D
                        0x7A => Instruction::Bit {
                            what: MemoryLocation::Register(D),
                            bit: Seven,
                        },
                        // BIT 7, E
                        0x7B => Instruction::Bit {
                            what: MemoryLocation::Register(E),
                            bit: Seven,
                        },
                        // BIT 7, H
                        0x7C => Instruction::Bit {
                            what: MemoryLocation::Register(H),
                            bit: Seven,
                        },
                        // BIT 7, L
                        0x7D => Instruction::Bit {
                            what: MemoryLocation::Register(L),
                            bit: Seven,
                        },
                        // BIT 7, (HL)
                        0x7E => Instruction::Bit {
                            what: MemoryLocation::Pointer(self.registers.get(HL)),
                            bit: Seven,
                        },
                        // BIT 7, A
                        0x7F => Instruction::Bit {
                            what: MemoryLocation::Register(A),
                            bit: Seven,
                        },
                        // RES 0, B
                        0x80 => Instruction::Res {
                            what: MemoryLocation::Register(B),
                            bit: Zero,
                        },
                        // RES 0, C
                        0x81 => Instruction::Res {
                            what: MemoryLocation::Register(C),
                            bit: Zero,
                        },
                        // RES 0, D
                        0x82 => Instruction::Res {
                            what: MemoryLocation::Register(D),
                            bit: Zero,
                        },
                        // Res 0, E
                        0x83 => Instruction::Res {
                            what: MemoryLocation::Register(E),
                            bit: Zero,
                        },
                        // Res 0, H
                        0x84 => Instruction::Res {
                            what: MemoryLocation::Register(H),
                            bit: Zero,
                        },
                        // RES 0, L
                        0x85 => Instruction::Res {
                            what: MemoryLocation::Register(L),
                            bit: Zero,
                        },
                        // RES 0, (HL)
                        0x86 => Instruction::Res {
                            what: MemoryLocation::Pointer(self.registers.get(HL)),
                            bit: Zero,
                        },
                        // RES 0, A
                        0x87 => Instruction::Res {
                            what: MemoryLocation::Register(A),
                            bit: Zero,
                        },
                        // RES 1, B
                        0x88 => Instruction::Res {
                            what: MemoryLocation::Register(B),
                            bit: One,
                        },
                        // RES 1, C
                        0x89 => Instruction::Res {
                            what: MemoryLocation::Register(C),
                            bit: One,
                        },
                        // RES 1, D
                        0x8A => Instruction::Res {
                            what: MemoryLocation::Register(D),
                            bit: One,
                        },
                        // RES 1, E
                        0x8B => Instruction::Res {
                            what: MemoryLocation::Register(E),
                            bit: One,
                        },
                        // RES 1, H
                        0x8C => Instruction::Res {
                            what: MemoryLocation::Register(H),
                            bit: One,
                        },
                        // RES 1, L
                        0x8D => Instruction::Res {
                            what: MemoryLocation::Register(L),
                            bit: One,
                        },
                        // RES 1, (HL)
                        0x8E => Instruction::Res {
                            what: MemoryLocation::Pointer(self.registers.get(HL)),
                            bit: One,
                        },
                        // RES 1, A
                        0x8F => Instruction::Res {
                            what: MemoryLocation::Register(A),
                            bit: One,
                        },
                        // RES 2, B
                        0x90 => Instruction::Res {
                            what: MemoryLocation::Register(B),
                            bit: Two,
                        },
                        // RES 2, C
                        0x91 => Instruction::Res {
                            what: MemoryLocation::Register(C),
                            bit: Two,
                        },
                        // RES 2, D
                        0x92 => Instruction::Res {
                            what: MemoryLocation::Register(D),
                            bit: Two,
                        },
                        // RES 2, E
                        0x93 => Instruction::Res {
                            what: MemoryLocation::Register(E),
                            bit: Two,
                        },
                        // RES 2, H
                        0x94 => Instruction::Res {
                            what: MemoryLocation::Register(H),
                            bit: Two,
                        },
                        // RES 2, L
                        0x95 => Instruction::Res {
                            what: MemoryLocation::Register(L),
                            bit: Two,
                        },
                        // RES 2, (HL)
                        0x96 => Instruction::Res {
                            what: MemoryLocation::Pointer(self.registers.get(HL)),
                            bit: Two,
                        },
                        // RES 2, A
                        0x97 => Instruction::Res {
                            what: MemoryLocation::Register(A),
                            bit: Two,
                        },
                        // RES 3, B
                        0x98 => Instruction::Res {
                            what: MemoryLocation::Register(B),
                            bit: Three,
                        },
                        // RES 3, C
                        0x99 => Instruction::Res {
                            what: MemoryLocation::Register(C),
                            bit: Three,
                        },
                        // RES 3, D
                        0x9A => Instruction::Res {
                            what: MemoryLocation::Register(D),
                            bit: Three,
                        },
                        // RES 3, E
                        0x9B => Instruction::Res {
                            what: MemoryLocation::Register(E),
                            bit: Three,
                        },
                        // RES 3, H
                        0x9C => Instruction::Res {
                            what: MemoryLocation::Register(H),
                            bit: Three,
                        },
                        // RES 3, L
                        0x9D => Instruction::Res {
                            what: MemoryLocation::Register(L),
                            bit: Three,
                        },
                        // RES 3, (HL)
                        0x9E => Instruction::Res {
                            what: MemoryLocation::Pointer(self.registers.get(HL)),
                            bit: Three,
                        },
                        // RES 3, A
                        0x9F => Instruction::Res {
                            what: MemoryLocation::Register(A),
                            bit: Three,
                        },
                        // RES 4, B
                        0xA0 => Instruction::Res {
                            what: MemoryLocation::Register(B),
                            bit: Four,
                        },
                        // RES 4, C
                        0xA1 => Instruction::Res {
                            what: MemoryLocation::Register(C),
                            bit: Four,
                        },
                        // RES 4, D
                        0xA2 => Instruction::Res {
                            what: MemoryLocation::Register(D),
                            bit: Four,
                        },
                        // RES 4, E
                        0xA3 => Instruction::Res {
                            what: MemoryLocation::Register(E),
                            bit: Four,
                        },
                        // RES 4, H
                        0xA4 => Instruction::Res {
                            what: MemoryLocation::Register(H),
                            bit: Four,
                        },
                        // RES 4, L
                        0xA5 => Instruction::Res {
                            what: MemoryLocation::Register(L),
                            bit: Four,
                        },
                        // RES 4, (HL)
                        0xA6 => Instruction::Res {
                            what: MemoryLocation::Pointer(self.registers.get(HL)),
                            bit: Four,
                        },
                        // RES 4, A
                        0xA7 => Instruction::Res {
                            what: MemoryLocation::Register(A),
                            bit: Four,
                        },
                        // RES 5, B
                        0xA8 => Instruction::Res {
                            what: MemoryLocation::Register(B),
                            bit: Five,
                        },
                        // RES 5, C
                        0xA9 => Instruction::Res {
                            what: MemoryLocation::Register(C),
                            bit: Five,
                        },
                        // RES 5, D
                        0xAA => Instruction::Res {
                            what: MemoryLocation::Register(D),
                            bit: Five,
                        },
                        // RES 5, E
                        0xAB => Instruction::Res {
                            what: MemoryLocation::Register(E),
                            bit: Five,
                        },
                        // RES 5, H
                        0xAC => Instruction::Res {
                            what: MemoryLocation::Register(H),
                            bit: Five,
                        },
                        // RES 5, L
                        0xAD => Instruction::Res {
                            what: MemoryLocation::Register(L),
                            bit: Five,
                        },
                        // RES 5, (HL)
                        0xAE => Instruction::Res {
                            what: MemoryLocation::Pointer(self.registers.get(HL)),
                            bit: Five,
                        },
                        // RES 5, A
                        0xAF => Instruction::Res {
                            what: MemoryLocation::Register(A),
                            bit: Five,
                        },
                        // RES 6, B
                        0xB0 => Instruction::Res {
                            what: MemoryLocation::Register(B),
                            bit: Six,
                        },
                        // RES 6, C
                        0xB1 => Instruction::Res {
                            what: MemoryLocation::Register(C),
                            bit: Six,
                        },
                        // RES 6, D
                        0xB2 => Instruction::Res {
                            what: MemoryLocation::Register(D),
                            bit: Six,
                        },
                        // RES 6, E
                        0xB3 => Instruction::Res {
                            what: MemoryLocation::Register(E),
                            bit: Six,
                        },
                        // RES 6, H
                        0xB4 => Instruction::Res {
                            what: MemoryLocation::Register(H),
                            bit: Six,
                        },
                        // RES 6, L
                        0xB5 => Instruction::Res {
                            what: MemoryLocation::Register(L),
                            bit: Six,
                        },
                        // RES 6, (HL)
                        0xB6 => Instruction::Res {
                            what: MemoryLocation::Pointer(self.registers.get(HL)),
                            bit: Six,
                        },
                        // RES 6, A
                        0xB7 => Instruction::Res {
                            what: MemoryLocation::Register(A),
                            bit: Six,
                        },
                        // RES 7, B
                        0xB8 => Instruction::Res {
                            what: MemoryLocation::Register(B),
                            bit: Seven,
                        },
                        // RES 7, C
                        0xB9 => Instruction::Res {
                            what: MemoryLocation::Register(C),
                            bit: Seven,
                        },
                        // RES 7, D
                        0xBA => Instruction::Res {
                            what: MemoryLocation::Register(D),
                            bit: Seven,
                        },
                        // RES 7, E
                        0xBB => Instruction::Res {
                            what: MemoryLocation::Register(E),
                            bit: Seven,
                        },
                        // RES 7, H
                        0xBC => Instruction::Res {
                            what: MemoryLocation::Register(H),
                            bit: Seven,
                        },
                        // RES 7, L
                        0xBD => Instruction::Res {
                            what: MemoryLocation::Register(L),
                            bit: Seven,
                        },
                        // RES 7, (HL)
                        0xBE => Instruction::Res {
                            what: MemoryLocation::Pointer(self.registers.get(HL)),
                            bit: Seven,
                        },
                        // RES 7, A
                        0xBF => Instruction::Res {
                            what: MemoryLocation::Register(A),
                            bit: Seven,
                        },
                        // SET 0, B
                        0xC0 => Instruction::Set {
                            what: MemoryLocation::Register(B),
                            bit: Zero,
                        },
                        // SET 0, C
                        0xC1 => Instruction::Set {
                            what: MemoryLocation::Register(C),
                            bit: Zero,
                        },
                        // SET 0, D
                        0xC2 => Instruction::Set {
                            what: MemoryLocation::Register(D),
                            bit: Zero,
                        },
                        // SET 0, E
                        0xC3 => Instruction::Set {
                            what: MemoryLocation::Register(E),
                            bit: Zero,
                        },
                        // SET 0, H
                        0xC4 => Instruction::Set {
                            what: MemoryLocation::Register(H),
                            bit: Zero,
                        },
                        // SET 0, L
                        0xC5 => Instruction::Set {
                            what: MemoryLocation::Register(L),
                            bit: Zero,
                        },
                        // SET 0, (HL)
                        0xC6 => Instruction::Set {
                            what: MemoryLocation::Pointer(self.registers.get(HL)),
                            bit: Zero,
                        },
                        // SET 0, A
                        0xC7 => Instruction::Set {
                            what: MemoryLocation::Register(A),
                            bit: Zero,
                        },
                        // SET 1, B
                        0xC8 => Instruction::Set {
                            what: MemoryLocation::Register(B),
                            bit: One,
                        },
                        // SET 1, C
                        0xC9 => Instruction::Set {
                            what: MemoryLocation::Register(C),
                            bit: One,
                        },
                        // SET 1, D
                        0xCA => Instruction::Set {
                            what: MemoryLocation::Register(D),
                            bit: One,
                        },
                        // SET 1, E
                        0xCB => Instruction::Set {
                            what: MemoryLocation::Register(E),
                            bit: One,
                        },
                        // SET 1, H
                        0xCC => Instruction::Set {
                            what: MemoryLocation::Register(H),
                            bit: One,
                        },
                        // SET 1, L
                        0xCD => Instruction::Set {
                            what: MemoryLocation::Register(L),
                            bit: One,
                        },
                        // SET 1, (HL)
                        0xCE => Instruction::Set {
                            what: MemoryLocation::Pointer(self.registers.get(HL)),
                            bit: One,
                        },
                        // SET 1, A
                        0xCF => Instruction::Set {
                            what: MemoryLocation::Register(A),
                            bit: One,
                        },
                        // SET 2, B
                        0xD0 => Instruction::Set {
                            what: MemoryLocation::Register(B),
                            bit: Two,
                        },
                        // SET 2, C
                        0xD1 => Instruction::Set {
                            what: MemoryLocation::Register(C),
                            bit: Two,
                        },
                        // SET 2, D
                        0xD2 => Instruction::Set {
                            what: MemoryLocation::Register(D),
                            bit: Two,
                        },
                        // SET 2, E
                        0xD3 => Instruction::Set {
                            what: MemoryLocation::Register(E),
                            bit: Two,
                        },
                        // SET 2, H
                        0xD4 => Instruction::Set {
                            what: MemoryLocation::Register(H),
                            bit: Two,
                        },
                        // SET 2, L
                        0xD5 => Instruction::Set {
                            what: MemoryLocation::Register(L),
                            bit: Two,
                        },
                        // SET 2, (HL)
                        0xD6 => Instruction::Set {
                            what: MemoryLocation::Pointer(self.registers.get(HL)),
                            bit: Two,
                        },
                        // SET 2, A
                        0xD7 => Instruction::Set {
                            what: MemoryLocation::Register(A),
                            bit: Two,
                        },
                        // SET 3, B
                        0xD8 => Instruction::Set {
                            what: MemoryLocation::Register(B),
                            bit: Three,
                        },
                        // SET 3, C
                        0xD9 => Instruction::Set {
                            what: MemoryLocation::Register(C),
                            bit: Three,
                        },
                        // SET 3, D
                        0xDA => Instruction::Set {
                            what: MemoryLocation::Register(D),
                            bit: Three,
                        },
                        // SET 3, E
                        0xDB => Instruction::Set {
                            what: MemoryLocation::Register(E),
                            bit: Three,
                        },
                        // SET 3, H
                        0xDC => Instruction::Set {
                            what: MemoryLocation::Register(H),
                            bit: Three,
                        },
                        // SET 3, L
                        0xDD => Instruction::Set {
                            what: MemoryLocation::Register(L),
                            bit: Three,
                        },
                        // SET 3, (HL)
                        0xDE => Instruction::Set {
                            what: MemoryLocation::Pointer(self.registers.get(HL)),
                            bit: Three,
                        },
                        // SET 3, A
                        0xDF => Instruction::Set {
                            what: MemoryLocation::Register(A),
                            bit: Three,
                        },
                        // SET 4, B
                        0xE0 => Instruction::Set {
                            what: MemoryLocation::Register(B),
                            bit: Four,
                        },
                        // SET 4, C
                        0xE1 => Instruction::Set {
                            what: MemoryLocation::Register(C),
                            bit: Four,
                        },
                        // SET 4, D
                        0xE2 => Instruction::Set {
                            what: MemoryLocation::Register(D),
                            bit: Four,
                        },
                        // SET 4, E
                        0xE3 => Instruction::Set {
                            what: MemoryLocation::Register(E),
                            bit: Four,
                        },
                        // SET 4, H
                        0xE4 => Instruction::Set {
                            what: MemoryLocation::Register(H),
                            bit: Four,
                        },
                        // SET 4, L
                        0xE5 => Instruction::Set {
                            what: MemoryLocation::Register(L),
                            bit: Four,
                        },
                        // SET 4, (HL)
                        0xE6 => Instruction::Set {
                            what: MemoryLocation::Pointer(self.registers.get(HL)),
                            bit: Four,
                        },
                        // SET 4, A
                        0xE7 => Instruction::Set {
                            what: MemoryLocation::Register(A),
                            bit: Four,
                        },
                        // SET 5, B
                        0xE8 => Instruction::Set {
                            what: MemoryLocation::Register(B),
                            bit: Five,
                        },
                        // SET 5, C
                        0xE9 => Instruction::Set {
                            what: MemoryLocation::Register(C),
                            bit: Five,
                        },
                        // SET 5, D
                        0xEA => Instruction::Set {
                            what: MemoryLocation::Register(D),
                            bit: Five,
                        },
                        // SET 5, E
                        0xEB => Instruction::Set {
                            what: MemoryLocation::Register(E),
                            bit: Five,
                        },
                        // SET 5, H
                        0xEC => Instruction::Set {
                            what: MemoryLocation::Register(H),
                            bit: Five,
                        },
                        // SET 5, L
                        0xED => Instruction::Set {
                            what: MemoryLocation::Register(L),
                            bit: Five,
                        },
                        // SET 5, (HL)
                        0xEE => Instruction::Set {
                            what: MemoryLocation::Pointer(self.registers.get(HL)),
                            bit: Five,
                        },
                        // SET 5, A
                        0xEF => Instruction::Set {
                            what: MemoryLocation::Register(A),
                            bit: Five,
                        },
                        // SET 6, B
                        0xF0 => Instruction::Set {
                            what: MemoryLocation::Register(B),
                            bit: Six,
                        },
                        // SET 6, C
                        0xF1 => Instruction::Set {
                            what: MemoryLocation::Register(C),
                            bit: Six,
                        },
                        // SET 6, D
                        0xF2 => Instruction::Set {
                            what: MemoryLocation::Register(D),
                            bit: Six,
                        },
                        // SET 6, E
                        0xF3 => Instruction::Set {
                            what: MemoryLocation::Register(E),
                            bit: Six,
                        },
                        // SET 6, H
                        0xF4 => Instruction::Set {
                            what: MemoryLocation::Register(H),
                            bit: Six,
                        },
                        // SET 6, L
                        0xF5 => Instruction::Set {
                            what: MemoryLocation::Register(L),
                            bit: Six,
                        },
                        // SET 6, (HL)
                        0xF6 => Instruction::Set {
                            what: MemoryLocation::Pointer(self.registers.get(HL)),
                            bit: Six,
                        },
                        // SET 6, A
                        0xF7 => Instruction::Set {
                            what: MemoryLocation::Register(A),
                            bit: Six,
                        },
                        // SET 7, B
                        0xF8 => Instruction::Set {
                            what: MemoryLocation::Register(B),
                            bit: Seven,
                        },
                        // SET 7, C
                        0xF9 => Instruction::Set {
                            what: MemoryLocation::Register(C),
                            bit: Seven,
                        },
                        // SET 7, D
                        0xFA => Instruction::Set {
                            what: MemoryLocation::Register(D),
                            bit: Seven,
                        },
                        // SET 7, E
                        0xFB => Instruction::Set {
                            what: MemoryLocation::Register(E),
                            bit: Seven,
                        },
                        // SET 7, H
                        0xFC => Instruction::Set {
                            what: MemoryLocation::Register(H),
                            bit: Seven,
                        },
                        // SET 7, L
                        0xFD => Instruction::Set {
                            what: MemoryLocation::Register(L),
                            bit: Seven,
                        },
                        // SET 7, (HL)
                        0xFE => Instruction::Set {
                            what: MemoryLocation::Pointer(self.registers.get(HL)),
                            bit: Seven,
                        },
                        // SET 7, A
                        0xFF => Instruction::Set {
                            what: MemoryLocation::Register(A),
                            bit: Seven,
                        },
                        _ => Instruction::Nop,
                    }
                } else {
                    panic!("Invalid Postfix OpCode value!")
                }
            }
            // CALL Z, a16
            0xCC => Instruction::Call(FlagOn(Z)),
            // CALL a16
            0xCD => Instruction::Call(Condition::None),
            // ADC A, d8
            0xCE => Instruction::Adc {
                to: MemoryLocation::Register(A),
                what: self.immediate_operand(false),
                cycles: 2,
                length: InstructionLength::Two,
            },
            // RST 1
            0xCF => Instruction::Rst(One),
            // RET CZ
            0xD0 => Instruction::Ret(FlagOff(Flag::C)),
            // POP DE
            0xD1 => Instruction::Pop(DE),
            // JP NC, a16
            0xD2 => Instruction::Jp {
                to: self.immediate_operand(true),
                condition: FlagOff(Flag::C),
                cycles: JumpCycles {
                    executed: 4,
                    not_executed: 3,
                },
                length: InstructionLength::Three,
            },
            // NO CODE
            0xD3 => panic!("Called 0xD3."),
            // CALL NC, a16
            0xD4 => Instruction::Call(FlagOff(Flag::C)),
            // PUSH DE
            0xD5 => Instruction::Push(DE),
            // SUB d8
            0xD6 => Instruction::Sub {
                from: MemoryLocation::Register(A),
                what: self.immediate_operand(false),
                cycles: 2,
                length: InstructionLength::Two,
            },
            // RST 2
            0xD7 => Instruction::Rst(Two),
            // RET C
            0xD8 => Instruction::Ret(FlagOn(Flag::C)),
            // RETI
            0xD9 => Instruction::Reti,
            // JP C, a16
            0xDA => Instruction::Jp {
                to: self.immediate_operand(true),
                condition: FlagOn(Flag::C),
                cycles: JumpCycles {
                    executed: 4,
                    not_executed: 3,
                },
                length: InstructionLength::Three,
            },
            // NO CODE
            0xDB => panic!("Called 0xDB."),
            // CALL C, a16
            0xDC => Instruction::Call(FlagOn(Flag::C)),
            // SBC A, d8
            0xDE => Instruction::Sbc {
                from: MemoryLocation::Register(A),
                what: self.immediate_operand(false),
                cycles: 2,
                length: InstructionLength::Two,
            },
            // RST 3
            0xDF => Instruction::Rst(Three),
            // LD (a8), A
            // INTERNAL PORT/MODE SWITCH
            0xE0 => Instruction::Load {
                to: MemoryLocation::Pointer(concat_values(
                    Value::EightBit(0xFF),
                    self.immediate_operand(false),
                )),
                what: self.registers.get(A),
                additional_instruction: AdditionalInstruction::None,
                cycles: 3,
                length: InstructionLength::Two,
            },
            0xE1 => Instruction::Pop(HL),
            // LD (C), A
            // INTERNAL PORT/MODE SWITCH
            0xE2 => Instruction::Load {
                to: MemoryLocation::Pointer(concat_values(
                    Value::EightBit(0xFF),
                    self.registers.get(C),
                )),
                what: self.registers.get(A),
                additional_instruction: AdditionalInstruction::None,
                cycles: 2,
                length: InstructionLength::One,
            },
            // PUSH HL
            0xE5 => Instruction::Push(HL),
            // AND D8
            0xE6 => Instruction::And {
                what: self.immediate_operand(false),
                cycles: 2,
                length: InstructionLength::Two,
            },
            // RST 4
            0xE7 => Instruction::Rst(Four),
            // ADD SP, s8
            0xE8 => todo!("Doing this one last."),
            // JP HL
            0xE9 => Instruction::Jp {
                to: self.registers.get(HL),
                condition: Condition::None,
                cycles: JumpCycles {
                    executed: 1,
                    not_executed: 1,
                },
                length: InstructionLength::One,
            },
            // LD (a16), A
            0xEA => Instruction::Load {
                to: MemoryLocation::Pointer(self.immediate_operand(true)),
                what: self.registers.get(A),
                additional_instruction: AdditionalInstruction::None,
                cycles: 4,
                length: InstructionLength::Three,
            },
            // NO CODE
            0xEB => panic!("Called 0xEB."),
            // NO CODE
            0xEC => panic!("Called 0xEC."),
            // NO CODE
            0xED => panic!("Called 0xED."),
            // XOR d8
            0xEE => Instruction::Xor {
                what: self.immediate_operand(false),
                cycles: 2,
                length: InstructionLength::Two,
            },
            // RST 5
            0xEF => Instruction::Rst(Five),
            // LD A, (a8)
            // INTERNAL PORT/MODE SWITCH
            0xF0 => Instruction::Load {
                to: MemoryLocation::Register(A),
                what: self.read(
                    concat_values(Value::EightBit(0xFF), self.immediate_operand(false)),
                    false,
                ),
                additional_instruction: AdditionalInstruction::None,
                cycles: 3,
                length: InstructionLength::Two,
            },
            // POP AF
            0xF1 => Instruction::Pop(AF),
            // LD (A), C
            // INTERNAL PORT/MODE SWITCH
            0xF2 => Instruction::Load {
                to: MemoryLocation::Register(A),
                what: self.read(
                    concat_values(Value::EightBit(0xFF), self.registers.get(C)),
                    false,
                ),
                additional_instruction: AdditionalInstruction::None,
                cycles: 2,
                length: InstructionLength::One,
            },
            // DI
            0xF3 => Instruction::Di,
            // NO CODE
            0xF4 => panic!("called 0xF4"),
            // PUSH AF
            0xF5 => Instruction::Push(AF),
            // OR d8
            0xF6 => Instruction::Or {
                what: self.immediate_operand(false),
                cycles: 2,
                length: InstructionLength::Two,
            },
            // RST 6
            0xF7 => Instruction::Rst(Six),
            // LD HL, SP+s8
            0xF8 => Instruction::Load {
                to: MemoryLocation::Register(HL),
                what: self.add_signed(
                    self.registers.get(SP),
                    unsigned_to_signed_8(self.immediate_operand(false)),
                ),
                additional_instruction: AdditionalInstruction::None,
                cycles: 3,
                length: InstructionLength::Two,
            },
            // LD SP, HL
            0xF9 => Instruction::Load {
                to: MemoryLocation::Register(SP),
                what: self.registers.get(HL),
                additional_instruction: AdditionalInstruction::None,
                cycles: 2,
                length: InstructionLength::One,
            },
            // LD A, (a16)
            0xFA => Instruction::Load {
                to: MemoryLocation::Register(A),
                what: self.read(self.immediate_operand(true), false),
                additional_instruction: AdditionalInstruction::None,
                cycles: 3,
                length: InstructionLength::Three,
            },
            // EI
            0xFB => Instruction::Ei,
            // NO CODE
            0xFC => panic!("called 0xFC"),
            // NO CODE
            0xFD => Instruction::Nop,
            // CP d8
            0xFE => Instruction::Cp {
                what: self.immediate_operand(false),
                cycles: 2,
                length: InstructionLength::Two,
            },
            // RST 7
            0xFF => Instruction::Rst(BitAddr::Seven),
            _ => Instruction::Nop,
        }
    }
}
