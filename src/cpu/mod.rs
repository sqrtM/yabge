pub mod flag;
mod opcode;
pub mod registers;

#[cfg(test)]
mod tests {
    use crate::cpu::flag::Flag;
    use crate::cpu::registers::{Register, Registers};

    #[test]
    fn test_af() {
        let mut cpu = Registers::default();
        cpu.f.set(Flag::H);
        cpu.f.set(Flag::C);
        cpu.set(Register::A, 0xAB);
        assert_eq!(cpu.af(), 0xAB03);
    }

    #[test]
    fn test_bc() {
        let mut cpu = Registers::default();
        cpu.set(Register::B, 0x12);
        cpu.set(Register::C, 0x34);
        assert_eq!(cpu.bc(), 0x1234);
    }

    #[test]
    fn test_de() {
        let mut cpu = Registers::default();
        cpu.set(Register::D, 0x56);
        cpu.set(Register::E, 0x78);
        assert_eq!(cpu.de(), 0x5678);
    }

    #[test]
    fn test_hl() {
        let mut cpu = Registers::default();
        cpu.set(Register::H, 0xAB);
        cpu.set(Register::L, 0xCD);
        assert_eq!(cpu.hl(), 0xABCD);
    }

    #[test]
    fn test_merge_registers() {
        assert_eq!(Registers::merge_registers(0xAB, 0xCD), 0xABCD);
    }
}
