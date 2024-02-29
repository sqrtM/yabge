pub enum Opcode {
    ADD,
    ADC,
    SUB,
    SBC,
    AND,
    CP,
    DEC,
    INC,
    OR,
    XOR,
    BIT,
    RES,
    SET,
    SWAP,
    RL,
    RLA,
    RLC,
    RR,
    RRA,
    RRC,
    RRCA,
    SLA,
    SRA,
    SRL,
    LD,
    LDH,
    CALL,
    JP,
    JR,
    RET,
    RETI,
    RST,
    POP,
    PUSH,
    CCF,
    CPL,
    DAA,
    DI,
    EI,
    HALT,
    NOP,
    SCF,
    STOP,
}

impl Opcode {
    pub fn execute(&self) {
        match self {
            Opcode::ADD => {}
            Opcode::ADC => {}
            Opcode::SUB => {}
            Opcode::SBC => {}
            Opcode::AND => {}
            Opcode::CP => {}
            Opcode::DEC => {}
            Opcode::INC => {}
            Opcode::OR => {}
            Opcode::XOR => {}
            Opcode::BIT => {}
            Opcode::RES => {}
            Opcode::SET => {}
            Opcode::SWAP => {}
            Opcode::RL => {}
            Opcode::RLA => {}
            Opcode::RLC => {}
            Opcode::RR => {}
            Opcode::RRA => {}
            Opcode::RRC => {}
            Opcode::RRCA => {}
            Opcode::SLA => {}
            Opcode::SRA => {}
            Opcode::SRL => {}
            Opcode::LD => {}
            Opcode::LDH => {}
            Opcode::CALL => {}
            Opcode::JP => {}
            Opcode::JR => {}
            Opcode::RET => {}
            Opcode::RETI => {}
            Opcode::RST => {}
            Opcode::POP => {}
            Opcode::PUSH => {}
            Opcode::CCF => {}
            Opcode::CPL => {}
            Opcode::DAA => {}
            Opcode::DI => {}
            Opcode::EI => {}
            Opcode::HALT => {}
            Opcode::NOP => {}
            Opcode::SCF => {}
            Opcode::STOP => {}
        }
    }
}

fn check_overflows(result: u8) -> (bool, bool) {
    let overflow_on_bit_3 = (result & (1 << 3)) != 0;
    let overflow_on_bit_7 = (result & (1 << 7)) != 0;
    (overflow_on_bit_3, overflow_on_bit_7)
}
