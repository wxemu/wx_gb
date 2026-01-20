use crate::bus::BUS;

#[allow(unused)]
const CARTRIGDE_ADDR: u16 = 0x0100;

#[allow(unused)]
pub struct CPU {
    pc: u16,
    sp: u16,
    bus: BUS,
}

#[allow(unused)]
impl CPU {
    pub fn new() -> Self {
        CPU {
            sp: 0x0,
            pc: 0x0,
            bus: BUS::new(),
        }
    }

    pub fn init(&mut self) {
        // https://gbdev.io/pandocs/Power_Up_Sequence.html
        // EMULATE BOOT ROM BEHAVIOUR
        self.pc = CARTRIGDE_ADDR;
        self.sp = 0xFFFE;
    }

    pub fn load(&mut self, rom: Vec<u8>) {}
}
