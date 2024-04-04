use std::env;
use std::fs::File;
use std::io::Read;

use crate::cpu::CPU;
use crate::cpu::registers::Register::PC;
use crate::cpu::value::Value;

mod cpu;

fn main() {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the user provided a ROM file path
    if args.len() != 2 {
        println!("Usage: {} <rom_file>", args[0]);
        return;
    }

    // Extract the ROM file path from the command-line arguments
    let rom_file_path = &args[1];

    // Open the ROM file
    let mut rom_file = match File::open(rom_file_path) {
        Ok(file) => file,
        Err(_) => {
            println!("Failed to open ROM file.");
            return;
        }
    };

    // Read ROM file contents
    let mut rom_data = Vec::new();
    if rom_file.read_to_end(&mut rom_data).is_err() {
        println!("Failed to read ROM file.");
    }
    


    let mut cpu = CPU::default();

    for (index, data) in rom_data.iter().enumerate() {
        cpu.write(Value::SixteenBit(index as u16), Value::EightBit(*data))
    }
    
    cpu.registers.set(PC, Value::SixteenBit(0x100));
    let mut current_code = Value::EightBit(rom_data[0x100usize]);

    while current_code != Value::EightBit(0xFD) {
        if let Value::EightBit(code) = current_code {
            let inst = cpu.lookup(code);
            println!("{:?}", inst);
            cpu.execute(inst);
            current_code = cpu.read(cpu.registers.get(PC), false);
        } else {
            println!("hmmm");
            break
        }
    }
    
}
