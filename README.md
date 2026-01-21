# wx_gb

A **Game Boy emulator** written in Rust.

> ⚠️ This project is purely educational.  
> The main goal is **not** to write clean, fast, or idiomatic Rust code, but to **learn how emulation works** and understand the Game Boy hardware.

---

## Current Notice

Currently, I'm learning how the Game Boy works. At first, I thought I was ready to write the emulator. Instead, after watching some videos, I decided to start slowly.

## About

This emulator was built mainly for learning purposes.

Resources used while developing it:

- [The Ultimate Game Boy Talk](https://media.ccc.de/v/33c3-8029-the_ultimate_game_boy_talk)
- [Pan Docs — Game Boy Technical Reference](https://gbdev.io/pandocs/)
- [Game Boy: Complete Technical Reference (Gekkio)](https://gekkio.fi/files/gb-docs/gbctr.pdf)
- [DMG-01: How to Emulate a Game Boy](https://rylev.github.io/DMG-01/public/book/)
- [Writing a Game Boy Emulator (Cinoop)](https://cturt.github.io/cinoop.html)
- [GameBoy implementation of mGBA](https://github.com/mgba-emu/mgba/tree/master/src/gb)

For testing, I used:

- Game Boy ROMs (e.g. Tetris)
- [Game Boy test ROM collection (blargg tests)](https://github.com/retrio/gb-test-roms)

I tried to write **most of the code myself**, using **minimal AI assistance**, only to clarify specific doubts during the learning process.

---

## Features

### Work in Progress
- [x] Cartridge
- [ ] Address Bus
- [ ] Memory map
- [ ] CPU (SM83)
- [ ] Timer
- [ ] PPU
- [ ] Input
- [ ] Audio
---

## Notes

This project is **not perfect** by design:

- No strong focus on optimization, architecture, or Rust best practices
- Bugs and incorrect behavior are expected
- The goal is learning, not production-quality code

Pull requests are welcome, especially for:
- Bug fixes
- Improving accuracy or ROM compatibility
- Correcting mistakes in the implementation
- Suggesting improvements or refactors

---

## Contributing

If you want to contribute:

1. Fork the repository
2. Create a branch for your changes
3. Submit a pull request explaining what you changed and why

Feedback is always appreciated.

---

## License

MIT License.  
Feel free to study, modify, and use this project as a reference.
