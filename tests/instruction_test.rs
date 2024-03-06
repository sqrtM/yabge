use yabge::cpu::flag::Flag::{C, H, N, Z};
use yabge::cpu::instruction::Condition::FlagOn;
use yabge::cpu::instruction::RstAddress::{Three, Two};
use yabge::cpu::instruction::{
    AdditionalInstruction, Condition, Instruction, InstructionLength, JumpCycles, RotateDirection,
};
use yabge::cpu::registers::Register;
use yabge::cpu::registers::Register::{A, AF, B, BC, HL, PC, SP};
use yabge::cpu::value::Value;
use yabge::cpu::{MemoryLocation, CPU};

#[test]
fn test_load_reg() {
    let mut cpu: CPU = Default::default();
    cpu.registers.set(HL, Value::SixteenBit(0x1234));

    let instruction = Instruction::Load {
        to: MemoryLocation::Register(SP),
        what: cpu.registers.get(HL),
        additional_instruction: AdditionalInstruction::None,
        cycles: 1,
        length: InstructionLength::One,
    };
    cpu.execute(instruction);

    assert_eq!(cpu.registers.get(SP), Value::SixteenBit(0x1234));
    assert!(!cpu.registers.f.is_set(C));
    assert!(!cpu.registers.f.is_set(H));
}

#[test]
fn test_add() {
    let mut cpu: CPU = Default::default();

    cpu.registers.set(A, Value::EightBit(0x3E));
    cpu.registers.set(Register::B, Value::EightBit(0x23));

    let instruction = Instruction::Add {
        to: MemoryLocation::Register(A),
        what: cpu.registers.get(Register::B),
        cycles: 4,
        length: InstructionLength::One,
    };
    cpu.execute(instruction);

    assert_eq!(cpu.registers.get(A), Value::EightBit(0x61));
    assert!(!cpu.registers.f.is_set(C));
    assert!(cpu.registers.f.is_set(H));
}

#[test]
fn test_adc_with_carry() {
    let mut cpu: CPU = Default::default();
    cpu.registers.f.set(C);
    cpu.registers.set(A, Value::EightBit(0x3E));
    cpu.registers.set(Register::B, Value::EightBit(0x23));
    let instruction = Instruction::Adc {
        to: MemoryLocation::Register(A),
        what: cpu.registers.get(Register::B),
        cycles: 4,
        length: InstructionLength::One,
    };
    cpu.execute(instruction);

    assert_eq!(cpu.registers.get(A), Value::EightBit(0x62));
    assert!(!cpu.registers.f.is_set(C));
    assert!(cpu.registers.f.is_set(H));
}

#[test]
fn test_adc_no_carry() {
    let mut cpu: CPU = Default::default();
    cpu.registers.set(A, Value::EightBit(0x3E));
    cpu.registers.set(Register::B, Value::EightBit(0x23));
    let instruction = Instruction::Adc {
        to: MemoryLocation::Register(A),
        what: cpu.registers.get(Register::B),
        cycles: 4,
        length: InstructionLength::One,
    };
    cpu.execute(instruction);

    assert_eq!(cpu.registers.get(A), Value::EightBit(0x61));
    assert!(!cpu.registers.f.is_set(C));
    assert!(cpu.registers.f.is_set(H));
}

#[test]
fn test_sub() {
    let mut cpu = CPU::default();

    cpu.registers.set(A, Value::EightBit(0xF2));
    cpu.registers.set(Register::B, Value::EightBit(0x1F));

    let instruction = Instruction::Sub {
        from: MemoryLocation::Register(A),
        what: cpu.registers.get(Register::B),
        cycles: 4,
        length: InstructionLength::One,
    };
    cpu.execute(instruction);

    assert_eq!(cpu.registers.get(A), Value::EightBit(0xD3));
    assert!(!cpu.registers.f.is_set(C));
    assert!(cpu.registers.f.is_set(H));
}

#[test]
fn test_sub_to_zero() {
    let mut cpu = CPU::default();

    cpu.registers.set(A, Value::EightBit(0x50));
    cpu.registers.set(Register::B, Value::EightBit(0x50));

    let instruction = Instruction::Sub {
        from: MemoryLocation::Register(A),
        what: cpu.registers.get(Register::B),
        cycles: 4,
        length: InstructionLength::One,
    };
    cpu.execute(instruction);

    assert_eq!(cpu.registers.get(A), Value::EightBit(0x00));
    assert!(!cpu.registers.f.is_set(C));
    assert!(!cpu.registers.f.is_set(H));
    assert!(cpu.registers.f.is_set(Z));
}

#[test]
fn test_sbc_with_carry() {
    let mut cpu: CPU = Default::default();
    cpu.registers.f.set(C);
    cpu.registers.set(A, Value::EightBit(0x3E));
    cpu.registers.set(Register::B, Value::EightBit(0x23));
    let instruction = Instruction::Sbc {
        from: MemoryLocation::Register(A),
        what: cpu.registers.get(Register::B),
        cycles: 4,
        length: InstructionLength::One,
    };
    cpu.execute(instruction);

    assert_eq!(cpu.registers.get(A), Value::EightBit(0x1A));
    assert!(!cpu.registers.f.is_set(C));
    assert!(!cpu.registers.f.is_set(H));
}

#[test]
fn test_sbc_no_carry() {
    let mut cpu: CPU = Default::default();
    cpu.registers.set(A, Value::EightBit(0x3E));
    cpu.registers.set(Register::B, Value::EightBit(0x23));
    let instruction = Instruction::Sbc {
        from: MemoryLocation::Register(A),
        what: cpu.registers.get(Register::B),
        cycles: 4,
        length: InstructionLength::One,
    };
    cpu.execute(instruction);

    assert_eq!(cpu.registers.get(A), Value::EightBit(0x1B));
    assert!(!cpu.registers.f.is_set(C));
    assert!(!cpu.registers.f.is_set(H));
}

#[test]
fn test_inc_eight_bit() {
    let mut cpu: CPU = Default::default();
    cpu.registers.set(A, Value::EightBit(0x3E));

    let instruction = Instruction::Inc {
        what: MemoryLocation::Register(A),
        cycles: 4,
    };
    cpu.execute(instruction);

    assert_eq!(cpu.registers.get(A), Value::EightBit(0x3F));
    assert!(!cpu.registers.f.is_set(N));
}

#[test]
fn test_inc_sixteen_bit() {
    let mut cpu: CPU = Default::default();
    cpu.registers
        .set(Register::BC, Value::SixteenBit(0b0000_0000_1111_1111));

    let instruction = Instruction::Inc {
        what: MemoryLocation::Register(Register::BC),
        cycles: 4,
    };
    cpu.execute(instruction);

    assert_eq!(
        cpu.registers.get(Register::BC),
        Value::SixteenBit(0b0000_0001_0000_0000)
    );
    assert!(cpu.registers.f.is_set(H));
    assert!(!cpu.registers.f.is_set(N));
    assert!(!cpu.registers.f.is_set(C));
}

#[test]
fn test_dec_eight_bit() {
    let mut cpu: CPU = Default::default();
    cpu.registers.set(A, Value::EightBit(0x00));

    let instruction = Instruction::Dec {
        what: MemoryLocation::Register(A),
        cycles: 4,
    };
    cpu.execute(instruction);

    assert_eq!(cpu.registers.get(A), Value::EightBit(0xFF));
    assert!(cpu.registers.f.is_set(N));
}

#[test]
fn test_dec_sixteen_bit() {
    let mut cpu: CPU = Default::default();
    cpu.registers.set(Register::BC, Value::SixteenBit(0x1234));

    let instruction = Instruction::Dec {
        what: MemoryLocation::Register(Register::BC),
        cycles: 4,
    };
    cpu.execute(instruction);

    assert_eq!(cpu.registers.get(Register::BC), Value::SixteenBit(0x1233));
    assert!(cpu.registers.f.is_set(N));
}

#[test]
fn test_rotate_left() {
    let mut cpu: CPU = Default::default();
    cpu.registers.set(A, Value::EightBit(0b1100_0011));
    let instruction = Instruction::Rot {
        what: MemoryLocation::Register(A),
        direction: RotateDirection::Left,
        use_carry: false,
        cycles: 4,
        length: InstructionLength::Two,
    };
    cpu.execute(instruction);
    assert_eq!(cpu.registers.get(A), Value::EightBit(0b1000_0111));
    assert!(cpu.registers.f.is_set(C));
}

#[test]
fn test_rotate_right_carry() {
    let mut cpu: CPU = Default::default();
    cpu.registers.set(B, Value::EightBit(0b1100_0010));
    cpu.registers.f.set(C);
    let instruction = Instruction::Rot {
        what: MemoryLocation::Register(B),
        direction: RotateDirection::Right,
        use_carry: true,
        cycles: 4,
        length: InstructionLength::One,
    };
    cpu.execute(instruction);
    assert_eq!(cpu.registers.get(B), Value::EightBit(0b1110_0001));
    assert!(!cpu.registers.f.is_set(C));
}

#[test]
fn test_rotate_left_carry() {
    let mut cpu: CPU = Default::default();
    cpu.registers.set(A, Value::EightBit(0b0100_1000));
    cpu.registers.f.set(C);
    let instruction = Instruction::Rot {
        what: MemoryLocation::Register(A),
        direction: RotateDirection::Left,
        use_carry: true,
        cycles: 4,
        length: InstructionLength::Two,
    };
    cpu.execute(instruction);
    assert_eq!(cpu.registers.get(A), Value::EightBit(0b1001_0001));
    assert!(!cpu.registers.f.is_set(C));
}

#[test]
fn test_jr() {
    let mut cpu: CPU = Default::default();
    cpu.registers.set(PC, Value::SixteenBit(0x1234));
    cpu.registers.f.set(C);

    let instruction = Instruction::Jr {
        how_far: Value::EightBit(0b1001_1001),
        condition: Condition::FlagOn(C),
        cycles: JumpCycles {
            executed: 2,
            not_executed: 3,
        },
        length: InstructionLength::Two,
    };
    cpu.execute(instruction);
    assert_eq!(cpu.registers.get(PC), Value::SixteenBit(0x11CD));
    assert!(cpu.registers.f.is_set(C));
}

#[test]
fn test_jp() {
    let mut cpu: CPU = Default::default();
    cpu.registers.set(PC, Value::SixteenBit(0x1234));

    let instruction = Instruction::Jp {
        to: Value::SixteenBit(0x5678),
        condition: Condition::FlagOff(Z),
        cycles: JumpCycles {
            executed: 2,
            not_executed: 3,
        },
        length: InstructionLength::Two,
    };
    cpu.execute(instruction);
    assert_eq!(cpu.registers.get(PC), Value::SixteenBit(0x5678));
}

#[test]
fn test_daa_after_add_1() {
    let mut cpu: CPU = Default::default();

    // ADD
    cpu.registers.set(A, Value::EightBit(0x54));
    cpu.registers.set(Register::B, Value::EightBit(0x28));

    let instruction = Instruction::Add {
        to: MemoryLocation::Register(A),
        what: cpu.registers.get(Register::B),
        cycles: 4,
        length: InstructionLength::One,
    };
    cpu.execute(instruction);

    assert_eq!(cpu.registers.get(A), Value::EightBit(0x7C));
    assert!(!cpu.registers.f.is_set(C));

    // DAA Correction
    cpu.execute(Instruction::Daa);
    assert_eq!(cpu.registers.get(A), Value::EightBit(0x82));
    assert!(!cpu.registers.f.is_set(C));
}

#[test]
fn test_daa_after_add_2() {
    let mut cpu: CPU = Default::default();

    // ADD
    cpu.registers.set(A, Value::EightBit(0x98));
    cpu.registers.set(Register::B, Value::EightBit(0x04));

    let instruction = Instruction::Add {
        to: MemoryLocation::Register(A),
        what: cpu.registers.get(Register::B),
        cycles: 4,
        length: InstructionLength::One,
    };
    cpu.execute(instruction);

    assert_eq!(cpu.registers.get(A), Value::EightBit(0x9C));
    assert!(!cpu.registers.f.is_set(C));
    assert!(!cpu.registers.f.is_set(H));

    // DAA Correction
    cpu.execute(Instruction::Daa);
    assert_eq!(cpu.registers.get(A), Value::EightBit(0x02));
    assert!(cpu.registers.f.is_set(C));
    assert!(!cpu.registers.f.is_set(H));
}

#[test]
fn test_daa_after_add_3() {
    let mut cpu: CPU = Default::default();

    // ADD
    cpu.registers.set(A, Value::EightBit(0x80));
    cpu.registers.set(Register::B, Value::EightBit(0x90));

    let instruction = Instruction::Add {
        to: MemoryLocation::Register(A),
        what: cpu.registers.get(Register::B),
        cycles: 4,
        length: InstructionLength::One,
    };
    cpu.execute(instruction);

    assert_eq!(cpu.registers.get(A), Value::EightBit(0x10));
    assert!(cpu.registers.f.is_set(C));

    // DAA Correction
    cpu.execute(Instruction::Daa);
    assert_eq!(cpu.registers.get(A), Value::EightBit(0x70));
}

#[test]
fn test_daa_after_add_4() {
    let mut cpu: CPU = Default::default();

    // ADD
    cpu.registers.set(A, Value::EightBit(0x19));
    cpu.registers.set(Register::B, Value::EightBit(0x28));

    let instruction = Instruction::Add {
        to: MemoryLocation::Register(A),
        what: cpu.registers.get(Register::B),
        cycles: 4,
        length: InstructionLength::One,
    };
    cpu.execute(instruction);

    assert_eq!(cpu.registers.get(A), Value::EightBit(0x41));
    assert!(cpu.registers.f.is_set(H));
    assert!(!cpu.registers.f.is_set(C));

    // DAA Correction
    cpu.execute(Instruction::Daa);
    assert_eq!(cpu.registers.get(A), Value::EightBit(0x47));
}

#[test]
fn test_daa_after_sub_1() {
    let mut cpu: CPU = Default::default();

    // SUB
    cpu.registers.set(A, Value::EightBit(0x47));
    cpu.registers.set(Register::B, Value::EightBit(0x28));

    let instruction = Instruction::Sub {
        from: MemoryLocation::Register(A),
        what: cpu.registers.get(Register::B),
        cycles: 4,
        length: InstructionLength::One,
    };
    cpu.execute(instruction);

    assert_eq!(cpu.registers.get(A), Value::EightBit(0x1F));
    assert!(cpu.registers.f.is_set(H));
    assert!(!cpu.registers.f.is_set(C));

    // DAA Correction
    cpu.execute(Instruction::Daa);
    assert_eq!(cpu.registers.get(A), Value::EightBit(0x19));
}

#[test]
fn test_daa_after_sub_2() {
    let mut cpu: CPU = Default::default();

    // SUB
    cpu.registers.set(A, Value::EightBit(0x20));
    cpu.registers.set(Register::B, Value::EightBit(0x13));

    let instruction = Instruction::Sub {
        from: MemoryLocation::Register(A),
        what: cpu.registers.get(Register::B),
        cycles: 4,
        length: InstructionLength::One,
    };
    cpu.execute(instruction);

    assert_eq!(cpu.registers.get(A), Value::EightBit(0x0D));
    assert!(cpu.registers.f.is_set(H));
    assert!(!cpu.registers.f.is_set(C));

    // DAA Correction
    cpu.execute(Instruction::Daa);
    assert_eq!(cpu.registers.get(A), Value::EightBit(0x07));
}

#[test]
fn test_daa_after_sub_3() {
    let mut cpu: CPU = Default::default();

    // SUB
    cpu.registers.set(A, Value::EightBit(0x05));
    cpu.registers.set(Register::B, Value::EightBit(0x21));

    let instruction = Instruction::Sub {
        from: MemoryLocation::Register(A),
        what: cpu.registers.get(Register::B),
        cycles: 4,
        length: InstructionLength::One,
    };
    cpu.execute(instruction);

    assert_eq!(cpu.registers.get(A), Value::EightBit(0xE4));
    assert!(!cpu.registers.f.is_set(H));
    assert!(cpu.registers.f.is_set(C));

    // DAA Correction
    cpu.execute(Instruction::Daa);
    assert_eq!(cpu.registers.get(A), Value::EightBit(0x84));
}

#[test]
fn test_cpl() {
    let mut cpu: CPU = Default::default();
    cpu.registers.set(A, Value::EightBit(0b0011_0101));
    cpu.execute(Instruction::Cpl);

    assert_eq!(cpu.registers.get(A), Value::EightBit(0b1100_1010));
}

#[test]
fn test_nop() {
    let mut cpu: CPU = Default::default();
    let instruction = Instruction::Nop;
    cpu.execute(instruction);

    assert_eq!(cpu.registers.get(PC), Value::SixteenBit(1));
}

#[test]
fn test_rotate_right() {
    let mut cpu: CPU = Default::default();
    cpu.registers.set(A, Value::EightBit(0b1100_0011));
    let instruction = Instruction::Rot {
        what: MemoryLocation::Register(A),
        direction: RotateDirection::Right,
        use_carry: false,
        cycles: 4,
        length: InstructionLength::One,
    };
    cpu.execute(instruction);
    assert_eq!(cpu.registers.get(A), Value::EightBit(0b1110_0001));
    assert!(cpu.registers.f.is_set(C));
}

#[test]
fn test_and() {
    let mut cpu: CPU = Default::default();
    cpu.registers.set(A, Value::EightBit(0b1100_0011));
    cpu.registers.set(B, Value::EightBit(0b0100_1110));
    let instruction = Instruction::And {
        what: cpu.registers.get(B),
        cycles: 1,
        length: InstructionLength::One,
    };
    cpu.execute(instruction);
    assert_eq!(cpu.registers.get(A), Value::EightBit(0b0100_0010));

    // -- // -- // -- //
    let mut cpu: CPU = Default::default();
    cpu.registers.set(A, Value::EightBit(0b1100_0011));
    cpu.registers.set(B, Value::EightBit(0b0011_1100));
    let instruction = Instruction::And {
        what: cpu.registers.get(B),
        cycles: 1,
        length: InstructionLength::One,
    };
    cpu.execute(instruction);
    assert_eq!(cpu.registers.get(A), Value::EightBit(0));
    assert!(cpu.registers.f.is_set(Z))
}

#[test]
fn test_xor() {
    let mut cpu: CPU = Default::default();
    cpu.registers.set(A, Value::EightBit(0b1100_0011));
    cpu.registers.set(B, Value::EightBit(0b0100_1110));
    let instruction = Instruction::Xor {
        what: cpu.registers.get(B),
        cycles: 1,
        length: InstructionLength::One,
    };
    cpu.execute(instruction);
    assert_eq!(cpu.registers.get(A), Value::EightBit(0b1000_1101));

    // -- // -- // -- //
    let mut cpu: CPU = Default::default();
    cpu.registers.set(A, Value::EightBit(0b1111_0111));
    cpu.registers.set(B, Value::EightBit(0b1011_1100));
    let instruction = Instruction::Xor {
        what: cpu.registers.get(B),
        cycles: 1,
        length: InstructionLength::One,
    };
    cpu.execute(instruction);
    assert_eq!(cpu.registers.get(A), Value::EightBit(0b0100_1011));
}

#[test]
fn test_or() {
    let mut cpu: CPU = Default::default();
    cpu.registers.set(A, Value::EightBit(0b1100_0011));
    cpu.registers.set(B, Value::EightBit(0b0100_1110));
    let instruction = Instruction::Or {
        what: cpu.registers.get(B),
        cycles: 1,
        length: InstructionLength::One,
    };
    cpu.execute(instruction);
    assert_eq!(cpu.registers.get(A), Value::EightBit(0b1100_1111));

    // -- // -- // -- //
    let mut cpu: CPU = Default::default();
    cpu.registers.set(A, Value::EightBit(0b1111_0111));
    cpu.registers.set(B, Value::EightBit(0b1011_1100));
    let instruction = Instruction::Or {
        what: cpu.registers.get(B),
        cycles: 1,
        length: InstructionLength::One,
    };
    cpu.execute(instruction);
    assert_eq!(cpu.registers.get(A), Value::EightBit(0b1111_1111));
}

#[test]
fn test_cp() {
    let mut cpu: CPU = Default::default();
    cpu.registers.set(A, Value::EightBit(0x20));
    cpu.registers.set(B, Value::EightBit(0x40));
    let instruction = Instruction::Cp {
        what: cpu.registers.get(B),
        cycles: 1,
        length: InstructionLength::One,
    };
    cpu.execute(instruction);
    assert_eq!(cpu.registers.get(A), Value::EightBit(0x20));
    assert!(cpu.registers.f.is_set(C));

    // -- // -- // -- //
    let mut cpu: CPU = Default::default();
    cpu.registers.set(A, Value::EightBit(0x30));
    cpu.registers.set(B, Value::EightBit(0x30));
    let instruction = Instruction::Cp {
        what: cpu.registers.get(B),
        cycles: 1,
        length: InstructionLength::One,
    };
    cpu.execute(instruction);
    assert_eq!(cpu.registers.get(A), Value::EightBit(0x30));
    assert!(cpu.registers.f.is_set(Z))
}

#[test]
fn test_ret() {
    let mut cpu: CPU = Default::default();
    cpu.registers.set(PC, Value::SixteenBit(0x3535));
    cpu.registers.set(SP, Value::SixteenBit(0x2000));
    cpu.registers.f.set(Z);

    cpu.write(Value::SixteenBit(0x2000), Value::EightBit(0xB5));
    cpu.write(Value::SixteenBit(0x2001), Value::EightBit(0x18));

    let instruction = Instruction::Ret(FlagOn(Z));
    cpu.execute(instruction);

    assert_eq!(cpu.registers.get(SP), Value::SixteenBit(0x2002));
    assert_eq!(cpu.registers.get(PC), Value::SixteenBit(0x18B5));
    assert!(cpu.registers.f.is_set(Z));
    // -- // -- // -- //
    let mut cpu: CPU = Default::default();
    cpu.registers.set(PC, Value::SixteenBit(0xCAFD));
    cpu.registers.set(SP, Value::SixteenBit(0xFACE));
    cpu.registers.f.set(Z);

    cpu.write(Value::SixteenBit(0xFACE), Value::EightBit(0xAD));
    cpu.write(Value::SixteenBit(0xFACF), Value::EightBit(0xBE));

    let instruction = Instruction::Ret(FlagOn(C));
    cpu.execute(instruction);

    assert_eq!(cpu.registers.get(SP), Value::SixteenBit(0xFACE));
    assert_eq!(cpu.registers.get(PC), Value::SixteenBit(0xCAFE));
    assert!(cpu.registers.f.is_set(Z));

    cpu.registers.f.set(C);
    let instruction = Instruction::Ret(FlagOn(C));
    cpu.execute(instruction);

    assert_eq!(cpu.registers.get(SP), Value::SixteenBit(0xFAD0));
    assert_eq!(cpu.registers.get(PC), Value::SixteenBit(0xBEAD));
    assert!(cpu.registers.f.is_set(Z));
    assert!(cpu.registers.f.is_set(C));
}

#[test]
fn test_reti() {
    let mut cpu: CPU = Default::default();
    cpu.registers.set(PC, Value::SixteenBit(0x3535));
    cpu.registers.set(SP, Value::SixteenBit(0x2000));
    assert!(!cpu.ime());

    cpu.write(Value::SixteenBit(0x2000), Value::EightBit(0xB5));
    cpu.write(Value::SixteenBit(0x2001), Value::EightBit(0x18));

    let instruction = Instruction::Reti;
    cpu.execute(instruction);

    assert_eq!(cpu.registers.get(SP), Value::SixteenBit(0x2002));
    assert_eq!(cpu.registers.get(PC), Value::SixteenBit(0x18B5));
    assert!(cpu.ime());
}

#[test]
fn test_pop() {
    let mut cpu: CPU = Default::default();
    cpu.registers.set(SP, Value::SixteenBit(0x1000));

    cpu.write(Value::SixteenBit(0x1000), Value::EightBit(0x55));
    cpu.write(Value::SixteenBit(0x1001), Value::EightBit(0x33));

    let instruction = Instruction::Pop(BC);
    cpu.execute(instruction);

    assert_eq!(cpu.registers.get(SP), Value::SixteenBit(0x1002));
    assert_eq!(cpu.registers.get(BC), Value::SixteenBit(0x3355));
}

#[test]
fn test_push() {
    let mut cpu: CPU = Default::default();
    cpu.registers.set(AF, Value::SixteenBit(0x2233));
    cpu.registers.set(SP, Value::SixteenBit(0x1007));

    let instruction = Instruction::Push(AF);
    cpu.execute(instruction);

    assert_eq!(
        cpu.read(Value::SixteenBit(0x1006), false),
        Value::EightBit(0x22)
    );
    assert_eq!(
        cpu.read(Value::SixteenBit(0x1005), false),
        Value::EightBit(0x33)
    );
    assert_eq!(cpu.registers.get(SP), Value::SixteenBit(0x1005));
}

#[test]
fn test_call() {
    let mut cpu: CPU = Default::default();
    cpu.registers.f.set(C);
    cpu.registers.set(PC, Value::SixteenBit(0x1A47));
    cpu.registers.set(SP, Value::SixteenBit(0x3002));

    cpu.write(Value::SixteenBit(0x1A47), Value::EightBit(0xD4));
    cpu.write(Value::SixteenBit(0x1A48), Value::EightBit(0x35));
    cpu.write(Value::SixteenBit(0x1A49), Value::EightBit(0x21));

    let instruction = Instruction::Call(FlagOn(C));
    cpu.execute(instruction);

    assert_eq!(
        cpu.read(Value::SixteenBit(0x3000), false),
        Value::EightBit(0x4A)
    );
    assert_eq!(cpu.registers.get(SP), Value::SixteenBit(0x3000));
    assert_eq!(cpu.registers.get(PC), Value::SixteenBit(0x2135));
}

#[test]
fn test_rst() {
    let mut cpu: CPU = Default::default();
    cpu.registers.set(PC, Value::SixteenBit(0x15B3));

    let instruction = Instruction::Rst(Three);
    cpu.execute(instruction);

    assert_eq!(cpu.registers.get(PC), Value::SixteenBit(0x0018));

    let instruction = Instruction::Rst(Two);
    cpu.execute(instruction);

    assert_eq!(cpu.registers.get(PC), Value::SixteenBit(0x0010));
}

#[test]
fn test_ei() {
    let mut cpu: CPU = Default::default();
    let instruction = Instruction::Ei;
    cpu.execute(instruction);
    assert!(cpu.ime());
}

#[test]
fn test_di() {
    let mut cpu: CPU = Default::default();
    let instruction = Instruction::Di;
    cpu.execute(instruction);
    assert!(!cpu.ime());
}
