#[allow(unused)]
use std::{
    collections::HashMap,
    env, fs,
    thread::sleep,
    time::{Duration, Instant},
};

use minifb::{Key, Window, WindowOptions};

mod bus;
mod cartridge;
mod cpu;

use cartridge::CartridgeHeader;
#[allow(unused)]
use cpu::CPU;

const WIDTH: usize = 160;
const HEIGHT: usize = 144;
const SIZE: usize = WIDTH * HEIGHT;

fn main() {
    let mut buffer = vec![0u32; SIZE];

    // I WILL KEEP SOME CODE FOR LATER USE

    // let args: Vec<String> = env::args().collect();

    // let rom_name = args.get(2).expect("Rom name not provided!");
    // let microsseconds = args
    //     .get(1)
    //     .expect("Delay not provided!")
    //     .parse::<u64>()
    //     .expect("Must provide a number");

    let rom = fs::read(format!("./roms/Tetris_B.gb")).expect("File not exists!");
    let mut cartridge = CartridgeHeader::new(rom);
    cartridge.display();

    let mut cpu = CPU::new(cartridge);
    cpu.init();

    // let kmap: HashMap<Key, u8> = HashMap::from([
    //     (Key::Key1, 0x1),
    //     (Key::Key2, 0x2),
    //     (Key::Key3, 0x3),
    //     (Key::Q, 0x4),
    //     (Key::W, 0x5),
    //     (Key::E, 0x6),
    //     (Key::A, 0x7),
    //     (Key::S, 0x8),
    //     (Key::D, 0x9),
    //     (Key::Z, 0xA),
    //     (Key::X, 0x0),
    //     (Key::C, 0xB),
    //     (Key::Key4, 0xC),
    //     (Key::R, 0xD),
    //     (Key::F, 0xE),
    //     (Key::V, 0xF),
    // ]);

    let mut window = Window::new(
        "wx_gb",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: true,
            scale: minifb::Scale::X4,
            scale_mode: minifb::ScaleMode::Stretch,
            ..Default::default()
        },
    )
    .expect("Unable to create window!");

    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // let instant = Instant::now();

        // Code for later use
        // for k in window.get_keys_pressed(minifb::KeyRepeat::Yes) {
        //     if kmap.contains_key(&k) {
        //         cpu.keypad[kmap[&k] as usize] = 0x1
        //     }
        // }

        // buffer.copy_from_slice(&cpu.video);
        buffer.fill(u32::MAX);

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
