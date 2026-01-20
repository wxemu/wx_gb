use std::num::Wrapping;

#[allow(unused)]
pub struct CartridgeHeader {
    entry: [u8; 4],
    logo: [u8; 48],
    title: [u8; 16],
    gbc_flags: u8,
    new_license: [u8; 2],
    sgb_flag: u8,
    rom_type: u8,
    cart_size: u8,
    ram_type: u8,
    destination: u8,
    old_license: u8,
    mask_rom_version_number: u8,
    header_checksum: u8,
    global_checksum: u16,
    data: Vec<u8>,
    rom_0: [u8; 0x8000],
}

#[allow(unused)]
impl CartridgeHeader {
    pub fn new(rom: Vec<u8>) -> Self {
        let mut header = Self {
            entry: [0x0; 4],
            logo: [0x0; 48],
            title: [0x0; 16],
            gbc_flags: 0,
            new_license: [0x0; 2],
            sgb_flag: 0,
            rom_type: 0,
            cart_size: 0,
            ram_type: 0,
            destination: 0,
            old_license: 0,
            mask_rom_version_number: 0,
            header_checksum: 0,
            global_checksum: 0,
            data: rom,
            rom_0: [0x0u8; 0x8000],
        };

        // Loading first 16KiB on rom_0
        for b in header.data[0x0..0x8000].iter().enumerate() {
            header.rom_0[b.0] = *b.1;
        }

        // Entry instructions
        for b in header.data[0x100..=0x103].iter().enumerate() {
            header.entry[b.0] = *b.1;
        }

        // Nintendo logo
        for b in header.data[0x104..=0x133].iter().enumerate() {
            header.logo[b.0] = *b.1;
        }

        // Title
        for b in header.data[0x134..=0x143].iter().enumerate() {
            header.title[b.0] = *b.1;
        }

        header.gbc_flags = header.data[0x143];

        // New license
        for b in header.data[0x144..=0x145].iter().enumerate() {
            header.new_license[b.0] = *b.1;
        }

        header.sgb_flag = header.data[0x146];
        header.rom_type = header.data[0x147];
        header.cart_size = header.data[0x148];
        header.ram_type = header.data[0x149];
        header.destination = header.data[0x14A];
        header.old_license = header.data[0x14B];
        header.mask_rom_version_number = header.data[0x14C];
        header.header_checksum = header.data[0x14D];
        header.global_checksum = ((header.data[0x14F] as u16) << 8) | (header.data[0x14E] as u16);

        return header;
    }

    pub fn display(&mut self) {
        let title = self.get_title();
        let checksum = self.header_checksum;
        let checksum_c = self.get_checksum();
        let dest = self.get_destination();
        let lis = self.get_license();
        let t = self.get_cartridge_type();
        println!(
            "TITLE: {title}\nHEADER_CHECKSUM: {checksum}\nCALCULATED_HEADER_CHECKSUM: {checksum_c}\nDESTINATION: {dest}\nLISCENSE: {lis}\nTYPE: {t}"
        );
    }

    pub fn get_title(&self) -> &str {
        str::from_utf8(&self.title).expect("Error when encoding title")
    }

    pub fn get_checksum(&self) -> u8 {
        let checksum_data = &self.rom_0[0x0134..=0x014C];
        let mut checksum: Wrapping<u8> = Wrapping::<u8>(0);
        let x = Wrapping::<u8>(1);

        for d in checksum_data {
            checksum = checksum - Wrapping::<u8>(*d) - x;
        }

        return checksum.0;
    }

    fn get_destination(&self) -> &'static str {
        match self.destination {
            0 => "Japan",
            1 => "World",
            _ => "Unknown",
        }
    }

    fn get_ram_size(&self) -> usize {
        match self.ram_type {
            0 => 0,
            1 => 0,
            2 => 8_000,   // 1 bank of 8KiB
            3 => 32_000,  // 4 bank of 8KiB
            4 => 128_000, // 16 bank of 8KiB
            5 => 64_000,  // 8 bank of 8KiB
            _ => 0,
        }
    }

    fn get_rom_size(&self) -> (usize, usize) {
        let mut c_size = (1 << self.cart_size) * 32_000;
        let mut b_size = (1 << self.cart_size) * 2;
        return (c_size as usize, b_size as usize);
    }

    fn get_cartridge_type(&self) -> &'static str {
        match self.rom_type {
            0x00 => "ROM ONLY",
            0x01 => "MBC1",
            0x02 => "MBC1+RAM",
            0x03 => "MBC1+RAM+BATTERY",
            0x05 => "MBC2",
            0x06 => "MBC2+BATTERY",
            0x08 => "ROM+RAM 11",
            0x09 => "ROM+RAM+BATTERY 11",
            0x0B => "MMM01",
            0x0C => "MMM01+RAM",
            0x0D => "MMM01+RAM+BATTERY",
            0x0F => "MBC3+TIMER+BATTERY",
            0x10 => "MBC3+TIMER+RAM+BATTERY 12",
            0x11 => "MBC3",
            0x12 => "MBC3+RAM 12",
            0x13 => "MBC3+RAM+BATTERY 12",
            0x19 => "MBC5",
            0x1A => "MBC5+RAM",
            0x1B => "MBC5+RAM+BATTERY",
            0x1C => "MBC5+RUMBLE",
            0x1D => "MBC5+RUMBLE+RAM",
            0x1E => "MBC5+RUMBLE+RAM+BATTERY",
            0x20 => "MBC6",
            0x22 => "MBC7+SENSOR+RUMBLE+RAM+BATTERY",
            0xFC => "POCKET CAMERA",
            0xFD => "BANDAI TAMA5",
            0xFE => "HuC3",
            0xFF => "HuC1+RAM+BATTERY",
            _ => "Unknown",
        }
    }

    fn get_license(&self) -> &'static str {
        if self.old_license == 0x33 {
            let s = str::from_utf8(&self.new_license).expect("Title encoding error!");
            match s {
                "00" => "None",
                "01" => "Nintendo Research & Development 1",
                "08" => "Capcom",
                "13" => "EA (Electronic Arts)",
                "18" => "Hudson Soft",
                "19" => "B-AI",
                "20" => "KSS",
                "22" => "Planning Office WADA",
                "24" => "PCM Complete",
                "25" => "San-X",
                "28" => "Kemco",
                "29" => "SETA Corporation",
                "30" => "Viacom",
                "31" => "Nintendo",
                "32" => "Bandai",
                "33" => "Ocean Software/Acclaim Entertainment",
                "34" => "Konami",
                "35" => "HectorSoft",
                "37" => "Taito",
                "38" => "Hudson Soft",
                "39" => "Banpresto",
                "41" => "Ubi Soft1",
                "42" => "Atlus",
                "44" => "Malibu Interactive",
                "46" => "Angel",
                "47" => "Bullet-Proof Software2",
                "49" => "Irem",
                "50" => "Absolute",
                "51" => "Acclaim Entertainment",
                "52" => "Activision",
                "53" => "Sammy USA Corporation",
                "54" => "Konami",
                "55" => "Hi Tech Expressions",
                "56" => "LJN",
                "57" => "Matchbox",
                "58" => "Mattel",
                "59" => "Milton Bradley Company",
                "60" => "Titus Interactive",
                "61" => "Virgin Games Ltd.3",
                "64" => "Lucasfilm Games4",
                "67" => "Ocean Software",
                "69" => "EA (Electronic Arts)",
                "70" => "Infogrames5",
                "71" => "Interplay Entertainment",
                "72" => "Broderbund",
                "73" => "Sculptured Software6",
                "75" => "The Sales Curve Limited7",
                "78" => "THQ",
                "79" => "Accolade8",
                "80" => "Misawa Entertainment",
                "83" => "LOZC G.",
                "86" => "Tokuma Shoten",
                "87" => "Tsukuda Original",
                "91" => "Chunsoft Co.9",
                "92" => "Video System",
                "93" => "Ocean Software/Acclaim Entertainment",
                "95" => "Varie",
                "96" => "Yonezawa10/S’Pal",
                "97" => "Kaneko",
                "99" => "Pack-In-Video",
                "9H" => "Bottom Up",
                "A4" => "Konami (Yu-Gi-Oh!)",
                "BL" => "MTO",
                "DK" => "Kodansha",
                _ => "Unknown",
            }
        } else {
            match self.old_license {
                0x00 => "None",
                0x01 => "Nintendo",
                0x08 => "Capcom",
                0x09 => "HOT-B",
                0x0A => "Jaleco",
                0x0B => "Coconuts Japan",
                0x0C => "Elite Systems",
                0x13 => "EA (Electronic Arts)",
                0x18 => "Hudson Soft",
                0x19 => "ITC Entertainment",
                0x1A => "Yanoman",
                0x1D => "Japan Clary",
                0x1F => "Virgin Games Ltd.3",
                0x24 => "PCM Complete",
                0x25 => "San-X",
                0x28 => "Kemco",
                0x29 => "SETA Corporation",
                0x30 => "Infogrames5",
                0x31 => "Nintendo",
                0x32 => "Bandai",
                0x33 => "New Liscense",
                0x34 => "Konami",
                0x35 => "HectorSoft",
                0x38 => "Capcom",
                0x39 => "Banpresto",
                0x3C => "Entertainment Interactive (stub)",
                0x3E => "Gremlin",
                0x41 => "Ubi Soft1",
                0x42 => "Atlus",
                0x44 => "Malibu Interactive",
                0x46 => "Angel",
                0x47 => "Spectrum HoloByte",
                0x49 => "Irem",
                0x4A => "Virgin Games Ltd.3",
                0x4D => "Malibu Interactive",
                0x4F => "U.S. Gold",
                0x50 => "Absolute",
                0x51 => "Acclaim Entertainment",
                0x52 => "Activision",
                0x53 => "Sammy USA Corporation",
                0x54 => "GameTek",
                0x55 => "Park Place15",
                0x56 => "LJN",
                0x57 => "Matchbox",
                0x59 => "Milton Bradley Company",
                0x5A => "Mindscape",
                0x5B => "Romstar",
                0x5C => "Naxat Soft16",
                0x5D => "Tradewest",
                0x60 => "Titus Interactive",
                0x61 => "Virgin Games Ltd.3",
                0x67 => "Ocean Software",
                0x69 => "EA (Electronic Arts)",
                0x6E => "Elite Systems",
                0x6F => "Electro Brain",
                0x70 => "Infogrames5",
                0x71 => "Interplay Entertainment",
                0x72 => "Broderbund",
                0x73 => "Sculptured Software6",
                0x75 => "The Sales Curve Limited7",
                0x78 => "THQ",
                0x79 => "Accolade8",
                0x7A => "Triffix Entertainment",
                0x7C => "MicroProse",
                0x7F => "Kemco",
                0x80 => "Misawa Entertainment",
                0x83 => "LOZC G.",
                0x86 => "Tokuma Shoten",
                0x8B => "Bullet-Proof Software2",
                0x8C => "Vic Tokai Corp.17",
                0x8E => "Ape Inc.18",
                0x8F => "I’Max19",
                0x91 => "Chunsoft Co.9",
                0x92 => "Video System",
                0x93 => "Tsubaraya Productions",
                0x95 => "Varie",
                0x96 => "Yonezawa10/S’Pal",
                0x97 => "Kemco",
                0x99 => "Arc",
                0x9A => "Nihon Bussan",
                0x9B => "Tecmo",
                0x9C => "Imagineer",
                0x9D => "Banpresto",
                0x9F => "Nova",
                0xA1 => "Hori Electric",
                0xA2 => "Bandai",
                0xA4 => "Konami",
                0xA6 => "Kawada",
                0xA7 => "Takara",
                0xA9 => "Technos Japan",
                0xAA => "Broderbund",
                0xAC => "Toei Animation",
                0xAD => "Toho",
                0xAF => "Namco",
                0xB0 => "Acclaim Entertainment",
                0xB1 => "ASCII Corporation or Nexsoft",
                0xB2 => "Bandai",
                0xB4 => "Square Enix",
                0xB6 => "HAL Laboratory",
                0xB7 => "SNK",
                0xB9 => "Pony Canyon",
                0xBA => "Culture Brain",
                0xBB => "Sunsoft",
                0xBD => "Sony Imagesoft",
                0xBF => "Sammy Corporation",
                0xC0 => "Taito",
                0xC2 => "Kemco",
                0xC3 => "Square",
                0xC4 => "Tokuma Shoten",
                0xC5 => "Data East",
                0xC6 => "Tonkin House",
                0xC8 => "Koei",
                0xC9 => "UFL",
                0xCA => "Ultra Games",
                0xCB => "VAP, Inc.",
                0xCC => "Use Corporation",
                0xCD => "Meldac",
                0xCE => "Pony Canyon",
                0xCF => "Angel",
                0xD0 => "Taito",
                0xD1 => "SOFEL (Software Engineering Lab)",
                0xD2 => "Quest",
                0xD3 => "Sigma Enterprises",
                0xD4 => "ASK Kodansha Co.",
                0xD6 => "Naxat Soft16",
                0xD7 => "Copya System",
                0xD9 => "Banpresto",
                0xDA => "Tomy",
                0xDB => "LJN",
                0xDD => "Nippon Computer Systems",
                0xDE => "Human Ent.",
                0xDF => "Altron",
                0xE0 => "Jaleco",
                0xE1 => "Towa Chiki",
                0xE2 => "Yutaka # Needs more info",
                0xE3 => "Varie",
                0xE5 => "Epoch",
                0xE7 => "Athena",
                0xE8 => "Asmik Ace Entertainment",
                0xE9 => "Natsume",
                0xEA => "King Records",
                0xEB => "Atlus",
                0xEC => "Epic/Sony Records",
                0xEE => "IGS",
                0xF0 => "A Wave",
                0xF3 => "Extreme Entertainment",
                0xFF => "LJN",
                _ => "Unknown",
            }
        }
    }
}
