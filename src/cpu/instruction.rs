use crate::cpu::CPU;
use crate::cpu::opcode::Opcode;

pub struct Instruction {
    pub opcode: Opcode,
    pub cycles: u8,
}

impl CPU {
    pub fn lookup(code: u8) -> Instruction {
        match code {
            0x00 => {
                Instruction {
                    opcode: Opcode::NOP,
                    cycles: 4,
                }
            },
            _ => {
                Instruction {
                    opcode: Opcode::NOP,
                    cycles: 4,
                }
            }
        }
    }
}