use crate::cpu::arithmetic::unsigned_to_signed;
use crate::cpu::flag::Flag;
use crate::cpu::flag::Flag::{C, H, N};
use crate::cpu::registers::Register;
use crate::cpu::registers::Register::{A, HL, PC, SP};
use crate::cpu::value::{concat_values, Value};
use crate::cpu::{MemoryLocation, CPU};

pub enum RotateDirection {
    Right,
    Left,
}

pub enum Condition {
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

pub enum RstAddress {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
}

impl RstAddress {
    fn new_pc(&self) -> Value {
        match self {
            RstAddress::Zero => Value::SixteenBit(0x0000),
            RstAddress::One => Value::SixteenBit(0x0008),
            RstAddress::Two => Value::SixteenBit(0x0010),
            RstAddress::Three => Value::SixteenBit(0x0018),
            RstAddress::Four => Value::SixteenBit(0x0020),
            RstAddress::Five => Value::SixteenBit(0x0028),
            RstAddress::Six => Value::SixteenBit(0x0030),
            RstAddress::Seven => Value::SixteenBit(0x0038),
        }
    }
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
    Shift {
        what: MemoryLocation,
        direction: RotateDirection,
        arithmetic: bool,
        cycles: u8,
        length: InstructionLength,
    },
    Jr {
        how_far: Value,
        condition: Condition,
        cycles: JumpCycles,
        length: InstructionLength,
    },
    Jp {
        to: Value,
        condition: Condition,
        cycles: JumpCycles,
        length: InstructionLength,
    },
    And {
        what: Value,
        cycles: u8,
        length: InstructionLength,
    },
    Xor {
        what: Value,
        cycles: u8,
        length: InstructionLength,
    },
    Or {
        what: Value,
        cycles: u8,
        length: InstructionLength,
    },
    Cp {
        what: Value,
        cycles: u8,
        length: InstructionLength,
    },
    Daa,
    Cpl,
    Scf,
    Ccf,
    Ret(Condition),
    Reti,
    Pop(Register),
    Push(Register),
    Call(Condition),
    Rst(RstAddress),
    Ei,
    Di,
    Nop,
}

pub enum PrefixedInstruction {
    Sla,
    Sra,
    Swap,
    Bit,
    Res,
    Set,
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
                self.inc_clock(cycles);
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
                self.inc_clock(cycles);
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
                self.inc_clock(cycles);
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
                self.inc_clock(cycles);
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
                self.inc_clock(cycles);
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
                self.inc_clock(cycles);
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
                self.inc_clock(cycles);
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
                        let result = match direction {
                            RotateDirection::Right => self.ror(val, use_carry),
                            RotateDirection::Left => self.rol(val, use_carry),
                        };
                        self.registers.set(reg, result);
                    }
                    MemoryLocation::Pointer(addr) => {
                        let val = self.read(addr, false);
                        let result = match direction {
                            RotateDirection::Right => self.ror(val, use_carry),
                            RotateDirection::Left => self.rol(val, use_carry),
                        };
                        self.write(addr, result);
                    }
                }
                self.inc_clock(cycles);
                self.registers.inc_pc(length.count());
            }
            Instruction::Shift {
                what,
                direction,
                arithmetic,
                cycles,
                length,
            } => {
                match what {
                    MemoryLocation::Register(reg) => {
                        let val = self.registers.get(reg);
                        let result = match direction {
                            RotateDirection::Right => self.shr(val, arithmetic),
                            RotateDirection::Left => self.shl(val),
                        };
                        self.registers.set(reg, result);
                    }
                    MemoryLocation::Pointer(addr) => {
                        let val = self.read(addr, false);
                        let result = match direction {
                            RotateDirection::Right => self.shr(val, arithmetic),
                            RotateDirection::Left => self.shl(val),
                        };
                        self.write(addr, result);
                    }
                }
                self.inc_clock(cycles);
                self.registers.inc_pc(length.count());
            }
            Instruction::Jr {
                how_far,
                condition,
                cycles,
                length,
            } => {
                if self.condition_passes(condition) {
                    let new_location = self.registers.get(PC) + unsigned_to_signed(how_far);
                    self.registers.set(PC, new_location);
                    self.inc_clock(cycles.executed);
                } else {
                    self.registers.inc_pc(length.count());
                    self.inc_clock(cycles.not_executed);
                }
            }
            Instruction::Jp {
                to,
                condition,
                cycles,
                length,
            } => {
                if self.condition_passes(condition) {
                    self.registers.set(PC, to);
                    self.inc_clock(cycles.executed);
                } else {
                    self.registers.inc_pc(length.count());
                    self.inc_clock(cycles.not_executed);
                }
            }
            Instruction::And {
                what,
                cycles,
                length,
            } => {
                let result = self.registers.get(A) & what;
                self.registers.set(A, result);
                // Apparently H is always set when running AND
                self.registers.f.set(H);
                self.registers.f.unset(N);
                self.registers.f.unset(C);
                self.check_zero_flag(result);
                self.registers.inc_pc(length.count());
                self.inc_clock(cycles);
            }
            Instruction::Xor {
                what,
                cycles,
                length,
            } => {
                let result = self.registers.get(A) ^ what;
                self.registers.set(A, result);
                self.registers.f.unset(N);
                self.registers.f.unset(C);
                self.registers.f.unset(H);
                self.check_zero_flag(result);
                self.registers.inc_pc(length.count());
                self.inc_clock(cycles);
            }
            Instruction::Or {
                what,
                cycles,
                length,
            } => {
                let result = self.registers.get(A) | what;
                self.registers.set(A, result);
                self.check_zero_flag(result);
                self.registers.f.unset(N);
                self.registers.f.unset(C);
                self.registers.f.unset(H);
                self.registers.inc_pc(length.count());
                self.inc_clock(cycles);
            }
            Instruction::Cp {
                what,
                cycles,
                length,
            } => {
                let result = self.sub(self.registers.get(A), what);
                self.check_zero_flag(result);
                self.registers.inc_pc(length.count());
                self.inc_clock(cycles);
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
                self.inc_clock(1);
                self.registers.f.set(H);
                self.registers.f.set(N);
                self.registers.inc_pc(1);
            }
            Instruction::Scf => {
                self.registers.f.set(C);
                self.inc_clock(1);
                self.registers.inc_pc(1);
            }
            Instruction::Ccf => {
                if self.registers.f.is_set(C) {
                    self.registers.f.unset(C);
                } else {
                    self.registers.f.set(C);
                }
                self.inc_clock(1);
                self.registers.inc_pc(1);
            }
            Instruction::Ret(condition) => {
                if self.condition_passes(condition) {
                    let lo = self.read(self.registers.get(SP), false);
                    self.registers.set(SP, self.registers.get(SP) + 1u16);

                    let hi = self.read(self.registers.get(SP), false);
                    self.registers.set(SP, self.registers.get(SP) + 1u16);

                    self.registers.set(PC, concat_values(hi, lo));
                    self.inc_clock(5);
                } else {
                    self.inc_clock(2);
                    self.registers.inc_pc(1);
                }
            }
            Instruction::Reti => {
                self.set_ime();
                let lo = self.read(self.registers.get(SP), false);
                self.registers.set(SP, self.registers.get(SP) + 1u16);

                let hi = self.read(self.registers.get(SP), false);
                self.registers.set(SP, self.registers.get(SP) + 1u16);

                self.registers.set(PC, concat_values(hi, lo));
                self.inc_clock(4);
            }
            Instruction::Pop(reg) => {
                let lo = self.read(self.registers.get(SP), false);
                self.registers.set(SP, self.registers.get(SP) + 1u16);

                let hi = self.read(self.registers.get(SP), false);
                self.registers.set(SP, self.registers.get(SP) + 1u16);

                self.registers.set(reg, concat_values(hi, lo));
                self.inc_clock(4);
                self.registers.inc_pc(1);
            }
            Instruction::Push(reg) => {
                self.registers.set(SP, self.registers.get(SP) - 1u16);
                self.write(self.registers.get(SP), self.registers.get(reg).high_byte());

                self.registers.set(SP, self.registers.get(SP) - 1u16);
                self.write(self.registers.get(SP), self.registers.get(reg).low_byte());

                self.inc_clock(4);
                self.registers.inc_pc(1);
            }
            Instruction::Call(condition) => {
                let pc_before_execution = self.registers.get(PC);
                self.registers.set(PC, pc_before_execution + 3u16);

                if self.condition_passes(condition) {
                    self.registers.set(SP, self.registers.get(SP) - 1u16);
                    self.write(self.registers.get(SP), self.registers.get(PC).high_byte());

                    self.registers.set(SP, self.registers.get(SP) - 1u16);
                    self.write(self.registers.get(SP), self.registers.get(PC).low_byte());

                    self.registers
                        .set(PC, self.read(pc_before_execution + 1u16, true));

                    self.inc_clock(6);
                } else {
                    self.inc_clock(3);
                }
            }
            Instruction::Rst(addr) => {
                // Inc PC before instruction
                self.registers.set(PC, self.registers.get(PC) + 1u16);

                self.registers.set(SP, self.registers.get(SP) - 1u16);
                self.write(self.registers.get(SP), self.registers.get(PC).high_byte());

                self.registers.set(SP, self.registers.get(SP) - 1u16);
                self.write(self.registers.get(SP), self.registers.get(PC).low_byte());

                self.registers.set(PC, addr.new_pc());
            }
            Instruction::Ei => {
                self.set_ime_next();
                self.registers.inc_pc(1);
                self.inc_clock(1)
            }
            Instruction::Di => {
                self.unset_ime();
                self.registers.inc_pc(1);
                self.inc_clock(1)
            }
            Instruction::Nop => {
                self.inc_clock(1);
                self.registers.inc_pc(1);
            }
        }
    }
}
