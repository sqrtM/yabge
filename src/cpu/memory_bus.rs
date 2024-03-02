#[derive(Debug)]
pub struct MemoryBus {
    rom_bank_00: [u8; 0x4000],         // 16 KiB ROM bank 00
    rom_bank_01: [u8; 0x4000],         // 16 KiB ROM Bank 01–NN
    vram: [u8; 0x2000],                // 8 KiB Video RAM (VRAM)
    external_ram: [u8; 0x2000],        // 8 KiB External RAM
    work_ram: [u8; 0x1000],            // 4 KiB Work RAM (WRAM)
    work_ram_switchable: [u8; 0x1000], // 4 KiB Work RAM (WRAM), switchable bank 1–7
    echo_ram: [u8; 0x1E00],            // Echo RAM (mirror of C000–DDFF)
    oam: [u8; 0xA0],                   // Object attribute memory (OAM)
    io_registers: [u8; 0x80],          // I/O Registers
    hram: [u8; 0x7F],                  // High RAM (HRAM)
    interrupt_enable: bool,            // Interrupt Enable register (IE)
}

impl Default for MemoryBus {
    fn default() -> Self {
        MemoryBus {
            rom_bank_00: [0; 0x4000],
            rom_bank_01: [0; 0x4000],
            vram: [0; 0x2000],
            external_ram: [0; 0x2000],
            work_ram: [0; 0x1000],
            work_ram_switchable: [0; 0x1000],
            echo_ram: [0; 0x1E00],
            oam: [0; 0xA0],
            io_registers: [0; 0x80],
            hram: [0; 0x7F],
            interrupt_enable: false,
        }
    }
}

impl MemoryBus {
    pub(crate) fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => self.rom_bank_00[address as usize],
            0x4000..=0x7FFF => self.rom_bank_01[address as usize - 0x4000],
            0x8000..=0x9FFF => self.vram[address as usize - 0x8000],
            0xA000..=0xBFFF => self.external_ram[address as usize - 0xA000],
            0xC000..=0xCFFF => self.work_ram[address as usize - 0xC000],
            0xD000..=0xDFFF => self.work_ram_switchable[address as usize - 0xD000],
            0xE000..=0xFDFF => self.echo_ram[address as usize - 0xE000],
            0xFE00..=0xFE9F => self.oam[address as usize - 0xFE00],
            0xFF00..=0xFF7F => self.io_registers[address as usize - 0xFF00],
            0xFF80..=0xFFFE => self.hram[address as usize - 0xFF80],
            0xFFFF => {
                if self.interrupt_enable {
                    1
                } else {
                    0
                }
            }
            _ => panic!("Invalid memory address: 0x{:04X}", address),
        }
    }

    pub(crate) fn write(&mut self, address: u16, data: u8) {
        match address {
            0x0000..=0x3FFF => self.rom_bank_00[address as usize] = data,
            0x4000..=0x7FFF => self.rom_bank_01[address as usize - 0x4000] = data,
            0x8000..=0x9FFF => self.vram[address as usize - 0x8000] = data,
            0xA000..=0xBFFF => self.external_ram[address as usize - 0xA000] = data,
            0xC000..=0xCFFF => self.work_ram[address as usize - 0xC000] = data,
            0xD000..=0xDFFF => self.work_ram_switchable[address as usize - 0xD000] = data,
            0xE000..=0xFDFF => self.echo_ram[address as usize - 0xE000] = data,
            0xFE00..=0xFE9F => self.oam[address as usize - 0xFE00] = data,
            0xFF00..=0xFF7F => self.io_registers[address as usize - 0xFF00] = data,
            0xFF80..=0xFFFE => self.hram[address as usize - 0xFF80] = data,
            0xFFFF => self.interrupt_enable = if data != 0 { true } else { false },
            _ => panic!("Invalid memory address: 0x{:04X}", address),
        }
    }
}
