use yabge::cpu::registers::Register;
use yabge::cpu::registers::Register::PC;
use yabge::cpu::value::Value;
use yabge::cpu::CPU;

#[test]
fn test_opcodes_1() {
    let mut cpu = CPU::default();

    cpu.write(Value::SixteenBit(0x0000), Value::EightBit(0x21)); // LD HL, d16
    cpu.write(Value::SixteenBit(0x0001), Value::SixteenBit(0xABCD)); // HL = 0xABCD
    cpu.write(Value::SixteenBit(0x0003), Value::EightBit(0x18)); // JR s8
    cpu.write(Value::SixteenBit(0x0004), Value::EightBit(0x30)); // s8 = 0x30
    cpu.write(Value::SixteenBit(0x0034), Value::EightBit(0x3C)); // INC A
    cpu.write(Value::SixteenBit(0x0035), Value::EightBit(0xFD)); // END

    let mut current_code = Value::EightBit(0x00);
    while current_code != Value::EightBit(0xFD) {
        current_code = cpu.read(cpu.registers.get(PC), false);
        if let Value::EightBit(code) = current_code {
            let inst = cpu.lookup(code);
            cpu.execute(inst);
        }
    }

    assert_eq!(cpu.registers.get(Register::HL), Value::SixteenBit(0xABCD));
    assert_eq!(cpu.registers.get(Register::A), Value::EightBit(0x01));
}
