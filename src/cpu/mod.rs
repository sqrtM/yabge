use crate::cpu::memory_bus::MemoryBus;
use crate::cpu::registers::{Register, Registers};
use crate::cpu::value::Value;

pub mod flag;
mod instruction;
mod memory_bus;
mod opcode;
pub mod registers;
pub(crate) mod value;

#[derive(Default, Debug)]
pub struct CPU {
    registers: Registers,
    memory_bus: MemoryBus,
    clock: u64,
}

#[derive(Clone, Copy)]
pub enum MemoryLocation {
    Register(Register),
    Pointer(Value),
}

impl CPU {
    pub fn read(&self, addr: Value, two_bytes: bool) -> Value {
        let val = match addr {
            Value::EightBit(a) => a as u16,
            Value::SixteenBit(a) => a,
        };
        if two_bytes {
            let byte_one = self.memory_bus.read(val);
            let byte_two = self.memory_bus.read(val + 1);
            // When reading, the first bit is low, second is high
            Value::SixteenBit(concat_bytes(byte_two, byte_one))
        } else {
            Value::EightBit(self.memory_bus.read(val))
        }
    }

    pub fn write(&mut self, addr: Value, data: u8) {
        let val = match addr {
            Value::EightBit(a) => a as u16,
            Value::SixteenBit(a) => a,
        };
        self.memory_bus.write(val, data)
    }
}

pub(crate) fn concat_bytes(hi: u8, lo: u8) -> u16 {
    (hi as u16) << 8 | lo as u16
}

pub(crate) fn split_bytes(value: u16) -> (u8, u8) {
    let high_byte = (value >> 8) as u8;
    let low_byte = value as u8;
    (high_byte, low_byte)
}

#[cfg(test)]
mod tests {
    use crate::cpu::flag::Flag;
    use crate::cpu::registers::Register::BC;
    use crate::cpu::registers::Registers;
    use crate::cpu::value::Value;
    use crate::cpu::{concat_bytes, Register, CPU};

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
        assert_eq!(concat_bytes(0xAB, 0xCD), 0xABCD);
    }

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
