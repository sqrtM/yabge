use crate::cpu::arithmetic::unsigned_to_signed;
use crate::cpu::flag::Flag;
use crate::cpu::flag::Flag::{C, H, N};
use crate::cpu::registers::Register::{A, HL, PC};
use crate::cpu::value::Value;
use crate::cpu::{MemoryLocation, CPU};

pub enum RotateDirection {
    Right,
    Left,
}

pub enum JumpCondition {
    FlagOn(Flag),
    FlagOff(Flag),
    None,
}

pub struct JumpCycles {
    pub executed: u8,
    pub not_executed: u8,
}

pub enum AdditionalInstruction {
    Inc,
    Dec,
    None,
}

pub enum Instruction {
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
    Cpl,
    Scf,
    Ccf,
    Nop,
}

pub enum InstructionLength {
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
                        self.registers.set(HL, self.registers.get(HL) + 1u8)
                    }
                    AdditionalInstruction::Dec => {
                        self.registers.set(HL, self.registers.get(HL) - 1u8)
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
                self.registers.inc_pc(1);
            }
            Instruction::Cpl => {
                self.registers.set(A, !self.registers.get(A));
                self.clock += 1;
                self.registers.f.set(H);
                self.registers.f.set(N);
                self.registers.inc_pc(1);
            }
            Instruction::Scf => {
                self.registers.f.set(C);
                self.clock += 1;
                self.registers.inc_pc(1);
            }
            Instruction::Ccf => {
                if self.registers.f.is_set(C) {
                    self.registers.f.unset(C);
                } else {
                    self.registers.f.set(C);
                }
                self.clock += 1;
                self.registers.inc_pc(1);
            }
            Instruction::Nop => {
                self.clock += 1;
                self.registers.inc_pc(1);
            }
        }
    }
}
