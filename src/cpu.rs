use crate::{bus::BUS, cartridge::CartridgeHeader};

#[allow(unused)]
const CARTRIGDE_ADDR: u16 = 0x0100;

#[allow(unused)]
pub struct CPU {
    register: [u8; 8],
    pc: u16,
    sp: u16,
    bus: BUS,
    cartridge: CartridgeHeader,
}

#[allow(unused)]
impl CPU {
    pub fn new(cartridge: CartridgeHeader) -> Self {
        CPU {
            register: [0x0; 8],
            sp: 0x0,
            pc: 0x0,
            bus: BUS::new(),
            cartridge: cartridge,
        }
    }

    pub fn init(&mut self) {
        // https://gbdev.io/pandocs/Power_Up_Sequence.html
        // EMULATE BOOT ROM BEHAVIOUR
        self.pc = CARTRIGDE_ADDR;
        self.sp = 0xFFFE;

        // LOAD CARTRIDGE
        for b in self.cartridge.data.iter().enumerate() {
            if b.0 > 0x7FFF {
                break;
            }
            self.bus.write(b.0 as u16, *b.1);
        }
    }

    pub fn load(&mut self, rom: Vec<u8>) {}
}
