use crate::cpu::registers::{Register, Registers};
use crate::cpu::value::Value;

pub mod flag;
mod instruction;
mod opcode;
pub mod registers;
mod value;

#[derive(Default, Debug)]
pub struct CPU {
    registers: Registers,
    clock: u64,
}

#[derive(Clone, Copy)]
pub enum MemoryLocation {
    Register(Register),
    Pointer(Value),
}

#[cfg(test)]
mod tests {
    use crate::cpu::flag::Flag;
    use crate::cpu::registers::Registers;
    use crate::cpu::value::Value;
    use crate::cpu::Register;

    #[test]
    fn test_af() {
        let mut cpu = Registers::default();
        cpu.f.set(Flag::H);
        cpu.f.set(Flag::C);
        cpu.set(Register::A, Value::EightBit(0xAB));
        assert_eq!(cpu.af(), 0xAB03);
    }

    #[test]
    fn test_bc() {
        let mut cpu = Registers::default();
        cpu.set(Register::B, Value::EightBit(0x12));
        cpu.set(Register::C, Value::EightBit(0x34));
        assert_eq!(cpu.bc(), 0x1234);
    }

    #[test]
    fn test_de() {
        let mut cpu = Registers::default();
        cpu.set(Register::D, Value::EightBit(0x56));
        cpu.set(Register::E, Value::EightBit(0x78));
        assert_eq!(cpu.de(), 0x5678);
    }

    #[test]
    fn test_hl() {
        let mut cpu = Registers::default();
        cpu.set(Register::H, Value::EightBit(0xAB));
        cpu.set(Register::L, Value::EightBit(0xCD));
        assert_eq!(cpu.hl(), 0xABCD);
    }

    #[test]
    fn test_merge_registers() {
        assert_eq!(Registers::concat_bytes(0xAB, 0xCD), 0xABCD);
    }
}
