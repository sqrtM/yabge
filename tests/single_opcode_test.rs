use yabge::cpu::flag::Flag;
use yabge::cpu::flag::Flag::Z;
use yabge::cpu::registers::Register::{A, B, BC, C, DE, HL, PC, SP};
use yabge::cpu::value::Value;
use yabge::cpu::CPU;

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
fn test_0x2a() {
    let mut cpu: CPU = Default::default();
    cpu.write(Value::SixteenBit(0x0000), Value::EightBit(0x2a));
    cpu.write(Value::SixteenBit(0xABCD), Value::EightBit(0xFE));
    cpu.registers.set(HL, Value::SixteenBit(0xABCD));

    let val = cpu.read(Value::SixteenBit(0x00), false);
    if let Value::EightBit(code) = val {
        let inst = cpu.lookup(code);
        cpu.execute(inst);
    }

    assert_eq!(cpu.registers.get(A), Value::EightBit(0xFE));
    assert_eq!(cpu.registers.get(HL), Value::SixteenBit(0xABCE));
}

#[test]
fn test_0x31() {
    let mut cpu: CPU = Default::default();
    cpu.write(Value::SixteenBit(0x0000), Value::EightBit(0x31));
    cpu.write(Value::SixteenBit(0x0001), Value::SixteenBit(0xFACE));
    cpu.registers.set(SP, Value::SixteenBit(0xCAFE));

    let val = cpu.read(Value::SixteenBit(0x00), false);
    if let Value::EightBit(code) = val {
        let inst = cpu.lookup(code);
        cpu.execute(inst);
    }

    assert_eq!(cpu.registers.get(SP), Value::SixteenBit(0xFACE));
}

#[test]
fn test_0x36() {
    let mut cpu: CPU = Default::default();
    cpu.write(Value::SixteenBit(0x0000), Value::EightBit(0x36));
    cpu.write(Value::SixteenBit(0x0001), Value::EightBit(0xAB));
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
    assert_eq!(
        cpu.read(cpu.registers.get(HL), false),
        Value::EightBit(0xAB)
    );

    // -- // -- // -- //

    let mut cpu: CPU = Default::default();
    cpu.write(Value::SixteenBit(0x00), Value::EightBit(0x36));
    cpu.write(Value::SixteenBit(0x01), Value::EightBit(0x28));
    cpu.registers.set(HL, Value::SixteenBit(0x4444));

    let val = cpu.read(Value::SixteenBit(0x00), false);
    if let Value::EightBit(code) = val {
        let inst = cpu.lookup(code);
        cpu.execute(inst);
    }

    assert_eq!(
        cpu.read(Value::SixteenBit(0x4444), false),
        Value::EightBit(0x28)
    );
    assert_eq!(
        cpu.read(cpu.registers.get(HL), false),
        Value::EightBit(0x28)
    );
}

#[test]
fn test_0x37() {
    let mut cpu: CPU = Default::default();
    cpu.write(Value::SixteenBit(0x0000), Value::EightBit(0x37));

    let val = cpu.read(Value::SixteenBit(0x00), false);
    if let Value::EightBit(code) = val {
        let inst = cpu.lookup(code);
        cpu.execute(inst);
    }

    assert!(cpu.registers.f.is_set(Flag::C));
}
