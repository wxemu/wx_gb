#[allow(unused)]
pub struct BUS {
    rom_a: [u8; 0x4000],
    rom_b: [u8; 0x4000],
    vram: [u8; 0x2000],
    eram: [u8; 0x2000],
    wram: [u8; 0x2000],
    mram: [u8; 0x2000],
    oam: [u8; 0xA0],
    io: [u8; 0x80],
    hram: [u8; 0x80],
    ie: u8,
}

#[allow(unused)]
impl BUS {
    pub fn new() -> Self {
        BUS {
            rom_a: [0x0; 0x4000],
            rom_b: [0x0; 0x4000],
            vram: [0x0; 0x2000],
            eram: [0x0; 0x2000],
            wram: [0x0; 0x2000],
            mram: [0x0; 0x2000],
            oam: [0x0; 0xA0],
            io: [0x0; 0x80],
            hram: [0x0; 0x80],
            ie: 0,
        }
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        // Not perfect, minimal implementation
        // https://gbdev.io/pandocs/Memory_Map.html
        match addr {
            0x0000..=0x3FFF => self.rom_a[addr as usize], // ROM 00
            0x4000..=0x7FFF => self.rom_b[addr as usize - 0x4000], // ROM 01, ROM NN (MBC)
            0x8000..=0x9FFF => self.vram[addr as usize - 0x8000], // VRAM
            0xA000..=0xBFFF => self.eram[addr as usize - 0xA000], // ERAM (Cartrigde RAM)
            0xC000..=0xDFFF => self.wram[addr as usize - 0xC000], // WRAM
            0xE000..=0xFDFF => self.mram[addr as usize - 0xE000], // Echo RAM (Mirror of WRAM C000-DDFF)
            0xFE00..=0xFE9F => self.oam[addr as usize - 0xFE00],  // OAM Object Attribute Memory
            0xFEA0..=0xFEFF => 0xFF,                              // Not usable like echo ram
            0xFF00..=0xFF7F => self.io[addr as usize - 0xFF00],   // I/O Registers
            0xFF80..=0xFFFE => self.hram[addr as usize - 0xFF80], // High RAM (HRAM)
            0xFFFF => self.ie,                                    // IE (Interrupt Enable Register)
        }
    }
}
